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
	pub fn prove<'a, F: PrimeField>(circuit: &'a Circuit, input: &'a Vec<F>) -> GKRProof<F> {
		let mut transcript = Transcript::new();
		let mut sumcheck_proofs: Vec<ComposedSumcheckProof<F>> = Vec::new();
		let mut w_i_b: Vec<F> = Vec::new();
		let mut w_i_c: Vec<F> = Vec::new();

		let circuit_evaluation = circuit.evaluate(input);
		let mut circuit_evaluation_layer_zero_pad = circuit_evaluation[0].clone();
		circuit_evaluation_layer_zero_pad.push(F::zero());

		let w_0_mle = w_mle(circuit_evaluation_layer_zero_pad.to_vec());
		transcript.append(&w_0_mle.to_bytes());

		let n_r: Vec<F> = transcript.sample_n_as_field_element(w_0_mle.number_of_variables);
		let mut claimed_sum: F = w_0_mle.evaluation(&n_r);

		let (add_mle_1, mult_mle_1) = circuit.add_i_mul_ext::<F>(0);
		let w_1_mle = w_mle(circuit_evaluation[1].to_vec());

		let (claimed, alph, bta, rb, rc) = perform_layer_one_prove_sumcheck(
			&add_mle_1,
			&mult_mle_1,
			&w_1_mle,
			&n_r,
			&claimed_sum,
			&mut transcript,
			&mut sumcheck_proofs,
			&mut w_i_b,
			&mut w_i_c,
		);

		claimed_sum = claimed;

		let mut alpha: F = alph;
		let mut beta: F = bta;
		let mut r_b: Vec<F> = rb;
		let mut r_c: Vec<F> = rc;

		for layer_index in 2..circuit_evaluation.len() {
			let (add_mle, mul_mle) = circuit.add_i_mul_ext::<F>(layer_index - 1);

			let add_rb_bc = add_mle.partial_evaluations(&r_b, &vec![0; r_b.len()]);
			let mul_rb_bc = mul_mle.partial_evaluations(&r_b, &vec![0; r_b.len()]);

			let add_rc_bc = add_mle.partial_evaluations(&r_c, &vec![0; r_b.len()]);
			let mul_rc_bc = mul_mle.partial_evaluations(&r_c, &vec![0; r_b.len()]);
			let w_i_mle = w_mle(circuit_evaluation[layer_index].to_vec());

			let wb = w_i_mle.clone();
			let wc = w_i_mle;

			let wb_add_wc = wb.add_distinct(&wc);
			let wb_mul_wc = wb.mul_distinct(&wc);

			// alpha * add(r_b, b, c) + beta * add(r_c, b, c)
			let add_alpha_beta = (add_rb_bc * alpha) + (add_rc_bc * beta);
			// alpha * mul(r_b, b, c) + beta * mul(r_c, b, c)
			let mul_alpha_beta = (mul_rb_bc * alpha) + (mul_rc_bc * beta);

			let fbc_add_alpha_beta =
				ComposedMultiLinearPolynomial::new(vec![add_alpha_beta, wb_add_wc]);
			let fbc_mul_alpha_beta =
				ComposedMultiLinearPolynomial::new(vec![mul_alpha_beta, wb_mul_wc]);

			let (sumcheck_proof, challenges) = MultiComposedSumcheckProver::prove_partial(
				&vec![fbc_add_alpha_beta, fbc_mul_alpha_beta],
				&claimed_sum,
			)
			.unwrap();

			transcript.append(&sumcheck_proof.to_bytes());
			sumcheck_proofs.push(sumcheck_proof);

			let (b, c) = challenges.split_at(&challenges.len() / 2);

			let eval_wb = wb.evaluation(&b);
			let eval_wc = wc.evaluation(&c);
			w_i_b.push(eval_wb);
			w_i_c.push(eval_wc);

			r_b = b.to_vec();
			r_c = c.to_vec();

			alpha = transcript.transform_challenge_to_field::<F>();
			beta = transcript.transform_challenge_to_field::<F>();

			claimed_sum = alpha * eval_wb + beta * eval_wc;
		}

		GKRProof { sumcheck_proofs, w_i_b, w_i_c, w_0_mle }
	}

	pub fn verify<F: PrimeField>(circuit: &Circuit, input: &[F], proof: &GKRProof<F>) -> bool {
		if proof.sumcheck_proofs.len() != proof.w_i_b.len()
			|| proof.sumcheck_proofs.len() != proof.w_i_c.len()
		{
			return false;
		}

		let mut transcript = Transcript::new();
		transcript.append(&proof.w_0_mle.to_bytes());

		let n_r: Vec<F> =
			transcript.sample_n_as_field_element::<F>(proof.w_0_mle.number_of_variables);
		let mut claimed_sum = proof.w_0_mle.evaluation(&n_r.clone().as_slice());

		let mut r_b: Vec<F> = vec![];
		let mut r_c: Vec<F> = vec![];
		let mut alpha: F = F::zero();
		let mut beta: F = F::zero();

		let (add_mle_1, mul_mle_1) = circuit.add_i_mul_ext::<F>(0);
		let (status, sum) = perform_layer_one_verify_sumcheck(
			&add_mle_1,
			&mul_mle_1,
			&proof.sumcheck_proofs[0],
			n_r,
			&claimed_sum,
			&mut transcript,
			&proof.w_i_b[0],
			&proof.w_i_c[0],
		);

		if !status {
			return false;
		}

		claimed_sum = sum;

		for i in 1..proof.sumcheck_proofs.len() {
			if claimed_sum != proof.sumcheck_proofs[i].sum {
				return false;
			}

			transcript.append(&proof.sumcheck_proofs[i].to_bytes());

			let verify_subclaim =
				MultiComposedSumcheckVerifier::verify_partial(&proof.sumcheck_proofs[i]).unwrap();

			let (b, c) = verify_subclaim.challenges.split_at(&verify_subclaim.challenges.len() / 2);

			r_b = b.to_vec();
			r_c = c.to_vec();

			let wb = proof.w_i_b[i];
			let wc = proof.w_i_c[i];

			let alph = transcript.transform_challenge_to_field::<F>();
			let bta = transcript.transform_challenge_to_field::<F>();

			claimed_sum = alph * wb + bta * wc;

			alpha = alph;
			beta = bta;
		}

		let w_mle_input = w_mle(input.to_vec());

		let w_mle_rb_input = w_mle_input.evaluation(&r_b);
		let w_mle_rc_input = w_mle_input.evaluation(&r_c);

		let sum = alpha * w_mle_rb_input + beta * w_mle_rc_input;

		if claimed_sum != sum {
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
