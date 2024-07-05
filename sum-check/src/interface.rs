use ark_ff::PrimeField;
use polynomial::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;


/// This struct is used to store the sum check proof
#[derive(Clone, Default, Debug)]
pub struct SumCheckProof<F: PrimeField> {
    pub polynomial: MultiLinearPolynomialEvaluationForm<F>,
    pub round_poly: Vec<MultiLinearPolynomialEvaluationForm<F>>,
    pub round_0_poly: MultiLinearPolynomialEvaluationForm<F>,
    pub sum: F,
}
pub trait ProverInterface<F:PrimeField>{
     fn calculate_sum(&mut self) -> F;
     fn compute_round_zero_poly(&mut self);
     fn sum_check_proof(&mut self)-> SumCheckProof<F>;
}

pub trait VerifierInterface<F: PrimeField> {
     /// This function verifies the sum check proof
     fn verify(&mut self, proof: &SumCheckProof<F>) -> bool;
 }