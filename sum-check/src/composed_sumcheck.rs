use std::io::Read;

use crate::interface::{ComposedSumCheckProof, SumCheckProof};
use crate::util::{convert_field_to_byte, transform_round_poly_to_uni_poly};
use ark_ff::{BigInt, BigInteger, PrimeField};
use polynomial::composed::interface::ComposedMultilinearInterface;
use polynomial::composed::multilinear::ComposedMultiLinearPolynomial;
use polynomial::interface::UnivariatePolynomialTrait;
use polynomial::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;
use polynomial::multilinear::interface::MultiLinearPolynomialEvaluationFormTrait;
use polynomial::UnivariatePolynomial;
use transcript::transcription::Transcript;

use crate::interface::ComposedSumCheckInterface;
#[derive(Clone, Debug)]
pub struct SumCheck<F: PrimeField> {
	pub polynomial: ComposedMultiLinearPolynomial<F>,
	pub sum: F,
	pub transcript: Transcript,
	pub round_poly: Vec<ComposedMultiLinearPolynomial<F>>,
}

impl<F: PrimeField> SumCheck<F> {
	pub fn new(poly: ComposedMultiLinearPolynomial<F>) -> Self {
		Self {
			polynomial: poly,
			sum: Default::default(),
			round_poly: Default::default(),
			transcript: Default::default(),
		}
	}
}

impl<F: PrimeField> ComposedSumCheckInterface<F> for SumCheck<F> {
	fn calculate_sum(&mut self) {
		self.sum = self.polynomial.elementwise_addition().iter().sum()
	}

	 fn prover(&self) -> (ComposedSumCheckProof<F>, Vec<F>) {
        // send sum as bytes to the transcript
        let mut transcript = Transcript::new();

        let mut current_poly: ComposedMultiLinearPolynomial<F> = self.polynomial.clone();
        let mut round_polys: Vec<Vec<F>> = vec![];
        let mut challenges: Vec<F> = vec![];

        for _ in 0..self.polynomial.number_of_variables() {
            let mut round_poly: Vec<F> = vec![];
            for i in 0..=current_poly.max_degree() {
                let round: F = current_poly
                    .partial_evaluation(F::from(i as u32), 0)
                    .elementwise_product()
                    .iter()
                    .sum::<F>();

                round_poly.push(round);
            }

            transcript.append(&vec_to_bytes(&round_poly));
            //get the random r
            let random_r: F = transcript.transform_challenge_to_field::<F>();
            challenges.push(random_r);
            round_polys.push(round_poly);

            current_poly = current_poly.partial_evaluation(random_r, 0);
        }

        (
            ComposedSumCheckProof {
                polynomial: self.polynomial.clone(),
                sum: self.sum,
               round_poly: round_polys,
            },
            challenges,
        )
    }
	// fn prover(&mut self) -> (ComposedSumCheckProof<F>, Vec<F>) {
	// 	let mut transcript = Transcript::new();
	// 	let poly_sum_byte = convert_field_to_byte(&self.sum);
	// 	transcript.append(&poly_sum_byte);
	// 	let mut round_polys = vec![];
	// 	let mut poly = self.polynomial.clone();
	// 	let mut challenges = vec![];
	// 	for _ in 0..self.polynomial.number_of_variables() {
	// 		let mut round_poly = vec![];
	// 		for i in 0..=self.polynomial.max_degree() {
	// 			let instance: F = self
	// 				.polynomial
	// 				.partial_evaluation(F::from(i as u128), 0)
	// 				.elementwise_product()
	// 				.iter()
	// 				.sum();
	// 			round_poly.push(instance);
	// 		}
	// 		transcript.append(&vec_to_bytes(&round_poly));
	// 		let random_r = transcript.transform_challenge_to_field();
	// 		poly = poly.partial_evaluation(random_r, 0);
	// 		round_polys.push(round_poly);
	// 		challenges.push(random_r);
	// 	}
	// 	(
	// 		ComposedSumCheckProof {
	// 			polynomial: self.polynomial.clone(),
	// 			sum: self.sum,
	// 			round_poly: round_polys,
	// 		},
	// 		challenges,
	// 	)
	// }

