// given the list [2,3,4] transform to -> (0,2) + (1,3) + (2,4) -> 2 + 3x + 4x;

use ark_ff::PrimeField;
use core::fmt;
#[derive(Debug)]
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


}


