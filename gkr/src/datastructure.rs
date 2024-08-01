use ark_ff::PrimeField;
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
