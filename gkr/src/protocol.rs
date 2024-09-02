use crate::{
	datastructure::{Circuit, GKRProof},
	utils::{perform_layer_one_prove_sumcheck, perform_layer_one_verify_sumcheck, w_mle},
};
use ark_ff::PrimeField;
use polynomial::composed::{
	interface::ComposedMultilinearInterface, multilinear::ComposedMultiLinearPolynomial,
};
use polynomial::multilinear::interface::MultiLinearPolynomialEvaluationFormTrait;
use sum_check::multi_composedsumcheck::{
	ComposedSumcheckProof, MultiComposedSumcheckProver, MultiComposedSumcheckVerifier,
};
use transcript::transcription::Transcript;
pub struct GKRProtocol;

impl GKRProtocol {
	pub fn prove<F: PrimeField>(circuit: &Circuit, input: &[F]) -> GKRProof<F> {
		let mut transcript = Transcript::new();
		let mut sumcheck_proofs: Vec<ComposedSumcheckProof<F>> = Vec::new();
		let mut w_i_b: Vec<F> = Vec::new();
		let mut w_i_c: Vec<F> = Vec::new();

		let circuit_evaluation = circuit.evaluate(input);
		let mut circuit_evaluation_layer_zero_pad = circuit_evaluation[0].clone();
		circuit_evaluation_layer_zero_pad.push(F::zero());

		let w_0_mle = w_mle(circuit_evaluation_layer_zero_pad);
		transcript.append(&w_0_mle.to_bytes());

		let n_r: Vec<F> = transcript.sample_n_as_field_element(w_0_mle.number_of_variables);
		let claim: F = w_0_mle.evaluation(&n_r);

		let (add_mle, mul_mle) = circuit.add_i_mul_ext::<F>(0);
		let w_1_mle = w_mle(circuit_evaluation[1].clone());

		// Run sumcheck on layer one
		let (layer_one_claim, alps, bta, layer_one_rand_b, layer_one_rand_c) =
			perform_layer_one_prove_sumcheck(
				&add_mle,
				&mul_mle,
				&w_1_mle,
				&n_r,
				&claim,
				&mut transcript,
				&mut sumcheck_proofs,
				&mut w_i_b,
				&mut w_i_c,
			);

		let mut claim = layer_one_claim;
		let mut alpha = alps;
		let mut beta = bta;
		let last_rand_b = layer_one_rand_b;
		let last_rand_c = layer_one_rand_c;
		// starting the GKR round reductions powered by sumcheck (layer 2 to n-1(excluding the input layer))
		for layer_index in 2..circuit_evaluation.len() {
			let (add_mle, mul_mle) = circuit.add_i_mul_ext::<F>(layer_index - 1);
			let number_of_round = last_rand_b.len();

			// add(r_b, b, c) ---> add(b, c)
			let add_rb_bc = add_mle.partial_evaluations(&last_rand_b, &vec![0; number_of_round]);
			// mul(r_b, b, c) ---> mul(b, c)
			let mul_rb_bc = mul_mle.partial_evaluations(&last_rand_b, &vec![0; number_of_round]);
			// add(r_c, b, c) ---> add(b, c)
			let add_rc_bc = add_mle.partial_evaluations(&last_rand_c, &vec![0; number_of_round]);
			// mul(r_c, b, c) ---> mul(b, c)
			let mul_rc_bc = mul_mle.partial_evaluations(&last_rand_c, &vec![0; number_of_round]);
			let w_i_mle = w_mle(circuit_evaluation[layer_index].clone());

			let wb = w_i_mle.clone();
			let wc = w_i_mle;
			// w_i(b) + w_i(c)
			let wb_add_wc = wb.add_distinct(&wc);
			// w_i(b) * w_i(c)

			let wb_mul_wc = wb.mul_distinct(&wc);
			// alpha * add(r_b, b, c) + beta * add(r_c, b, c)
			let add_alpha_beta = (add_rb_bc * alpha) + (add_rc_bc * beta);
			// alpha * mul(r_b, b, c) + beta * mul(r_c, b, c)
			let mul_alpha_beta = (mul_rb_bc * alpha) + (mul_rc_bc * beta);

			// alpha * add(r_b, b, c) + beta * add(r_c, b, c)(w_i(b) + w_i(c))
			let fbc_add = ComposedMultiLinearPolynomial::new(vec![add_alpha_beta, wb_add_wc]);

			// f(b, c) = alpha * add(r_b, b, c) + beta * add(r_c, b, c)(w_i(b) + w_i(c)) + alpha * mul(r_b, b, c) + beta * mul(r_c, b, c)(w_i(b) * w_i(c))
			let fbc_mul = ComposedMultiLinearPolynomial::new(vec![mul_alpha_beta, wb_mul_wc]);

			// this prover that the `claim` is the result of the evalution of the previous layer
			let (sumcheck_proof, challenges) =
				MultiComposedSumcheckProver::prove_partial(&vec![fbc_add, fbc_mul], &claim)
					.unwrap();

			transcript.append(&sumcheck_proof.to_bytes());
			sumcheck_proofs.push(sumcheck_proof);
			// split challenge between rand_b and rand_c
			let (rand_b, rand_c) = challenges.split_at(challenges.len() / 2);

			let eval_w_i_b = wb.evaluation(&rand_b.to_vec());
			let eval_w_i_c = wc.evaluation(&rand_c.to_vec());
			w_i_b.push(eval_w_i_b);
			w_i_c.push(eval_w_i_c);

			alpha = transcript.transform_challenge_to_field::<F>();
			beta = transcript.transform_challenge_to_field::<F>();

			claim = alpha * eval_w_i_b + beta * eval_w_i_c;
		}

		GKRProof { sumcheck_proofs, w_i_b, w_i_c, w_0_mle }
	}

