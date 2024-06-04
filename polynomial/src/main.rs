use ark_ff::{Fp64, MontBackend, MontConfig};
use polynomial::{MultilinearMonomial,MultilinearPolynomial};
fn main() {
 
    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;
    let monomial1 =  MultilinearMonomial{coefficient:Fq::from(3), variables:vec![false, false]}; 
    let monomial2 =  MultilinearMonomial{coefficient:Fq::from(1), variables:vec![true, false]}; 
    let monomial3 =  MultilinearMonomial{coefficient:Fq::from(2), variables:vec![false, true]}; 
    let monomial4 =  MultilinearMonomial{coefficient:Fq::from(5), variables:vec![true, true]}; 
    let polynomial = MultilinearPolynomial::new(vec![monomial1,monomial2, monomial3, monomial4]);
   
    let r1 =  MultilinearMonomial{coefficient:Fq::from(0), variables:vec![false]}; 
    let r2 =  MultilinearMonomial{coefficient:Fq::from(13), variables:vec![false]}; 
    let r_polynomial = MultilinearPolynomial::new(vec![r1,r2]);
    let eval_points = vec![Some(Fq::from(3_u8)), None, None];
    let p = MultilinearPolynomial::partial_eval(&polynomial, &eval_points);
    println!("polynomial: {}", polynomial);
    println!("res_polynomial: {}", r_polynomial);
    println!("p_eval: {}", p)


}
// 3 + 1(3) + 2(3) + 5(3) a