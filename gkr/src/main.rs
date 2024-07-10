use gkr::circuit::circuit_layers;
use ark_ff::PrimeField;

fn main() {
    use ark_ff::MontConfig;
	use ark_ff::{Fp64, MontBackend};

	#[derive(MontConfig)]
	#[modulus = "17"]
	#[generator = "3"]
	struct FqConfig;
	type Fq = Fp64<MontBackend<FqConfig, 1>>;


let circuit = circuit_layers();

let layers = circuit.evaluate(&[Fq::from(3), Fq::from(2), Fq::from(3), Fq::from(1)]);
println!("layers: {:?}", layers.layers);
println!("layers: {:?}", layers);
}
