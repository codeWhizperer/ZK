use ark_ff::{PrimeField,BigInteger};


pub fn boolean_hypercube<F: PrimeField>(n:usize) -> Vec<Vec<F>>{
    let mut result = Vec::new();
    for i in 0..1u128 << n{
        let mut current = Vec::new();
        for j in 0..n{
            if (i >> j) & 1 == 1{
                current.push(F::one());
            }else{
                current.push(F::zero());
            }
        }
        current.reverse();
        result.push(current)
    }
    result
}

pub fn convert_field_to_byte<F: PrimeField>(element: &F) -> Vec<u8> {
    element.into_bigint().to_bytes_be()
}

pub fn transform_round_poly_to_uni_poly<F: PrimeField>(round_poly: &Vec<F>) -> Vec<(F, F)> {
    round_poly
        .iter()
        .enumerate()
        .map(|(i, val)| (F::from(i as u64), *val))
        .collect()
}

pub fn vec_to_bytes<F: PrimeField>(poly: &Vec<F>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for p in poly {
        bytes.extend_from_slice(&p.into_bigint().to_bytes_be());
    }
    bytes
}
