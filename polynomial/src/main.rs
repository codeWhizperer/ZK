use ark_ff::{Fp64, MontBackend, MontConfig};
use polynomial::univariate_polynomial::UnivariatePolynomial;
fn main() {
 
    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;
    let coeffs = vec![Fq::from(1), Fq::from(16), Fq::from(13)];
    let new_polynomial = UnivariatePolynomial::new(coeffs);
    let evaluate_result = UnivariatePolynomial::evaluate(&new_polynomial,&Fq::from(2));
    println!("{}", new_polynomial);
    println!("{}", evaluate_result);

// a = [0, 3, 2]
// b = [9, 0, 2, 3]
    let coef_one = { vec![Fq::from(0), Fq::from(3), Fq::from(2)] };
    let coef_two = {  vec![Fq::from(9), Fq::from(0), Fq::from(2), Fq::from(3)] };
    let polynomial_one = UnivariatePolynomial::new(coef_one);
    let polynomial_two = UnivariatePolynomial::new(coef_two);
    let poly_res = polynomial_one + polynomial_two;
    let res_coeffs = vec![Fq::from(1), Fq::from(3), Fq::from(2)];
    let res_coeffs2 =  {  vec![Fq::from(5), Fq::from(0), Fq::from(2), Fq::from(4)] };
    println!("{}", poly_res);
    println!("{:?}", res_coeffs);
    println!("{:?}", res_coeffs2);

}