	// fn verify(&mut self, proof: &ComposedSumCheckProof<F>) -> bool {
	// 	let mut transcript = Transcript::new();
	// 	transcript.append(&self.polynomial.to_bytes());
	// 	transcript.append(&proof.sum.into_bigint().to_bytes_be());

	// 	let mut all_rands = Vec::new();

	// 	let mut claimed_sum = proof.sum;

	// 	for round_poly in proof.round_poly.iter() {
	// 		transcript.append(&vec_to_bytes(&round_poly));
	// 		let all_round = transcript.transform_challenge_to_field();
	// 		all_rands.push(all_round);

	// 		let round_polys_uni = transform_round_poly_to_uni_poly(&round_poly);
	// 		let univariate_poly = UnivariatePolynomial::interpolate(&round_polys_uni);

	// 		let eval_point =
	// 			univariate_poly.evaluate(F::zero()) + univariate_poly.evaluate(F::one());
	// 		if claimed_sum != eval_point {
	// 			return false;
	// 		}
	// 		claimed_sum = univariate_poly.evaluate(all_round);
	// 	}
	// 	proof.polynomial.evaluation(all_rands.as_slice()) == claimed_sum
	// }

	fn verify(&self, proof: &ComposedSumCheckProof<F>) -> bool {
		let mut transcript = Transcript::new();

		let mut claimed_sum = proof.sum;
		let mut challenges: Vec<F> = vec![];

		for round_poly in proof.round_poly.iter() {
			transcript.append(&vec_to_bytes(&round_poly));
			// genrate the challenge for this round
			let challenge: F = transcript.transform_challenge_to_field::<F>();
			challenges.push(challenge);

			let round_polys_uni: Vec<(F, F)> = transform_round_poly_to_uni_poly(&round_poly);
			let uni_poly: UnivariatePolynomial<F> =
				UnivariatePolynomial::interpolate(&round_polys_uni);

			let eval_p0_p1 = uni_poly.evaluate(F::zero()) + uni_poly.evaluate(F::one());
			if claimed_sum != eval_p0_p1 {
				return false;
			}

			// update the sum
			claimed_sum = uni_poly.evaluate(challenge);
		}

		proof.polynomial.evaluation(challenges.as_slice()) == claimed_sum
	}
}

fn vec_to_bytes<F: PrimeField>(poly: &Vec<F>) -> Vec<u8> {
	let mut bytes = Vec::new();
	for p in poly {
		bytes.extend_from_slice(&p.into_bigint().to_bytes_be());
	}
	bytes
}

#[cfg(test)]
mod tests {

	use super::*;
	use ark_ff::{ MontConfig};
	use ark_ff::{Fp64, MontBackend};
	use polynomial::composed::interface::ComposedMultilinearInterface;
	use polynomial::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;

	#[derive(MontConfig)]
	#[modulus = "17"]
	#[generator = "3"]
	struct FqConfig;
	type Fq = Fp64<MontBackend<FqConfig, 1>>;
	#[test]
	fn test_composed_calculate_sum() {
		let mle1 = MultiLinearPolynomialEvaluationForm::new(vec![
			Fq::from(0),
			Fq::from(0),
			Fq::from(0),
			Fq::from(2),
		]);

		let mle2 = MultiLinearPolynomialEvaluationForm::new(vec![
			Fq::from(2),
			Fq::from(2),
			Fq::from(2),
			Fq::from(4),
		]);
		let poly = ComposedMultiLinearPolynomial::new(vec![mle1, mle2]);
		let mut prover = SumCheck::new(poly);
		prover.calculate_sum();
		assert_eq!(prover.sum, Fq::from(12));
	}

	#[test]
	fn test_sum_check_proof() {
		let poly = MultiLinearPolynomialEvaluationForm::new(vec![
			Fq::from(0),
			Fq::from(0),
			Fq::from(2),
			Fq::from(7),
			Fq::from(3),
			Fq::from(3),
			Fq::from(6),
			Fq::from(11),
		]);

		let composed: ComposedMultiLinearPolynomial<Fq> =
			ComposedMultiLinearPolynomial::new(vec![poly]);
		let mut sumcheck = SumCheck::new(composed);
		sumcheck.calculate_sum();
		let (proof, _) = sumcheck.prover();
		let verifier = sumcheck.verify(&proof);
		assert_eq!(verifier, true)
	}
}
