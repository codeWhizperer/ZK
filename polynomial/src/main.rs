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
    // println!("{}", new_polynomial);
    println!("{}", evaluate_result);

// a = [0, 3]
// b = [9, 0]
    let coef_one = { vec![Fq::from(0), Fq::from(3), Fq::from(2)] };
    let coef_two = {  vec![Fq::from(9), Fq::from(0), Fq::from(2), Fq::from(3)] };
    let polynomial_one = UnivariatePolynomial::new(coef_one);
    let polynomial_two = UnivariatePolynomial::new(coef_two);
    let poly_res = polynomial_one + polynomial_two;
    let x_vals = vec![
        Fq::from(0),
        Fq::from(2),
        Fq::from(4),
        // Fq::from(4),
    ];

    let y_vals = vec![
        Fq::from(0),
        Fq::from(4),
        Fq::from(16),
        // Fq::from(8),
    ];
    let _interpolate = UnivariatePolynomial::interpolate(x_vals, y_vals);
    let _res_coeffs = vec![Fq::from(1), Fq::from(3), Fq::from(2)];
    let _res_coeffs2 =  {  vec![Fq::from(5), Fq::from(0), Fq::from(2), Fq::from(4)] };

    let res_vals = vec![Fq::from(3), Fq::from(2), Fq::from(1)];
    let _pp = UnivariatePolynomial::new(res_vals);
    // println!("{}", poly_res);
    // println!("{:?}", res_coeffs);
    // println!("{:?}", res_coeffs2);
    // println!("interpolate {:?}", interpolate);
    let res_coeffs = vec![Fq::from(1), Fq::from(3), Fq::from(2)];
    let res_coeffs2 =  {  vec![Fq::from(5), Fq::from(0), Fq::from(2), Fq::from(4)] };
    let poly_1 = UnivariatePolynomial::new(res_coeffs);
    let poly_2 = UnivariatePolynomial::new(res_coeffs2);
    let result = poly_1 + poly_2;

    println!("{}", result);


    let coeffs1 = vec![Fq::from(1), Fq::from(2), Fq::from(3)];
    let coeffs2 = vec![Fq::from(2), Fq::from(3), Fq::from(4)];
    let poly1 = UnivariatePolynomial::new(coeffs1);
    let poly2 = UnivariatePolynomial::new(coeffs2);
    // let poly_res = poly1 * poly2;
    // let res_coeff = vec![Fq::from(2), Fq::from(6), Fq::from(12)];
//   println!("poly_res {}", UnivariatePolynomial::new(res_coeff));
  println!("multiply {}", poly_res);
//   println!("poly_1 {}",poly1);
  println!("poly_2 {}", poly2 * poly1);

}