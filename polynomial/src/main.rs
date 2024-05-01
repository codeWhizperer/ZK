use ark_ff::{Fp64, MontBackend, MontConfig};
use polynomial::univariate_polynomial::UnivariatePolynomial;
fn main() {
 
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