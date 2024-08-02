use std::io::Read;

use crate::interface::{ComposedSumCheckProof, SumCheckProof};
use crate::util::convert_field_to_byte;
use ark_ff::{BigInt, BigInteger, PrimeField};
use polynomial::composed::interface::ComposedMultilinearInterface;
use polynomial::composed::multilinear::ComposedMultiLinearPolynomial;
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
	fn calculate_sum(&mut self)  {
	self.sum =	self.polynomial.elementwise_addition().iter().sum()
	}

	fn prover(&mut self) -> ComposedSumCheckProof<F> {
		let mut transcript = Transcript::new();
		let poly_sum_byte = convert_field_to_byte(&self.sum);
		transcript.append(&poly_sum_byte);
		let mut round_polys = vec![];
		let mut poly = self.polynomial.clone();
		let mut challenges = vec![];
		for _ in 0..self.polynomial.number_of_variables() {
			let mut round_poly = vec![];
			for i in 0..=self.polynomial.max_degree() {
				let instance: F = self
					.polynomial
					.partial_evaluation(F::from(i as u128), 0)
					.elementwise_product()
					.iter()
					.sum();
				round_poly.push(instance);
			}
			transcript.append(&vec_to_bytes(&round_poly));
			let random_r = transcript.transform_challenge_to_field();
			poly = poly.partial_evaluation(random_r, 0);
			round_polys.push(round_poly);
			challenges.push(random_r);
		}
		ComposedSumCheckProof {
			polynomial: self.polynomial.clone(),
			sum: self.sum,
			round_poly: round_polys,
		}
	}

	fn verify(&mut self, proof: &ComposedSumCheckProof<F>) -> bool {
		todo!()
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
	use ark_ff::{Fp, MontConfig};
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
		let mut prover = SumCheck::new(composed.clone());
		prover.calculate_sum();
		prover.prover();

	}
}

