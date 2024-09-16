use ark_ff::PrimeField;

use crate::univariate_polynomial::{interface::UnivariantPolynomialInterface, univariant_coefficient::UnivariantPolynomial, univariate::UnivariateMonomial};

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


pub fn lbasis<F:PrimeField>(points: &Vec<F>, y_s:&Vec<F>) -> Vec<UnivariantPolynomial<F>>{
    let mut basis = Vec::new();
    assert!(points.len() == y_s.len(), "Length of points and y_s should be the same");
    for i in 0..points.len(){
        let mut basis_element = UnivariantPolynomial::new(vec![F::one()]);
        for j in 0..points.len(){
            if i == j{
                continue;
            }
            let numerator = UnivariantPolynomial::from_coefficients_vec(vec![-points[j], F::one()]);
            let denominator = points[i] - points[j];
            basis_element = basis_element * (numerator * UnivariantPolynomial::from_coefficients_vec(vec![denominator.inverse().unwrap()]) )
        }
        basis.push(basis_element * UnivariantPolynomial::from_coefficients_vec(vec![y_s[i]]))
    }
    basis
}