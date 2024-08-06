use crate::{ UnivariatePolynomial};
use ark_ff::PrimeField;

pub trait UnivariatePolynomialTrait<F: PrimeField> {
    fn new(data: Vec<F>) -> Self;
    fn evaluate(&self, point: F) -> F;
    fn interpolate(points: &[(F, F)]) -> UnivariatePolynomial<F>;
    fn degree(&self) -> F;
}