use ark_ff::{BigInteger, PrimeField};

use crate::multilinear::interface::MultiLinearPolynomialEvaluationFormTrait;

use super::utils::pick_pairs_with_index;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MultiLinearPolynomialEvaluationForm<F: PrimeField> {
	pub number_of_variables: usize,
	pub evaluations: Vec<F>,
}

impl<F: PrimeField> MultiLinearPolynomialEvaluationFormTrait<F>
	for MultiLinearPolynomialEvaluationForm<F>
{
	fn new(evaluations: Vec<F>) -> Self {
		let number_of_variables = (evaluations.len() as f64).log2() as usize;
		assert_eq!(
			evaluations.len(),
			1 << number_of_variables,
			"Num of evaluation must be equal 2^number_of_variable"
		);
		Self { number_of_variables, evaluations }
	}

	fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> Self {
		let new_evaluation: &Vec<F> = &self.evaluations;
		let mut result: Vec<F> = Vec::with_capacity(self.evaluations.len() / 2);

		for (i, j) in pick_pairs_with_index(self.evaluations.len(), variable_index) {
			let y_1 = &new_evaluation[i];
			let y_2 = &new_evaluation[j];

			let result_y: F = (evaluation_point * y_2) + ((F::one() - evaluation_point) * y_1);
			result.push(result_y);
		}
		Self { number_of_variables: self.number_of_variables - 1, evaluations: result }
	}

	fn evaluation(&self, evaluation_points: &[F]) -> F {
		assert_eq!(
			evaluation_points.len(),
			self.number_of_variables,
			"Number of evaluation points must match the number of variables"
		);

		let mut eval_result: MultiLinearPolynomialEvaluationForm<F> = self.clone();
		for i in 0..evaluation_points.len() {
			eval_result = eval_result.partial_evaluation(evaluation_points[i], 0);
		}

		eval_result.evaluations[0]
	}

	fn generate_variable_names(&self) -> Vec<String> {
		(0..self.number_of_variables)
			.map(|i| (b'a' + i as u8) as char)
			.map(|c| c.to_string())
			.collect()
	}

	fn zero(num_vars: usize) -> Self {
		let addictive = MultiLinearPolynomialEvaluationForm::new(vec![F::zero(); 1 << num_vars]);
		addictive
	}
	fn to_bytes(&self) -> Vec<u8> {
		let mut m_ploy_bytes = Vec::new();

		for eval in &self.evaluations {
			let big_int = eval.into_bigint().to_bytes_be();
			m_ploy_bytes.extend_from_slice(&big_int);
		}

		m_ploy_bytes
	}

	fn split_poly(&mut self) -> MultiLinearPolynomialEvaluationForm<F> {
		assert!(self.evaluations.len() > 1, "Cannot split with less than two elements");

		let mid: usize = self.evaluations.len() / 2;
		let first_half: F = self.evaluations[..mid].iter().fold(F::zero(), |acc, &x| acc + x);
		let second_half: F = self.evaluations[mid..].iter().fold(F::zero(), |acc, &x| acc + x);
		Self::new(vec![first_half, second_half])
	}
	 fn is_zero(&self) -> bool {
        self.evaluations.iter().all(|&eval| eval.is_zero())
    }
}

impl<F: PrimeField> Add for MultiLinearPolynomialEvaluationForm<F> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let lhs = self.evaluations;
		let mut res = vec![];

		for i in 0..lhs.len() {
			res.push(lhs[i] + rhs.evaluations[i])
		}

		Self { number_of_variables: self.number_of_variables, evaluations: res }
	}
}

impl<F: PrimeField> AddAssign for MultiLinearPolynomialEvaluationForm<F> {
	fn add_assign(&mut self, other: Self) {
		if self.number_of_variables != other.number_of_variables {
			panic!("The number of variables in the two polynomials must be the same. Self: {}, Other: {}", self.number_of_variables, other.number_of_variables);
		}

		for i in 0..self.evaluations.len() {
			self.evaluations[i] += other.evaluations[i];
		}
	}
}

#[cfg(test)]
mod test {
	use std::vec;

	use crate::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;
	use crate::multilinear::interface::MultiLinearPolynomialEvaluationFormTrait;
	use ark_ff::MontConfig;
	use ark_ff::{Fp64, MontBackend};
	#[derive(MontConfig)]
	#[modulus = "17"]
	#[generator = "3"]
	struct FqConfig;
	type Fq = Fp64<MontBackend<FqConfig, 1>>;

	#[test]
	fn test_new_multilinear_polynomial() {
		let evaluations = vec![Fq::from(3), Fq::from(1), Fq::from(2), Fq::from(5)];
		let new_poly = MultiLinearPolynomialEvaluationForm::new(evaluations);
		assert_eq!(new_poly.evaluations.len(), 1 << new_poly.number_of_variables);
	}

	#[test]
	fn test_partial_evaluation() {
		let evaluations = vec![Fq::from(3), Fq::from(1), Fq::from(2), Fq::from(5)];
		let polynomial = MultiLinearPolynomialEvaluationForm::new(evaluations);
		let evaluation_point = Fq::from(5);
		let new_polynomial = MultiLinearPolynomialEvaluationForm::partial_evaluation(
			&polynomial,
			evaluation_point,
			0,
		);
		let expected_polynomial =
			MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(-2), Fq::from(21)]);
		assert_eq!(new_polynomial, expected_polynomial);
	}

	#[test]
	fn test_full_evaluation() {
		let evaluations = vec![
			Fq::from(0),
			Fq::from(0),
			Fq::from(0),
			Fq::from(3),
			Fq::from(0),
			Fq::from(0),
			Fq::from(2),
			Fq::from(5),
		];
		let polynomial = MultiLinearPolynomialEvaluationForm::new(evaluations);
		let evaluation_points = vec![Fq::from(2), Fq::from(3), Fq::from(4)];
		let evaluation_result =
			MultiLinearPolynomialEvaluationForm::evaluation(&polynomial, &evaluation_points);
		assert_eq!(evaluation_result, Fq::from(48));
	}

	#[test]
	fn test_partial_evaluation_2() {
		let evaluations = vec![
			Fq::from(3),
			Fq::from(9),
			Fq::from(7),
			Fq::from(13),
			Fq::from(6),
			Fq::from(12),
			Fq::from(10),
			Fq::from(18),
		];

		let evaluations2 = vec![
			Fq::from(3),
			Fq::from(9),
			Fq::from(7),
			Fq::from(13),
			Fq::from(6),
			Fq::from(12),
			Fq::from(10),
			Fq::from(18),
		];
		let new_poly = MultiLinearPolynomialEvaluationForm::new(evaluations);
		let points = vec![Fq::from(2), Fq::from(3), Fq::from(1)];

		let evaluation_result = MultiLinearPolynomialEvaluationForm::evaluation(&new_poly, &points);

		assert_eq!(evaluation_result, Fq::from(39));

		let poly2 = MultiLinearPolynomialEvaluationForm::new(evaluations2);
		let point2 = vec![Fq::from(3), Fq::from(2)];
		let partial_eval =
			MultiLinearPolynomialEvaluationForm::partial_evaluation(&poly2, Fq::from(3), 1);
		let expected = MultiLinearPolynomialEvaluationForm::evaluation(&partial_eval, &point2);
		assert_eq!(expected, Fq::from(72));
	}

	#[test]
    fn test_split_poly() {
        let mut poly = MultiLinearPolynomialEvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(4),
        ]);

        let evaluation = poly.split_poly();
        let expected_polynomial1 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(2), Fq::from(10)]);
        assert_eq!(evaluation, expected_polynomial1);
    }
}
