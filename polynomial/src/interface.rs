use crate::univariate_polynomial::univariate::UnivariatePolynomial;
use ark_ff::PrimeField;

pub trait UnivariatePolynomialTrait<F: PrimeField> {
    fn new(data: Vec<F>) -> Self;
    fn evaluate(&self, point: F) -> F;
    fn interpolate(points: &[(F, F)]) -> UnivariatePolynomial<F>;
    fn degree(&self) -> F;
    fn zero() -> Self;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait MLETrait<F: PrimeField> {
    fn new(evaluations: Vec<F>) -> Self;
    fn partial_evaluation(&self, eval_point: F, variable_index: usize) -> Self;
    fn evaluation(&self, evaluation_points: &[F]) -> F;
}