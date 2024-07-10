use crate::datastructure::{Circuit, CircuitEvaluation, CircuitLayer, GateType,Gate};
use ark_ff::PrimeField;
use std::ops::{Add, Mul};

impl Circuit {
	pub fn new(layers: Vec<CircuitLayer>, number_of_inputs: usize) -> Self {
		Self { layers, number_of_inputs }
	}
	pub fn evaluate<F:PrimeField>(&self, input: &[F]) -> CircuitEvaluation<F>
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
}



pub fn circuit_layers() -> Circuit {
    Circuit {
        layers: vec![
            CircuitLayer {
                layer: vec![
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [0, 1],
                    },
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [2, 3],
                    },
                ],
            },
            CircuitLayer {
                layer: vec![
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [0, 0],
                    },
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [1, 1],
                    },
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [1, 2],
                    },
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [3, 3],
                    },
                ],
            },
        ],
        number_of_inputs: 4,
    }
}

#[cfg(test)]
pub(crate) fn circuit_template() -> Circuit {
    Circuit {
        layers: vec![
            CircuitLayer {
                layer: vec![
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [0, 1],
                    },
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [2, 3],
                    },
                ],
            },
            CircuitLayer {
                layer: vec![
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [0, 0],
                    },
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [1, 1],
                    },
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [1, 2],
                    },
                    Gate {
                        gate_type: GateType::Mul,
                        inputs: [3, 3],
                    },
                ],
            },
        ],
        number_of_inputs: 4,
    }
}

mod tests{
    use super::*;
    use ark_ff::MontConfig;
	use ark_ff::{Fp64, MontBackend};
    use crate::datastructure::{CircuitLayer,Gate};

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
            vec![vec![Fq::from(36), Fq::from(6)], vec![Fq::from(9), Fq::from(4), Fq::from(6), Fq::from(1)], vec![Fq::from(3), Fq::from(2), Fq::from(3), Fq::from(1)]]
        );
    }
}