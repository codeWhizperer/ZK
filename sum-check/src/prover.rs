use crate::{interface::ProverInterface, util::boolean_hypercube};
use crate::verifier::{Verifier};
use ark_ff::PrimeField;
use polynomial::multilinear::{
	evaluation_form::MultiLinearPolynomialEvaluationForm,
	interface::MultiLinearPolynomialEvaluationFormTrait,
};
use crate::interface::SumCheckProof;
use transcript::transcription::Transcript;

#[derive(Clone, Debug)]
pub struct Prover<F: PrimeField> {
	pub polynomial: MultiLinearPolynomialEvaluationForm<F>,
	pub sum: F,
	pub round_0_poly: MultiLinearPolynomialEvaluationForm<F>,
    pub transcript: Transcript,
    pub round_poly: Vec<MultiLinearPolynomialEvaluationForm<F>>
}

impl<F: PrimeField> Prover<F> {
	pub fn new(poly: MultiLinearPolynomialEvaluationForm<F>) -> Self {
		Self { polynomial: poly, sum: Default::default(), round_poly:Default::default(),round_0_poly: Default::default(), transcript:Default::default() }
	}

	// create new prover with sum (is this needed?)
	pub fn create_prover_with_sum(poly: MultiLinearPolynomialEvaluationForm<F>, sum: F) -> Self {
		Self { polynomial: poly, sum, round_0_poly: Default::default(), transcript:Default::default(), round_poly:Default::default() }
	}
}

impl<F: PrimeField> ProverInterface<F> for Prover<F> {
	fn calculate_sum(&mut self) -> F {
		self.polynomial.evaluations.iter().sum()
	}
	fn compute_round_zero_poly(&mut self) {
		let num_of_rounds = self.polynomial.number_of_variables - 1;
		let bh = boolean_hypercube::<F>(num_of_rounds);
		let mut bh_partials: MultiLinearPolynomialEvaluationForm<F> =
			MultiLinearPolynomialEvaluationForm::zero(1);
		for bh_i in bh {
            let mut current_poly = self.polynomial.clone();
			for bh_ii in bh_i {
				current_poly = current_poly.partial_evaluation(bh_ii, 1);
			}
            bh_partials += current_poly;
		}
        self.round_0_poly = bh_partials;
	}

    
    // fn sum_check_proof(&mut self) -> SumCheckProof<F> {
    //     self.compute_round_zero_poly();
    //     let mut all_random_response = Vec::new();
    //     for i in 1..self.polynomial.number_of_variables {
    //         let num_of_rounds = self.polynomial.number_of_variables - 1 - 1;
    //         let bh = boolean_hypercube::<F>(num_of_rounds);
    //         let mut bh_partials = MultiLinearPolynomialEvaluationForm::zero(1);
    //         let verifier_random_response = F::from_be_bytes_mod_order(&self.transcript.sample_challenge());
    //         all_random_response.push(verifier_random_response);
    
    //         for bh_i in bh {
    //             // let bh_len = bh_i.len();
    //             let mut eval_vector = all_random_response.clone();
    //             eval_vector.extend(bh_i.clone());
    //             let  eval_index = vec![0; all_random_response.len()];
    //             let mut current_poly = self.polynomial.clone();
    //             for b in bh_i.clone() {
    //                 current_poly = current_poly.partial_evaluation(b, eval_index[1]);
    //             }
    //             bh_partials += current_poly;
    //         }
    //         self.transcript.append(&bh_partials.to_bytes());
    //         self.round_poly.push(bh_partials);
    //     }
    
    //     SumCheckProof {
    //         polynomial: self.polynomial.clone(),
    //         round_poly: self.round_poly.clone(),
    //         round_0_poly: self.round_0_poly.clone(),
    //         sum: self.sum,
    //     }
    // }


