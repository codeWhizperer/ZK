use crate::interface::UnivariatePolynomialTrait;
use crate::util::lagrange_basis;
use ark_ff::{PrimeField, BigInteger};
use std::{
	fmt::{Display, Formatter, Result},
	ops::{Add, Mul},
};

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct UnivariateMonomial<F: PrimeField> {
	pub coeff: F,
	pub pow: F,
}
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct UnivariatePolynomial<F: PrimeField> {
	pub monomial: Vec<UnivariateMonomial<F>>,
}

impl<F: PrimeField> UnivariatePolynomialTrait<F> for UnivariatePolynomial<F> {
	fn new(data: Vec<F>) -> Self {
		let mut monomial: Vec<UnivariateMonomial<F>> = Vec::new();

		for n in (0..data.len()).step_by(2) {
			if n < data.len() - 1 {
				let monomial_value: UnivariateMonomial<F> =
					UnivariateMonomial { coeff: data[n], pow: data[n + 1] };
				monomial.push(monomial_value);
			} else if n == data.len() - 1 {
				let monomial_value: UnivariateMonomial<F> =
					UnivariateMonomial { coeff: data[n], pow: F::zero() };
				monomial.push(monomial_value);
			}
		}

		UnivariatePolynomial { monomial }
	}

	fn zero() -> Self {
		Self { monomial: vec![] }
	}
	fn evaluate(&self, point: F) -> F {
		let mut point_evaluation: F = F::from(0_u8);
		for n in self.monomial.iter() {
			let coefficient = n.coeff;
			let n_pow: <F as PrimeField>::BigInt = n.pow.into();
			let power = point.pow(&n_pow);

			let evaluation = coefficient * power;
			point_evaluation += evaluation;
		}

		point_evaluation
	}

	fn interpolate(points: &[(F, F)]) -> UnivariatePolynomial<F> {
		let mut result: Vec<F> = vec![F::zero(); points.len()];

		for (i, &(_, y_i)) in points.iter().enumerate() {
			let l_i: Vec<F> = lagrange_basis(points, i);
			let l_i: Vec<F> = l_i.into_iter().map(|coeff| coeff * y_i).collect();

			for (k, &coeff) in l_i.iter().enumerate() {
				result[k] += coeff;
			}
		}

		let monomial: Vec<UnivariateMonomial<F>> = result
			.into_iter()
			.enumerate()
			.filter(|&(_, coeff)| coeff != F::zero())
			.map(|(pow, coeff)| UnivariateMonomial { coeff, pow: F::from(pow as u64) })
			.collect();

		UnivariatePolynomial { monomial }
	}

	fn degree(&self) -> F {
		let mut highest_degree: F = F::from(0_u8);
		for m in self.monomial.iter() {
			if m.pow > highest_degree {
				highest_degree = m.pow;
			}
		}

		highest_degree.try_into().unwrap()
	}

	 fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for p in self.monomial.iter() {
            bytes.extend_from_slice(&p.coeff.into_bigint().to_bytes_be());
            bytes.extend_from_slice(&p.pow.into_bigint().to_bytes_be());
        }
        bytes
    }

}

impl<F: PrimeField> Mul for UnivariatePolynomial<F> {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		let mut result_monomial: Vec<UnivariateMonomial<F>> = Vec::new();

		for lhs_mn in &self.monomial {
			for rhs_mn in &rhs.monomial {
				let new_coeff = lhs_mn.coeff * rhs_mn.coeff;
				let new_pow = lhs_mn.pow + rhs_mn.pow;

				let mut found_like_terms = false;
				for res_mn in &mut result_monomial {
					if res_mn.pow == new_pow {
						res_mn.coeff += new_coeff;
						found_like_terms = true;
						break;
					}
				}

				if !found_like_terms {
					result_monomial.push(UnivariateMonomial { coeff: new_coeff, pow: new_pow });
				}
			}
		}

		UnivariatePolynomial { monomial: result_monomial }
	}
}

impl<F: PrimeField> Add for UnivariatePolynomial<F> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut result_monomials = Vec::new();
		let mut lhs_iter = self.monomial.into_iter();
		let mut rhs_iter = rhs.monomial.into_iter();
		let mut lhs_mn = lhs_iter.next();
		let mut rhs_mn = rhs_iter.next();

		while lhs_mn.is_some() || rhs_mn.is_some() {
			match (lhs_mn.clone(), rhs_mn.clone()) {
				(Some(l), Some(r)) => {
					if l.pow == r.pow {
						result_monomials
							.push(UnivariateMonomial { coeff: l.coeff + r.coeff, pow: l.pow });
						lhs_mn = lhs_iter.next();
						rhs_mn = rhs_iter.next();
					} else if l.pow < r.pow {
						result_monomials.push(l);
						lhs_mn = lhs_iter.next();
					} else {
						result_monomials.push(r);
						rhs_mn = rhs_iter.next();
					}
				},
				(Some(l), None) => {
					result_monomials.push(l);
					lhs_mn = lhs_iter.next();
				},
				(None, Some(r)) => {
					result_monomials.push(r);
					rhs_mn = rhs_iter.next();
				},
				(None, None) => break,
			}
		}

		UnivariatePolynomial { monomial: result_monomials }
	}
}

