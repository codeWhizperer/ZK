use ark_ff::PrimeField;
use polynomial::{
	composed::{
		interface::ComposedMultilinearInterface, multilinear::ComposedMultiLinearPolynomial,
	},
	multilinear::{
		evaluation_form::MultiLinearPolynomialEvaluationForm,
		interface::MultiLinearPolynomialEvaluationFormTrait,
	},
};
use sum_check::multi_composedsumcheck::MultiComposedSumcheckVerifier;
use sum_check::multi_composedsumcheck::{ComposedSumcheckProof, MultiComposedSumcheckProver};
use transcript::transcription::Transcript;
pub fn label_to_binary_to_decimal(a: usize, b: usize, c: usize) -> usize {
	let a_shifted = a << 4;
	let b_shifted = b << 2;
	a_shifted | b_shifted | c
}

pub fn size_of_number_of_variable_at_each_layer(layer_index: usize) -> usize {
	if layer_index == 0 {
		return 1 << 3;
	}
	let layer_index_plus_one = layer_index + 1;
	let number_of_variable = layer_index + (2 * layer_index_plus_one);
	1 << number_of_variable
}
pub fn gen_w_mle<F: PrimeField>(
	evals: &[Vec<F>],
	layer_index: usize,
) -> MultiLinearPolynomialEvaluationForm<F> {
	//checks if layer index is out bound
	if layer_index >= evals.len() {
		panic!("Layer index out of bounds");
	}
	MultiLinearPolynomialEvaluationForm::interpolate(&evals[layer_index])
}

pub fn perform_layer_one_prove_sumcheck<F: PrimeField>(
	add_mle: &MultiLinearPolynomialEvaluationForm<F>,
	mul_mle: &MultiLinearPolynomialEvaluationForm<F>,
	w_mle: &MultiLinearPolynomialEvaluationForm<F>,
	n_r: &Vec<F>,
	sum: &F,
	transcript: &mut Transcript,
	sumcheck_proofs: &mut Vec<ComposedSumcheckProof<F>>,
	w_i_b: &mut Vec<F>,
	w_i_c: &mut Vec<F>,
) -> (F, F, F, Vec<F>, Vec<F>) {
	let add_rbc = add_mle.partial_evaluations(&n_r, &vec![0; n_r.len()]);
	let mul_rbc = mul_mle.partial_evaluations(&n_r, &vec![0; n_r.len()]);

	let wb = w_mle.clone();
	let wc = w_mle;

	let wb_add_wc = wb.add_distinct(&wc);
	let wb_mul_wc = wb.mul_distinct(&wc);

	let add_fbc = ComposedMultiLinearPolynomial::new(vec![add_rbc, wb_add_wc]);
	let mul_fbc = ComposedMultiLinearPolynomial::new(vec![mul_rbc, wb_mul_wc]);

	let (sumcheck_proof, challenges) =
		MultiComposedSumcheckProver::prove_partial(&vec![add_fbc, mul_fbc], &sum).unwrap();
	transcript.append(&sumcheck_proof.to_bytes());
	sumcheck_proofs.push(sumcheck_proof);

	let (rand_b, rand_c) = challenges.split_at(&challenges.len() / 2);

	let eval_w_i_b = wb.evaluation(&rand_b.to_vec());
	let eval_w_i_c = wc.evaluation(&rand_c.to_vec());
	w_i_b.push(eval_w_i_b);
	w_i_c.push(eval_w_i_c);

	let alpha = transcript.transform_challenge_to_field::<F>();
	let beta = transcript.transform_challenge_to_field::<F>();

	let new_claim: F = alpha * eval_w_i_b + beta * eval_w_i_c;

	let claimed_sum = new_claim;
	(claimed_sum, alpha, beta, rand_b.to_vec(), rand_c.to_vec())
}

pub fn perform_layer_one_verify_sumcheck<F: PrimeField>(
	add_mle: &MultiLinearPolynomialEvaluationForm<F>,
	mul_mle: &MultiLinearPolynomialEvaluationForm<F>,
	proof: &ComposedSumcheckProof<F>,
	n_r: Vec<F>,
	layer_one_expected_sum: &F,
	transcript: &mut Transcript,
	w_b: &F,
	w_c: &F,
) -> (bool, F) {
	if *layer_one_expected_sum != proof.sum {
		return (false, F::zero());
	}

	transcript.append(&proof.to_bytes());

	let verify_subclaim = MultiComposedSumcheckVerifier::verify_partial(proof).unwrap();

	let mut r_b_c = n_r;
	r_b_c.extend_from_slice(&verify_subclaim.challenges);

	let add_b_c = add_mle.evaluation(&r_b_c);
	let mul_b_c = mul_mle.evaluation(&r_b_c);

	let fbc_add = add_b_c * (*w_b + *w_c);
	let fbc_mul = mul_b_c * (*w_b * *w_c);

	let fbc_eval = fbc_add + fbc_mul;

	if fbc_eval != verify_subclaim.sum {
		return (false, F::zero());
	}

	let alpha = transcript.transform_challenge_to_field::<F>();
	let beta = transcript.transform_challenge_to_field::<F>();

	let new_claim: F = alpha * w_b + beta * w_c;

	(true, new_claim)
}

pub fn w_mle<F: PrimeField>(layer_eval: Vec<F>) -> MultiLinearPolynomialEvaluationForm<F> {
	MultiLinearPolynomialEvaluationForm::new(layer_eval)
}

mod tests {
	use super::*;
	#[test]
	fn test_label_binary_and_to_decimal() {
		assert_eq!(label_to_binary_to_decimal(0, 0, 1), 1);
		assert_eq!(label_to_binary_to_decimal(1, 2, 3), 27);
	}
}