    fn sum_check_proof(&mut self) -> SumCheckProof<F> {
    //    let sum = self.calculate_sum();
        self.compute_round_zero_poly();
        let mut all_random_response = Vec::new();
        for i in 1..self.polynomial.number_of_variables {
            let num_of_rounds = self.polynomial.number_of_variables - i - 1;
            let bh = boolean_hypercube::<F>(num_of_rounds);
            let mut bh_partials = MultiLinearPolynomialEvaluationForm::zero(1);
            let verifier_random_response = F::from_be_bytes_mod_order(&self.transcript.sample_challenge());
            all_random_response.push(verifier_random_response);
    
            for bh_i in bh {
                let mut eval_vector = all_random_response.clone();
                eval_vector.extend(bh_i.clone());
                let mut current_poly = self.polynomial.clone();
                for (index, &b) in bh_i.iter().enumerate() {
                    current_poly = current_poly.partial_evaluation(b, index);
                }
                bh_partials += current_poly;
            }
            self.transcript.append(&bh_partials.to_bytes());
            self.round_poly.push(bh_partials);
        }
        SumCheckProof {
            polynomial: self.polynomial.clone(),
            round_poly: self.round_poly.clone(),
            round_0_poly: self.round_0_poly.clone(),
            sum: self.sum,
        }
    }
    
}


#[cfg(test)]
mod tests{
use crate::interface::VerifierInterface;

use super::*;
use ark_ff::MontConfig;
use ark_ff::{Fp64, MontBackend};

#[derive(MontConfig)]
#[modulus = "17"]
#[generator = "3"]
struct FqConfig;
type Fq = Fp64<MontBackend<FqConfig, 1>>;
#[test]
fn test_calculate_sum(){
    let poly = MultiLinearPolynomialEvaluationForm::new(
        vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(4),
        ]
    );
    let mut poly = Prover::new(poly);
    let  sum = Prover::calculate_sum(&mut poly);
    assert_eq!(sum, Fq::from(12));
}

#[test]
fn test_calculate_sum2(){
    let poly = MultiLinearPolynomialEvaluationForm::new(
            vec![
                Fq::from(0),
                Fq::from(0),
                Fq::from(2),
                Fq::from(7),
                Fq::from(3),
                Fq::from(3),
                Fq::from(5),
                Fq::from(11),
            ],
    );
    let mut poly = Prover::new(poly);
    let sum = Prover::calculate_sum(&mut poly);
    assert_eq!(sum,Fq::from(31))
}

#[test]
fn test_compute(){
    let poly = MultiLinearPolynomialEvaluationForm::new(
        vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(4),
        ]
    );
    
    let mut poly = Prover::new(poly);
     Prover::compute_round_zero_poly(&mut poly);
    assert_eq!(
        poly.round_0_poly.evaluations,
        vec![Fq::from(2), Fq::from(10)]
    );
}

#[test]
fn test_compute_round_zero_poly_2() {
    let poly = MultiLinearPolynomialEvaluationForm::new(
        vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(7),
            Fq::from(3),
            Fq::from(3),
            Fq::from(5),
            Fq::from(11),
        ],
    );
    let mut prover = Prover::new(poly);
    prover.compute_round_zero_poly();
    let sum = prover.round_0_poly.evaluate(&vec![Fq::from(1)]).unwrap()
        + prover.round_0_poly.evaluate(&vec![Fq::from(0)]).unwrap();
    assert_eq!(sum, Fq::from(31));
}

#[test]
fn test_compute_round_zero_poly() {
    let poly = MultiLinearPolynomialEvaluationForm::new(
        vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(4),
        ]
    );
    let mut prover = Prover::new(poly);
    prover.compute_round_zero_poly();
    assert_eq!(
        prover.round_0_poly.evaluations,
        vec![Fq::from(2), Fq::from(10)]
    );
}


#[test]
fn test_sum_check_proof_sum(){
    let poly = MultiLinearPolynomialEvaluationForm::new(
        vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(4),
        ]
    );


    let mut prover = Prover::new(poly);
    prover.compute_round_zero_poly();
let sum =  prover.round_0_poly.evaluate(&vec![Fq::from(1)]).unwrap() + prover.round_0_poly.evaluate(&vec![Fq::from(0)]).unwrap();
assert_eq!(sum, Fq::from(12))
}


// #[test]
// fn test_sum_check_proof() {
//     let poly = MultiLinearPolynomialEvaluationForm::new(
//         vec![
//             Fq::from(0),
//             Fq::from(0),
//             Fq::from(0),
//             Fq::from(2),
//             Fq::from(2),
//             Fq::from(2),
//             Fq::from(2),
//             Fq::from(4),
//         ]
//     );
//     let mut prover = Prover::new(poly);
//     prover.calculate_sum();
//     let proof = prover.sum_check_proof();
//     let mut verifer = Verifier::new();
//     assert!(verifer.verify(&proof));
// }
// }
}