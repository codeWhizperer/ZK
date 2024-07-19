use crate::datastructure::{Circuit, CircuitEvaluation, CircuitLayer, Gate, GateType};
use crate::utils::{label_to_binary_to_decimal, size_of_number_of_variable_at_each_layer};
use ark_ff::PrimeField;
use polynomial::multilinear::{
	evaluation_form::MultiLinearPolynomialEvaluationForm,
	interface::MultiLinearPolynomialEvaluationFormTrait,
};
use std::ops::{Add, Mul};
impl Circuit {
	pub fn new(layers: Vec<CircuitLayer>) -> Self {
		Self { layers }
	}

	pub fn evaluate<F: PrimeField>(&self, input: &[F]) -> CircuitEvaluation<F>
	where
		F: Add<Output = F> + Mul<Output = F> + Copy,
	{
		let mut layers = vec![];
		let mut current_input = input;

		layers.push(input.to_vec());

		for layer in self.layers.iter().rev() {
			let temp_layer: Vec<_> = layer
				.layer
				.iter()
				.map(|e| match e.gate_type {
					GateType::Add => current_input[e.inputs[0]] + current_input[e.inputs[1]],
					GateType::Mul => current_input[e.inputs[0]] * current_input[e.inputs[1]],
				})
				.collect();

			layers.push(temp_layer);
			current_input = &layers[layers.len() - 1];
		}

		layers.reverse();
		CircuitEvaluation { layers }
	}

	pub fn add_i(&self, layer_index: usize, gate_index: usize, b: usize, c: usize) -> bool {
		let gate = &self.layers[layer_index].layer[gate_index];
		gate.gate_type == GateType::Add && gate.inputs[0] == b && gate.inputs[1] == c
	}

	pub fn mul_i(&self, layer_index: usize, gate_index: usize, b: usize, c: usize) -> bool {
		let gate = &self.layers[layer_index].layer[gate_index];
		gate.gate_type == GateType::Mul && gate.inputs[0] == b && gate.inputs[1] == c
	}

	pub fn add_i_mul_ext<F: PrimeField>(
		&self,
		layer_index: usize,
	) -> (MultiLinearPolynomialEvaluationForm<F>, MultiLinearPolynomialEvaluationForm<F>) {
		let layers = &self.layers[layer_index];
		let number_of_variables = Circuit::size_of_number_of_variable_at_each_layer(layer_index); // get number of variables at each layer

		let mut add_i_evaluations = vec![F::zero(); number_of_variables];
		let mut mul_i_evaluations = vec![F::zero(); number_of_variables];

		for (gate_index, gate) in layers.layer.iter().enumerate() {
			match gate.gate_type {
				GateType::Add => {
					let gate_decimal =
						label_to_binary_to_decimal(gate_index, gate.inputs[0], gate.inputs[1]);
					add_i_evaluations[gate_decimal] = F::one();
				},
				GateType::Mul => {
					let gate_decimal =
						label_to_binary_to_decimal(gate_index, gate.inputs[0], gate.inputs[1]);
					mul_i_evaluations[gate_decimal] = F::one();
				},
			}
		}
		let add_i_mle = MultiLinearPolynomialEvaluationForm::new(add_i_evaluations);
		let mul_i_mle = MultiLinearPolynomialEvaluationForm::new(mul_i_evaluations);
		(add_i_mle, mul_i_mle)
	}
	pub fn size_of_number_of_variable_at_each_layer(layer_index: usize) -> usize {
		if layer_index == 0 {
			return 1 << 3;
		}
		let layer_index_plus_one = layer_index + 1;
		let number_of_variable = layer_index + (2 * layer_index_plus_one);
		1 << number_of_variable
	}
}

impl CircuitLayer {
	pub fn new(layer: Vec<Gate>) -> Self {
		CircuitLayer { layer }
	}
}

impl Gate {
	pub fn new(gate_type: GateType, inputs: [usize; 2]) -> Self {
		Self { gate_type, inputs }
	}
}

