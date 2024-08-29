use ark_ff::{BigInteger, PrimeField};

use crate::multilinear::interface::MultiLinearPolynomialEvaluationFormTrait;

use super::utils::{compute_number_of_variables, pick_pairs_with_index};
use std::ops::{Add, AddAssign,Mul};

#[derive(Debug, Clone, PartialEq, Default,Eq,Hash)]
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

	fn partial_evaluation(&self, eval_point: &F, variable_index: &usize) -> Self {
        let new_evaluation: &Vec<F> = &self.evaluations;

        let mut result: Vec<F> = Vec::with_capacity(self.evaluations.len() / 2);

        for (i, j) in pick_pairs_with_index(self.evaluations.len(), *variable_index) {
            let y1: &F = &new_evaluation[i];
            let y2: &F = &new_evaluation[j];

            let res_y: F = (*eval_point * y2) + ((F::one() - eval_point) * y1);
            result.push(res_y);
        }

        Self {
            number_of_variables: self.number_of_variables - 1,
            evaluations: result,
        }
    }

	fn partial_evaluations(&self, points: &[F], variable_indices: &Vec<usize>) -> Self {
        let mut evaluation = self.clone();

        if points.len() != variable_indices.len() {
            panic!(
                "The length of evaluation_points and variable_indices should be the same: {}, {}",
                points.len(),
                variable_indices.len()
            );
        }

        for i in 0..points.len() {
            evaluation = evaluation.partial_evaluation(&points[i], &variable_indices[i]);
        }

        evaluation
    }
	 fn add_distinct(&self, rhs: &Self) -> Self {
        let mut new_evaluations = Vec::new();
        let repeat_sequence = rhs.evaluations.len();

        for i in 0..self.evaluations.len() {
            for j in 0..repeat_sequence {
                new_evaluations.push(self.evaluations[i] + rhs.evaluations[j]);
            }
        }

        Self::new(new_evaluations)
    }

     fn mul_distinct(&self, rhs: &Self) -> Self {
        let mut new_evaluations = Vec::new();
        let repeat_sequence = rhs.evaluations.len();

        for i in 0..self.evaluations.len() {
            for j in 0..repeat_sequence {
                new_evaluations.push(self.evaluations[i] * rhs.evaluations[j]);
            }
        }

        Self::new(new_evaluations)
    }

	fn evaluation(&self, evaluation_points: &[F]) -> F {
        assert_eq!(
            evaluation_points.len(),
            self.number_of_variables,
            "Number of evaluation points must match the number of variables"
        );

        let mut eval_result:MultiLinearPolynomialEvaluationForm<F> = self.clone();
        for i in 0..evaluation_points.len() {
            eval_result = eval_result.partial_evaluation(&evaluation_points[i], &0);
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
     fn sum_over_the_boolean_hypercube(&self) -> F {
        self.evaluations
            .iter()
            .fold(F::zero(), |acc, val| acc + val)
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

	fn interpolate(y_s:&[F])->Self{
		let (_, eval_size) = compute_number_of_variables(y_s.len() as u128);
		let mut y_s = y_s.to_vec();
		y_s.resize(eval_size as usize, F::ZERO);

		Self::new(y_s)
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

impl<F: PrimeField> Mul<F> for MultiLinearPolynomialEvaluationForm<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        let lhs = self.evaluations;
        let mut res = vec![];

        for i in 0..lhs.len() {
            res.push(lhs[i] * rhs)
        }

        Self {
            number_of_variables: self.number_of_variables,
            evaluations: res,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::interface::MLETrait;
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
    fn test_partial_evaluation_1() {
        let evaluations = vec![Fq::from(3), Fq::from(1), Fq::from(2), Fq::from(5)];
        let polynomial = MultiLinearPolynomialEvaluationForm::new(evaluations);

        let evaluation_point = Fq::from(5);
        let new_polynomial = polynomial.partial_evaluation(&evaluation_point, &0);

        let expected_polynomial = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(15), Fq::from(4)]);

        assert_eq!(new_polynomial, expected_polynomial);
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
        let polynomial = MultiLinearPolynomialEvaluationForm::new(evaluations);

        // obtain: f(2,y,z) = 4yz + 4y + 6z + 9 at y = 3, z = 2 = 57
        let new_polynomial_x_1 = polynomial.partial_evaluation(&Fq::from(2), &0);
        // 4yz + 4y + 6z + 9
        let x_1_eval_result = new_polynomial_x_1.evaluation(&vec![Fq::from(3), Fq::from(2)]);
        assert_eq!(x_1_eval_result, Fq::from(57));

        // obtain: f(x,3,z) = 6xz + 3x + 6z + 15 at y = 3, z = 2 = 72
        let new_polynomial_y_1 = polynomial.partial_evaluation(&Fq::from(3), &1);
        // 6xz + 3x + 6z + 15
        let y_1_eval_result = new_polynomial_y_1.evaluation(&vec![Fq::from(3), Fq::from(2)]);
        assert_eq!(y_1_eval_result, Fq::from(72));

        // obtain: f(x,y,1) = 2xy + 3x + 4y + 9  at y = 3, z = 2 = 38
        let new_polynomial_z_1 = polynomial.partial_evaluation(&Fq::from(1), &2);
        // 2xy + 3x + 4y + 9
        let z_1_eval_result = new_polynomial_z_1.evaluation(&vec![Fq::from(3), Fq::from(2)]);
        assert_eq!(z_1_eval_result, Fq::from(38));
    }

    #[test]
    fn test_evaluation_1() {
        let evaluations = vec![Fq::from(3), Fq::from(1), Fq::from(2), Fq::from(5)];
        let polynomial = MultiLinearPolynomialEvaluationForm::new(evaluations);

        let points = vec![Fq::from(5), Fq::from(6)];
        let result_polynomial = polynomial.evaluation(&points);

        assert_eq!(result_polynomial, Fq::from(0_u8));
        assert_ne!(result_polynomial, Fq::from(3_u8));

        let evaluations_2 = vec![
            Fq::from(3),
            Fq::from(9),
            Fq::from(7),
            Fq::from(13),
            Fq::from(6),
            Fq::from(12),
            Fq::from(10),
            Fq::from(18),
        ];
        let polynomial_2 = MultiLinearPolynomialEvaluationForm::new(evaluations_2);
        let points_2 = vec![Fq::from(2), Fq::from(3), Fq::from(1)];
        let result_polynomial_2 = polynomial_2.evaluation(&points_2);
        assert_eq!(result_polynomial_2, Fq::from(5));
    }

    #[test]
    fn test_evaluation_2() {
        // f(a, b, c) = 2ab + 3bc
        let poly = MultiLinearPolynomialEvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(3),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(5),
        ]);

        let evaluation_result = poly.evaluation(&[Fq::from(2), Fq::from(3), Fq::from(4)]);
        assert_eq!(evaluation_result, Fq::from(48));
    }

    #[test]
    fn test_split_poly_into_two_and_sum_each_part() {
        let mut poly1 = MultiLinearPolynomialEvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(4),
        ]);
        let mut poly2 = MultiLinearPolynomialEvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(7),
            Fq::from(3),
            Fq::from(3),
            Fq::from(6),
            Fq::from(11),
        ]);
        let evaluation1 = poly1.split_poly();
        let evaluation2 = poly2.split_poly();

        let expected_polynomial1 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(2), Fq::from(10)]);
        let expected_polynomial2 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(9), Fq::from(6)]);

        assert_eq!(evaluation1, expected_polynomial1);
        assert_eq!(evaluation2, expected_polynomial2);
    }

    #[test]
    fn test_sum_over_boolean_hypercube() {
        let val = vec![
            Fq::from(1),
            Fq::from(2),
            Fq::from(3),
            Fq::from(4),
            Fq::from(5),
            Fq::from(6),
            Fq::from(7),
            Fq::from(8),
        ];

        let poly = MultiLinearPolynomialEvaluationForm::new(val);

        let res = poly.sum_over_the_boolean_hypercube();

        assert!(
            res == Fq::from(36),
            "Incorrect sum over the boolean hypercube"
        )
	}
}
