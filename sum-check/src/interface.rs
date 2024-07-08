use ark_ff::PrimeField;
use polynomial::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;


#[derive(Clone, Default, Debug)]
pub struct SumCheckProof<F: PrimeField> {
    pub polynomial: MultiLinearPolynomialEvaluationForm<F>,
    pub round_poly: Vec<MultiLinearPolynomialEvaluationForm<F>>,
    pub sum: F,
}
pub trait SumCheckInterface<F:PrimeField>{
     fn calculate_sum(&mut self);
     fn sum_check_proof(&mut self)-> SumCheckProof<F>;
     fn verify(&mut self, proof: &SumCheckProof<F>) -> bool;
}
