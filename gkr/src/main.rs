use gkr::{
	datastructure::{Circuit, CircuitLayer, Gate, GateType},
};

fn main() {
	use ark_ff::MontConfig;
	use ark_ff::{Fp64, MontBackend};
	#[derive(MontConfig)]
	#[modulus = "17"]
	#[generator = "3"]
	struct FqConfig;
	type Fq = Fp64<MontBackend<FqConfig, 1>>;
	let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1])]);
	let layer_1 =
		CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1]), Gate::new(GateType::Mul, [2, 3])]);
	let circuit = Circuit::new(vec![layer_0, layer_1]);
	// let (add_i_mle, mul_i_mle) = circuit.add_i_mul_ext::<Fq>(0);
	let (add_i_mle_1, mul_i_mle_1) = circuit.add_i_mul_ext::<Fq>(1);
	// println!("circuit_output ----> {:?}",circuit);
	// println!("add_i_mle_output ----> {:?}   mul_i_mle -----> {:?}", add_i_mle, mul_i_mle);
	println!("add_i_mle_output ----> {:?}   mul_i_mle -----> {:?}", add_i_mle_1, mul_i_mle_1);
}
