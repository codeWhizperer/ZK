// given the list [2,3,4] transform to -> (0,2) + (1,3) + (2,4) -> 2 + 3x + 4x;
use ark_ff::PrimeField;
use std::ops::{Add,Mul};
use core::fmt;
#[derive(Debug,Clone,PartialEq)]
pub struct UnivariatePolynomial<F:PrimeField> {
    coef: Vec<F>,
}




impl<F:PrimeField> fmt::Display for UnivariatePolynomial<F> {
 fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, &c) in self.coef.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", c)?;
            } else {
                write!(f, " + {}x^{}", c, i)?;
            }
        }
        Ok(())
    }
}

impl<F:PrimeField> UnivariatePolynomial<F> {
    pub fn new(coef: Vec<F>) -> Self {
        Self { coef }
    }
    

   pub fn evaluate(&self, x: &F) -> F {
        self.coef.iter().rev().fold(F::zero(), |acc, cof| acc * x + cof)
    }

    
    pub fn interpolate(x_values: Vec<F>, y_values: Vec<F>) -> Self {
        assert_eq!(x_values.len(), y_values.len());
        let mut result = Self::new(vec![]);
        for i in 0..x_values.len() {
            let mut y_poly_value = Self::new(vec![y_values[i]]);
            for j in 0..x_values.len() {
                if x_values[i] == x_values[j] {
                    continue;
                };
                let num = Self::new(vec![-x_values[j], F::one()]);
                let mut denom = Self::new(vec![F::one()]);
                if let Some(val) = (x_values[i] - x_values[j]).inverse() {
                    denom = UnivariatePolynomial::new(vec![val]);
                } else {
                    panic!("Invalid inverse");
                }
                y_poly_value = y_poly_value * num * denom;
            }
            result = result + y_poly_value;
        }
        result
    }
    
    

}

// a = [0, 3, 2]
// b = [9, 0, 2, 3]

impl<F:PrimeField> Mul for UnivariatePolynomial<F>
where
    F: Copy + Mul<Output = F> + Default,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {

        let mut result_coefficients = vec![F::default(); self.coef.len() + other.coef.len() - 1];
        for (i, &coeff1) in self.coef.iter().enumerate() {
            for (j, &coeff2) in other.coef.iter().enumerate() {
                result_coefficients[i + j] = result_coefficients[i + j] + coeff1 * coeff2;
            }
        }
        Self {
            coef: result_coefficients,
        }
    }
}

impl<F:PrimeField> Add for UnivariatePolynomial<F>
where
    F: Copy + Add<Output = F> + Default,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let max_degree = self.coef.len().max(other.coef.len());
        let mut result_coefficients = vec![F::default(); max_degree];
        for i in 0..max_degree {
            let coeff1 = if i < self.coef.len() { self.coef[i] } else { F::default() };
            let coeff2 = if i < other.coef.len() { other.coef[i] } else { F::default() };
            result_coefficients[i] = coeff1 + coeff2;
        }
        Self {
            coef: result_coefficients,
        }
    }
}


// impl<F:PrimeField> UnivariatePolynomial<F>
// where
//     F: Copy + Default + Div<Output = F> + PartialEq + Mul<Output = F>,
// {
//     fn reciprocal(&self) -> Self {
//         let mut result = self.clone();
//         for coeff in &mut result.coef {
//             *coeff = F::one() / *coeff;
//         }
//         result
//     }
// }

#[cfg(test)]
mod test{
    use ark_ff::{Fp64, MontBackend, MontConfig};
    use super::UnivariatePolynomial;
    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

    #[test]
    fn test_add_poly() {
        let coeffs1 = vec![Fq::from(1), Fq::from(2), Fq::from(3)];
        let coeffs2 = vec![Fq::from(2), Fq::from(3), Fq::from(4)];
        let poly1 = UnivariatePolynomial::new(coeffs1);
        let poly2 = UnivariatePolynomial::new(coeffs2);
        let poly_res = poly1 + poly2;
        let res_coeffs = vec![Fq::from(3), Fq::from(5), Fq::from(7)];
        assert_eq!(poly_res, UnivariatePolynomial::new(res_coeffs));
    }

    #[test]
    fn test_mul_poly() {
        let coeffs1 = vec![Fq::from(1), Fq::from(2), Fq::from(3)];
        let coeffs2 = vec![Fq::from(2), Fq::from(3), Fq::from(4)];
        let poly1 = UnivariatePolynomial::new(coeffs1);
        let poly2 = UnivariatePolynomial::new(coeffs2);
        let poly_res = poly1 * poly2;
        let res_coeffs = vec![Fq::from(2), Fq::from(7),Fq::from(16),Fq::from(0), Fq::from(12)];
        assert_eq!(poly_res, UnivariatePolynomial::new(res_coeffs));
    }

    #[test]
    fn test_interpolation_simple() {
        let x_values = vec![Fq::from(0), Fq::from(1)];
        let y_values = vec![Fq::from(0), Fq::from(1)];

        let interpolated_poly = UnivariatePolynomial::interpolate(x_values, y_values);
    }
    
}