pub fn circuit_layers() -> Circuit {
	Circuit {
		layers: vec![
			CircuitLayer {
				layer: vec![
					Gate { gate_type: GateType::Mul, inputs: [0, 1] },
					Gate { gate_type: GateType::Mul, inputs: [2, 3] },
				],
			},
			CircuitLayer {
				layer: vec![
					Gate { gate_type: GateType::Mul, inputs: [0, 0] },
					Gate { gate_type: GateType::Mul, inputs: [1, 1] },
					Gate { gate_type: GateType::Mul, inputs: [1, 2] },
					Gate { gate_type: GateType::Mul, inputs: [3, 3] },
				],
			},
		],
	}
}

#[cfg(test)]
pub(crate) fn circuit_template() -> Circuit {
	Circuit {
		layers: vec![
			CircuitLayer {
				layer: vec![
					Gate { gate_type: GateType::Mul, inputs: [0, 1] },
					Gate { gate_type: GateType::Mul, inputs: [2, 3] },
				],
			},
			CircuitLayer {
				layer: vec![
					Gate { gate_type: GateType::Mul, inputs: [0, 0] },
					Gate { gate_type: GateType::Mul, inputs: [1, 1] },
					Gate { gate_type: GateType::Mul, inputs: [1, 2] },
					Gate { gate_type: GateType::Mul, inputs: [3, 3] },
				],
			},
		],
	}
}

mod tests {
	use super::*;
	use ark_ff::MontConfig;
	use ark_ff::{Fp64, MontBackend};

	#[derive(MontConfig)]
	#[modulus = "17"]
	#[generator = "3"]
	struct FqConfig;
	type Fq = Fp64<MontBackend<FqConfig, 1>>;
	#[test]
	fn circuit_test() {
		let circuit = circuit_template();

		let layers = circuit.evaluate(&[Fq::from(3), Fq::from(2), Fq::from(3), Fq::from(1)]);
		assert_eq!(
			layers.layers,
			vec![
				vec![Fq::from(36), Fq::from(6)],
				vec![Fq::from(9), Fq::from(4), Fq::from(6), Fq::from(1)],
				vec![Fq::from(3), Fq::from(2), Fq::from(3), Fq::from(1)]
			]
		);
	}
	#[test]
	fn test_circuit_evaluation() {
		let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1])]);
		let layer_1 = CircuitLayer::new(vec![
			Gate::new(GateType::Add, [0, 1]),
			Gate::new(GateType::Mul, [2, 3]),
		]);
		let circuit = Circuit::new(vec![layer_0, layer_1]);
		let input = [Fq::from(1u32), Fq::from(2u32), Fq::from(3u32), Fq::from(4u32)];
		let evaluation = circuit.evaluate(&input);
		let expected_output = vec![
			vec![Fq::from(15u32)],
			vec![Fq::from(3u32), Fq::from(12u32)],
			vec![Fq::from(1u32), Fq::from(2u32), Fq::from(3u32), Fq::from(4u32)],
		];

		assert_eq!(evaluation.layers, expected_output);
	}

	#[test]
	fn test_label_binary_and_to_decimal() {
		assert_eq!(label_to_binary_to_decimal(0, 0, 1), 1);
		assert_eq!(label_to_binary_to_decimal(1, 2, 3), 27);
	}

	#[test]
	fn test_add_i_mul_i_ext_at_layer_0() {
		let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1])]);
		let layer_1 = CircuitLayer::new(vec![
			Gate::new(GateType::Add, [0, 1]),
			Gate::new(GateType::Mul, [2, 3]),
		]);

		let gkr_circuit = Circuit::new(vec![layer_0, layer_1]);
		let (add_i_mle, mul_i_mle) = gkr_circuit.add_i_mul_ext::<Fq>(0);
		assert_eq!(mul_i_mle.is_zero(), true);
		assert_eq!(add_i_mle.is_zero(), false);
		assert_eq!(add_i_mle.number_of_variables, 3);
		assert_eq!(mul_i_mle.number_of_variables, 3);
	}

	#[test]
	fn test_add_i_mul_i_ext_at_layer_1() {
		let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1])]);
		let layer_1 = CircuitLayer::new(vec![
			Gate::new(GateType::Add, [0, 1]),
			Gate::new(GateType::Mul, [2, 3]),
		]);

		let gkr_circuit = Circuit::new(vec![layer_0, layer_1]);
		let (add_i_mle, mul_i_mle) = gkr_circuit.add_i_mul_ext::<Fq>(1);
		assert_eq!(mul_i_mle.is_zero(), false);
		assert_eq!(add_i_mle.is_zero(), false);
	}
}
