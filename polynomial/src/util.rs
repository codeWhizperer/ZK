use ark_ff::PrimeField;

pub fn lagrange_basis<F: PrimeField>(points: &[(F, F)], i: usize) -> Vec<F> {
    let mut l_i = vec![F::one()];

    for (j, &(x_j, _)) in points.iter().enumerate() {
        if i != j {
            let mut new_l_i = vec![F::zero(); l_i.len() + 1];
            for (k, &coeff) in l_i.iter().enumerate() {
                new_l_i[k] -= coeff * x_j;
                new_l_i[k + 1] += coeff;
            }
            l_i = new_l_i;
        }
    }

    let denom = points
        .iter()
        .enumerate()
        .filter(|&(j, _)| j != i)
        .fold(F::one(), |acc, (_, &(x_j, _))| acc * (points[i].0 - x_j));
    l_i.into_iter()
        .map(|coeff| coeff * denom.inverse().unwrap())
        .collect()
}