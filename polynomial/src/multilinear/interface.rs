use ark_ff::PrimeField;
use crate::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;
pub trait MultiLinearPolynomialEvaluationFormTrait<F: PrimeField> {
	fn new(evaluations: Vec<F>) -> Self;
	fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> Self;
	fn evaluation(&self, evaluation_points: &[F]) -> F;
	fn generate_variable_names(&self) -> Vec<String>;
	fn zero(num_vars: usize) -> Self;
	fn to_bytes(&self) -> Vec<u8>;
	fn split_poly(&mut self) -> MultiLinearPolynomialEvaluationForm<F> ;
	fn is_zero(&self) -> bool;
}
