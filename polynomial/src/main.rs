use ark_ff::{Fp64, MontBackend, MontConfig};
use polynomial::multilinear::{evaluation_form::MultiLinearPolynomialEvaluationForm, interface::MultiLinearPolynomialEvaluationFormTrait};
fn main() {
 
    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;
    let evaluations = vec![
        Fq::from(3),
        Fq::from(9),
        Fq::from(7),
        Fq::from(13),
        Fq::from(6),
        Fq::from(12),
        Fq::from(10),
        Fq::from(18),
    ];
    
let new_poly = MultiLinearPolynomialEvaluationForm::new(evaluations);

println!("new polynomial: {:?}", new_poly);
}
