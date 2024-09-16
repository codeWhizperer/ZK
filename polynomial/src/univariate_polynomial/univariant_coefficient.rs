use std::ops::{Add, Mul};

use crate::{
	univariate_polynomial::interface::{PolynomialInterface, UnivariantPolynomialInterface},
	util::lbasis,
};
use ark_ff::{BigInteger, PrimeField};
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct UnivariantPolynomial<F: PrimeField> {
	pub coefficients: Vec<F>,
}

impl<F: PrimeField> PolynomialInterface<F> for UnivariantPolynomial<F> {
	type Point = F;

	fn degree(&self) -> usize {
		if self.coefficients.is_empty() {
			return 0;
		}
		self.coefficients.len() - 1
	}

	fn evaluate(&self, x: &F) -> F {
		self.coefficients.iter().rev().fold(F::zero(), |acc, c| acc * x + c)
	}

	fn is_zero(&self) -> bool {
		self.coefficients.is_empty()
	}
}

impl<F: PrimeField> UnivariantPolynomialInterface<F> for UnivariantPolynomial<F> {
	fn from_coefficients_slice(coefficients: &[F]) -> Self {
		UnivariantPolynomial { coefficients: coefficients.to_vec() }
	}
	fn from_coefficients_vec(coefficients: Vec<F>) -> Self {
		UnivariantPolynomial { coefficients }
	}
	fn coefficients(&self) -> &[F] {
		&self.coefficients
	}
	fn interpolate(point_ys: Vec<F>, domain: Vec<F>) -> Self {
		let langrange_poly_vec = lbasis(&domain, &point_ys);
		let langrange_poly = langrange_poly_vec
			.iter()
			.fold(UnivariantPolynomial::new(vec![]), |acc, x| acc + x.clone());
		langrange_poly
	}
}

impl<F: PrimeField> UnivariantPolynomial<F> {
	pub fn new(coefficients: Vec<F>) -> Self {
		UnivariantPolynomial { coefficients }
	}
	pub fn zero() -> Self {
		UnivariantPolynomial::new(vec![])
	}
	pub fn one() -> Self {
		UnivariantPolynomial::new(vec![F::one()])
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		let mut bytes = vec![];
		for c in &self.coefficients {
			let big_int = c.into_bigint().to_bytes_be();
			bytes.extend_from_slice(&big_int);
		}
		bytes
	}
}

impl<F: PrimeField> Mul for UnivariantPolynomial<F> {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		if self.is_zero() || rhs.is_zero() {
			return UnivariantPolynomial::new(vec![]);
		}
		let product_degree = self.degree() + rhs.degree();

		let mut polynomial_product_coefficients = vec![F::zero(); product_degree + 1];
		for i in 0..=self.degree() {
			for j in 0..=rhs.degree() {
				polynomial_product_coefficients[i + j] += self.coefficients[i] * rhs.coefficients[j]
			}
		}
		UnivariantPolynomial::new(polynomial_product_coefficients)
	}
}

impl<F: PrimeField> Add for UnivariantPolynomial<F> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let result = if self.degree() >= other.degree() {
			let mut result_coff = Vec::new();

			for i in 0..self.coefficients.len() {
				result_coff
					.push(self.coefficients[i] + other.coefficients.get(i).unwrap_or(&F::zero()));
			}

			UnivariantPolynomial::from_coefficients_vec(result_coff)
		} else {
			let mut result_coff = Vec::new();

			for i in 0..other.coefficients.len() {
				result_coff
					.push(other.coefficients[i] + self.coefficients.get(i).unwrap_or(&F::zero()));
			}

			UnivariantPolynomial::from_coefficients_vec(result_coff)
		};

		result
	}
}