	pub fn verify<F: PrimeField>(circuit: &Circuit, input: &[F], proof: &GKRProof<F>) -> bool {
		// check sumcheckproof length against w_i_b length
		if proof.sumcheck_proofs.len() != proof.w_i_b.len()
			|| proof.sumcheck_proofs.len() != proof.w_i_c.len()
		{
			return false;
		}

		let mut transcript = Transcript::new();
		transcript.append(&proof.w_0_mle.to_bytes());

		let n_r: Vec<F> =
			transcript.sample_n_as_field_element::<F>(proof.w_0_mle.number_of_variables);
		let mut claim = proof.w_0_mle.evaluation(&n_r.clone().as_slice());

		let mut last_rand_b: Vec<F> = vec![];
		let mut last_rand_c: Vec<F> = vec![];
		let mut alpha: F = F::zero();
		let mut beta: F = F::zero();

		//layer one verification logic
		let (add_mle, mul_mle) = circuit.add_i_mul_ext::<F>(0);
		let (status, layer_one_sum) = perform_layer_one_verify_sumcheck(
			&add_mle,
			&mul_mle,
			&proof.sumcheck_proofs[0],
			n_r,
			&claim,
			&mut transcript,
			&proof.w_i_b[0],
			&proof.w_i_c[0],
		);

		if !status {
			return false;
		}

		claim = layer_one_sum;

		for i in 1..proof.sumcheck_proofs.len() {
			if claim != proof.sumcheck_proofs[i].sum {
				return false;
			}

			transcript.append(&proof.sumcheck_proofs[i].to_bytes());

			let verify_subclaim =
				MultiComposedSumcheckVerifier::verify_partial(&proof.sumcheck_proofs[i]).unwrap();
			// split challenge between rand_b and rand_c
			// println!("challenge:={:?}", verify_subclaim.challenges);
			let (rand_b, rand_c) =
				verify_subclaim.challenges.split_at(&verify_subclaim.challenges.len() / 2);

			last_rand_b = rand_b.to_vec();
			last_rand_c = rand_c.to_vec();

			let w_b = proof.w_i_b[i];
			let w_c = proof.w_i_c[i];

			let alps = transcript.transform_challenge_to_field::<F>();
			let bta = transcript.transform_challenge_to_field::<F>();

			claim = alps * w_b + bta * w_c;

			alpha = alps;
			beta = bta;
		}
		// perform verification for the input layer
		let w_mle_input = w_mle(input.to_vec());

		let w_mle_input_b = w_mle_input.evaluation(&last_rand_b);
		let w_mle_input_c = w_mle_input.evaluation(&last_rand_c);

		let expected_claim = (alpha * w_mle_input_b) + (beta * w_mle_input_c);

		if expected_claim != claim {
			return false;
		}

		true
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::datastructure::{Circuit, CircuitLayer, Gate, GateType};
	use ark_ff::MontConfig;
	use ark_ff::{Fp64, MontBackend};

	#[derive(MontConfig)]
	#[modulus = "17"]
	#[generator = "3"]
	struct FqConfig;
	type Fq = Fp64<MontBackend<FqConfig, 1>>;

	#[test]
	fn test_gkr_protocol() {
		let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 1])]);
		let layer_1 = CircuitLayer::new(vec![
			Gate::new(GateType::Add, [0, 1]),
			Gate::new(GateType::Mul, [2, 3]),
		]);
		let circuit = Circuit::new(vec![layer_0, layer_1]);
		let input = vec![Fq::from(2u32), Fq::from(3u32), Fq::from(4u32), Fq::from(5u32)];

		let proof = GKRProtocol::prove(&circuit, &input);
		let verify = GKRProtocol::verify(&circuit, &input, &proof);

		assert!(verify);
	}
}
