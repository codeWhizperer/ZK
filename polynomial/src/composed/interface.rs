use ark_ff::PrimeField;
use crate::composed::multilinear::ComposedMultiLinearPolynomial;
use crate::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;

pub trait ComposedMultilinearInterface<F: PrimeField> {
    fn new(multilineal_polynomial: Vec<MultiLinearPolynomialEvaluationForm<F>>) -> Self;
    fn number_of_variables(&self) -> usize ;
    fn zero(&self) -> Self ;
    fn is_zero(&self) -> bool ;
    fn multilineal_to_bytes(&self) -> Vec<u8>;
    fn elementwise_product(&self) -> Vec<F>;
    fn elementwise_addition(&self) -> Vec<F>;
    fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> ComposedMultiLinearPolynomial<F>;
    fn evaluation(&self, points:&[F]) -> F;
}