use ark_ff::PrimeField;
use polynomial::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;
use sum_check::multi_composed_sumcheck::ComposedSumcheckProof;
#[derive(Debug, Clone,PartialEq)]
pub enum GateType {
	Add,
	Mul,
}
#[derive(Debug, Clone)]
pub struct Gate {
	pub gate_type: GateType,
	pub inputs: [usize; 2],
}
#[derive(Debug, Clone)]
pub struct CircuitLayer {
	pub layer: Vec<Gate>,
}
#[derive(Debug,Clone)]
pub struct Circuit {
	pub layers: Vec<CircuitLayer>,
}
#[derive(Debug,Clone)]
pub struct CircuitEvaluation<F: PrimeField> {
	pub layers: Vec<Vec<F>>,
}
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct GKRProof<F: PrimeField> {
 pub   sumcheck_proofs: Vec<ComposedSumcheckProof<F>>,
  pub  wb_s: Vec<F>,    // w_mle for layer one onward for rb
  pub  wc_s: Vec<F>,    // w_mle for layer one onward for rc
  pub  w_0_mle: MultiLinearPolynomialEvaluationForm<F>, // w_mle for layer
}
