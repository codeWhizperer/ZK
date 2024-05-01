// given the list [2,3,4] transform to -> (0,2) + (1,3) + (2,4) -> 2 + 3x + 4x;

use ark_ff::PrimeField;
use core::fmt;
#[derive(Debug)]
struct UnivariatePolynomial<F:PrimeField> {
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
    

    fn evaluate(&self, x: &F) -> F {
        self.coef.iter().rev().fold(F::zero(), |acc, cof| acc * x + cof)
    }


}




fn main() {
    use ark_ff::{Fp64, MontBackend, MontConfig};

    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;
    let coeffs = vec![Fq::from(1), Fq::from(2), Fq::from(5)];
    let new_polynomial = UnivariatePolynomial::new(coeffs);
    let evaluate_result = UnivariatePolynomial::evaluate(&new_polynomial,&Fq::from(2));
    println!("{}", new_polynomial);
    println!("{}", evaluate_result);


}
