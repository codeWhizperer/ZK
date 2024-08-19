use crate::composed::interface::ComposedMultilinearInterface;
use crate::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;
use crate::multilinear::interface::MultiLinearPolynomialEvaluationFormTrait;

use ark_ff::PrimeField;

#[derive(Debug, Clone,PartialEq, Eq,Hash)]
pub struct ComposedMultiLinearPolynomial<F: PrimeField> {
	pub multilineal_polynomial: Vec<MultiLinearPolynomialEvaluationForm<F>>,
}

impl<F: PrimeField> ComposedMultilinearInterface<F> for ComposedMultiLinearPolynomial<F> {
	fn new(multilineal_polynomial: Vec<MultiLinearPolynomialEvaluationForm<F>>) -> Self {
		let number_of_variables: usize = multilineal_polynomial[0].number_of_variables;
		assert!(multilineal_polynomial
			.iter()
			.all(|x| x.number_of_variables == number_of_variables));
		Self { multilineal_polynomial }
	}

	fn number_of_variables(&self) -> usize {
		let number_of_variables = self.multilineal_polynomial[0].number_of_variables;
		number_of_variables
	}

	fn zero(&self) -> Self {
		Self { multilineal_polynomial: vec![] }
	}

	fn is_zero(&self) -> bool {
		self.multilineal_polynomial.is_empty()
			|| self
				.multilineal_polynomial
				.iter()
				.all(|p: &MultiLinearPolynomialEvaluationForm<F>| p.evaluations.is_empty())
	}

	fn multilineal_to_bytes(&self) -> Vec<u8> {
		let mut mul_byte = Vec::new();

		for mle in &self.multilineal_polynomial {
			mul_byte.extend(mle.to_bytes())
		}
		mul_byte
	}

	fn evaluation(&self, points: &[F]) -> F {
		let mut poly_result = F::one();

		for mle in &self.multilineal_polynomial {
			let poly_eval = mle.evaluation(points);
			poly_result *= poly_eval;
		}
		poly_result
	}

	fn partial_evaluation(
		&self,
		evaluation_point: F,
		variable_index: usize,
	) -> ComposedMultiLinearPolynomial<F> {
		let mut partial_eval_result = Vec::new();
		for mle in &self.multilineal_polynomial {
			partial_eval_result.push(mle.partial_evaluation(&evaluation_point, &variable_index));
		}
		ComposedMultiLinearPolynomial { multilineal_polynomial: partial_eval_result }
	}

	fn elementwise_addition(&self) -> Vec<F> {
		let polynomial_length = &self.multilineal_polynomial[0].evaluations.len();
		(0..*polynomial_length)
			.map(|x| {
				self.multilineal_polynomial
					.iter()
					.map(|y: &MultiLinearPolynomialEvaluationForm<F>| y.evaluations[x])
					.sum()
			})
			.collect()
	}

	fn elementwise_product(&self) -> Vec<F> {
		let polynomial_length = &self.multilineal_polynomial[0].evaluations.len();
		(0..*polynomial_length)
			.map(|x| {
				self.multilineal_polynomial
					.iter()
					.map(|y: &MultiLinearPolynomialEvaluationForm<F>| y.evaluations[x])
					.product()
			})
			.collect()
	}
	fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        for poly in &self.multilineal_polynomial {
            bytes.extend_from_slice(&poly.to_bytes());
        }

        bytes
    }

	fn max_degree(&self) -> usize {
	self.multilineal_polynomial.len()
}
}

#[cfg(test)]
mod tests {
	use ark_ff::MontConfig;
	use ark_ff::{Fp64, MontBackend};

	use super::*;

	#[derive(MontConfig)]
	#[modulus = "17"]
	#[generator = "3"]
	struct FqConfig;
	type Fq = Fp64<MontBackend<FqConfig, 1>>;

	#[test]
	fn composed_test_evaluation() {
		let mle1: MultiLinearPolynomialEvaluationForm<ark_ff::Fp<MontBackend<FqConfig, 1>, 1>> =
			MultiLinearPolynomialEvaluationForm::new(vec![
				Fq::from(0),
				Fq::from(1),
				Fq::from(2),
				Fq::from(3),
			]);
		let mle2: MultiLinearPolynomialEvaluationForm<ark_ff::Fp<MontBackend<FqConfig, 1>, 1>> =
			MultiLinearPolynomialEvaluationForm::new(vec![
				Fq::from(0),
				Fq::from(0),
				Fq::from(0),
				Fq::from(1),
			]);

		let mles: ComposedMultiLinearPolynomial<ark_ff::Fp<MontBackend<FqConfig, 1>, 1>> =
			ComposedMultiLinearPolynomial::new(vec![mle1, mle2]);
		let evaluation = mles.evaluation(&vec![Fq::from(2), Fq::from(3)]);

		assert_eq!(evaluation, Fq::from(42));
	}

	#[test]
	fn test_partial_evaluation() {
		let mle1 = MultiLinearPolynomialEvaluationForm::new(vec![
			Fq::from(0),
			Fq::from(1),
			Fq::from(2),
			Fq::from(3),
		]);
		let mle2 = MultiLinearPolynomialEvaluationForm::new(vec![
			Fq::from(0),
			Fq::from(0),
			Fq::from(0),
			Fq::from(1),
		]);

		let mles = ComposedMultiLinearPolynomial::new(vec![mle1, mle2]);
		let partial_evaluation = mles.partial_evaluation(Fq::from(2), 0);

		let evaluation = partial_evaluation.evaluation(&vec![Fq::from(3)]);
		assert_eq!(evaluation, Fq::from(42));
	}

	#[test]
	fn test_element_product() {
		let mle1 = MultiLinearPolynomialEvaluationForm::new(vec![
			Fq::from(0),
			Fq::from(1),
			Fq::from(2),
			Fq::from(3),
		]);
		let mle2 = MultiLinearPolynomialEvaluationForm::new(vec![
			Fq::from(0),
			Fq::from(0),
			Fq::from(0),
			Fq::from(1),
		]);

		let mles = ComposedMultiLinearPolynomial::new(vec![mle1, mle2]);
		let element_product = mles.elementwise_product();
		assert_eq!(element_product, vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);
	}

	#[test]
	fn test_element_addtion() {
		let mle1 = MultiLinearPolynomialEvaluationForm::new(vec![
			Fq::from(0),
			Fq::from(1),
			Fq::from(2),
			Fq::from(3),
		]);
		let mle2 = MultiLinearPolynomialEvaluationForm::new(vec![
			Fq::from(0),
			Fq::from(0),
			Fq::from(0),
			Fq::from(1),
		]);

		let mles = ComposedMultiLinearPolynomial::new(vec![mle1, mle2]);
		let element_addition = mles.elementwise_addition();
		assert_eq!(element_addition, vec![Fq::from(0), Fq::from(1), Fq::from(2), Fq::from(4)]);
	}
}
