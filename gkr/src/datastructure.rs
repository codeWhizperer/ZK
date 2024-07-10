use ark_ff::PrimeField;
#[derive(Debug, Clone)]
pub enum GateType {
	Add,
	Mul,
}
#[derive(Debug, Clone)]
pub struct Gate {
	pub gate_type: GateType,
	pub inputs: [usize; 2],
}

pub struct CircuitLayer {
	pub layer: Vec<Gate>,
}

pub struct Circuit {
	pub layers: Vec<CircuitLayer>,
	pub number_of_inputs: usize,
}
#[derive(Debug)]
pub struct CircuitEvaluation<F: PrimeField> {
	pub layers: Vec<Vec<F>>,
}
