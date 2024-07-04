use ark_ff::PrimeField;


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