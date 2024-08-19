use ark_ff::PrimeField;
use polynomial::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;
use polynomial::composed::multilinear::ComposedMultiLinearPolynomial;

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

pub trait ComposedSumCheckInterface<F:PrimeField>{
    // fn calculate_sum(&mut self)->F;
    fn calculate_sum(poly: &ComposedMultiLinearPolynomial<F>) -> F ;
    fn prover(&self) -> (ComposedSumCheckProof<F>, Vec<F>);
    fn verify(&self, proof: &ComposedSumCheckProof<F>) -> bool;
}

/// This struct is used to store the sum check proof
#[derive(Debug, Clone,PartialEq,Eq,Hash)]
pub struct ComposedSumCheckProof<F: PrimeField> {
    pub polynomial: ComposedMultiLinearPolynomial<F>,
    pub sum: F,
    pub round_poly: Vec<Vec<F>>
   
}