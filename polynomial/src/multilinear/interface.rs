use ark_ff::PrimeField;
pub trait MultiLinearPolynomialEvaluationFormTrait<F: PrimeField> {
    fn new(evaluations: Vec<F>) -> Self;
    fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> Self;
    fn full_evaluation(&self, evaluation_points: &[F]) -> F;
}