impl<F: PrimeField> Display for UnivariatePolynomial<F> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		for (index, mn) in self.monomial.iter().enumerate() {
			if index == 0 {
				write!(f, "{}", mn.coeff)?;
			} else {
				if mn.pow == F::from(0_u8) || mn.coeff == F::from(0_u8) {
					write!(f, " + {}", mn.coeff)?;
				} else if mn.pow == F::from(1_u8) {
					write!(f, " + {}x", mn.coeff)?;
				} else {
					write!(f, " + {}x^{}", mn.coeff, mn.pow)?;
				}
			}
		}
		Ok(())
	}
}

mod tests {
	use super::*;
	use ark_ff::MontConfig;
	use ark_ff::{Fp64, MontBackend};

	#[derive(MontConfig)]
	#[modulus = "17"]
	#[generator = "3"]
	struct FqConfig;
	type Fq = Fp64<MontBackend<FqConfig, 1>>;

	#[test]
	fn test_polynomial_evaluation() {
		let data = vec![
			Fq::from(5_u8),
			Fq::from(0_u8),
			Fq::from(2_u8),
			Fq::from(1_u8),
			Fq::from(4_u8),
			Fq::from(6_u8),
		];
		let polynomial = UnivariatePolynomial::new(data);
		let evaluation = polynomial.evaluate(Fq::from(2_u8));

		assert_eq!(evaluation, Fq::from(10_u8));
	}

	#[test]
	fn test_polynomial_addition() {
		let data = vec![Fq::from(5_u8), Fq::from(0_u8)];
		let data2 = vec![Fq::from(2_u8), Fq::from(1_u8)];


		let polynomial1 = UnivariatePolynomial::new(data);
		let polynomial2 = UnivariatePolynomial::new(data2);

		assert_eq!(
			polynomial1 + polynomial2,
			UnivariatePolynomial::new(vec![
				Fq::from(5_u8),
				Fq::from(0_u8),
				Fq::from(2_u8),
				Fq::from(1_u8),
			])
		);

		let data3 = vec![Fq::from(5_u8), Fq::from(0_u8), Fq::from(5_u8), Fq::from(2_u8)];
		let data4 = vec![Fq::from(2_u8), Fq::from(1_u8), Fq::from(2_u8), Fq::from(2_u8)];
		let polynomial1 = UnivariatePolynomial::new(data3);
		let polynomial2 = UnivariatePolynomial::new(data4);

		assert_eq!(
			polynomial1 + polynomial2,
			UnivariatePolynomial::new(vec![
				Fq::from(5_u8),
				Fq::from(0_u8),
				Fq::from(2_u8),
				Fq::from(1_u8),
				Fq::from(7_u8),
				Fq::from(2_u8),
			])
		);
	}

	#[test]
	fn test_polynomial_multiplication() {
		let data = vec![Fq::from(5_u8), Fq::from(0_u8)];
		let data2 = vec![Fq::from(2_u8), Fq::from(1_u8)];

		let polynomial1 = UnivariatePolynomial::new(data);
		let polynomial2 = UnivariatePolynomial::new(data2);

		assert_eq!(
			polynomial1 + polynomial2,
			UnivariatePolynomial::new(vec![
				Fq::from(5_u8),
				Fq::from(0_u8),
				Fq::from(2_u8),
				Fq::from(1_u8),
			])
		);

		let data3 = vec![
			Fq::from(5_u8),
			Fq::from(0_u8),
			Fq::from(2_u8),
			Fq::from(1_u8),
			Fq::from(4_u8),
			Fq::from(6_u8),
		];
		let data4 =
			vec![Fq::from(4), Fq::from(0), Fq::from(3), Fq::from(2), Fq::from(4), Fq::from(5)];

		let polynomial3 = UnivariatePolynomial::new(data3);
		let polynomial4 = UnivariatePolynomial::new(data4);

		assert_eq!(
			polynomial3 * polynomial4,
			UnivariatePolynomial::new(vec![
				Fq::from(3_u8),
				Fq::from(0_u8),
				Fq::from(15_u8),
				Fq::from(2_u8),
				Fq::from(3_u8),
				Fq::from(5_u8),
				Fq::from(8_u8),
				Fq::from(1_u8),
				Fq::from(6_u8),
				Fq::from(3_u8),
				Fq::from(7_u8),
				Fq::from(6_u8),
				Fq::from(12_u8),
				Fq::from(8_u8),
				Fq::from(16_u8),
				Fq::from(11_u8),
			])
		);
	}

	#[test]
	fn test_polynomial_interpolate() {
		let interpolation = UnivariatePolynomial::interpolate(&[
			(Fq::from(1), Fq::from(2)),
			(Fq::from(2), Fq::from(3)),
			(Fq::from(4), Fq::from(11)),
		]);

		let interpolation_check = UnivariatePolynomial::new(vec![
			Fq::from(3_u8),
			Fq::from(0_u8),
			Fq::from(15_u8),
			Fq::from(1_u8),
			Fq::from(1_u8),
			Fq::from(2_u8),
		]);
		assert_eq!(interpolation, interpolation_check);

		// to test the evaluation of the polynomial
		let evaluation = interpolation.evaluate(Fq::from(2_u8));
		assert_eq!(evaluation, Fq::from(3_u8));
	}

}
