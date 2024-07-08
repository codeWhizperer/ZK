use ark_ff::{PrimeField};
use polynomial::multilinear::{
	evaluation_form::MultiLinearPolynomialEvaluationForm,
	interface::MultiLinearPolynomialEvaluationFormTrait,
};
use crate::interface::SumCheckProof;
use crate::util::convert_field_to_byte;
use transcript::transcription::Transcript;

use crate::interface::SumCheckInterface;
#[derive(Clone, Debug)]
pub struct SumCheck<F: PrimeField> {
	pub polynomial: MultiLinearPolynomialEvaluationForm<F>,
	pub sum: F,
	pub transcript: Transcript,
	pub round_poly: Vec<MultiLinearPolynomialEvaluationForm<F>>,
}

impl<F: PrimeField> SumCheck<F> {
	pub fn new(poly: MultiLinearPolynomialEvaluationForm<F>) -> Self {
		Self {
			polynomial: poly,
			sum: Default::default(),
			round_poly: Default::default(),
			transcript: Default::default(),
		}
	}

	pub fn create_prover_with_sum(poly: MultiLinearPolynomialEvaluationForm<F>, sum: F) -> Self {
		Self {
			polynomial: poly,
			sum,
			transcript: Default::default(),
			round_poly: Default::default(),
		}
	}
}

impl<F: PrimeField> SumCheckInterface<F> for SumCheck<F> {
	fn calculate_sum(&mut self) {
	self.sum =	self.polynomial.evaluations.iter().sum()
	}

	fn sum_check_proof(&mut self) -> SumCheckProof<F> {
        let mut uni_polys = vec![];

        let mut transcript = Transcript::new();
        let poly_sum_bytes = convert_field_to_byte(&self.sum);
        transcript.append(&poly_sum_bytes);

        let mut challenges: Vec<F> = vec![];
        let mut current_poly: MultiLinearPolynomialEvaluationForm<F> = self.polynomial.clone();
        for _ in 0..self.polynomial.number_of_variables {
            let uni_poly = current_poly.split_poly();
            transcript.append(&uni_poly.to_bytes());
            uni_polys.push(uni_poly);
            let random_r: F = transcript.transform_challenge_to_field::<F>();
            challenges.push(random_r);
            current_poly = current_poly.partial_evaluation(random_r, 0);
        
        }


            SumCheckProof {
                polynomial: self.polynomial.clone(),
                sum: self.sum,
                round_poly: uni_polys,
            }
    
	}

    fn verify(&mut self, proof: &SumCheckProof<F>) -> bool {
        let mut transcript = Transcript::new();
        let poly_sum_bytes = convert_field_to_byte(&proof.sum);
        transcript.append(&poly_sum_bytes);

        let mut claimed_sum = proof.sum;
        let mut challenges: Vec<F> = vec![];

        let univariate_poly = &proof.round_poly;
        for i in 0..proof.polynomial.number_of_variables {
            let uni_poly = &univariate_poly[i];
            let eval_p0_p1 =
                uni_poly.evaluation(&vec![F::zero()]) + uni_poly.evaluation(&vec![F::one()]);
            if eval_p0_p1 != claimed_sum {
                return false;
            }
            transcript.append(&uni_poly.to_bytes());
            let challenge: F = transcript.transform_challenge_to_field::<F>();
            challenges.push(challenge);
            claimed_sum = uni_poly.evaluation(&vec![challenge]);
        }
        proof.polynomial.evaluation(challenges.as_slice()) == claimed_sum
    
    }
}

#[cfg(test)]
mod tests {

	use super::*;
	use ark_ff::MontConfig;
	use ark_ff::{Fp64, MontBackend};

	#[derive(MontConfig)]
	#[modulus = "17"]
	#[generator = "3"]
	struct FqConfig;
	type Fq = Fp64<MontBackend<FqConfig, 1>>;
	#[test]
	fn test_calculate_sum() {
		let poly = MultiLinearPolynomialEvaluationForm::new(vec![
			Fq::from(0),
			Fq::from(0),
			Fq::from(0),
			Fq::from(2),
			Fq::from(2),
			Fq::from(2),
			Fq::from(2),
			Fq::from(4),
		]);
		let mut prove = SumCheck::new(poly);
		 prove.calculate_sum();
            assert_eq!(prove.sum, Fq::from(12));
	}

	#[test]
	fn test_calculate_sum2() {
		let poly = MultiLinearPolynomialEvaluationForm::new(vec![
			Fq::from(0),
			Fq::from(0),
			Fq::from(2),
			Fq::from(7),
			Fq::from(3),
			Fq::from(3),
			Fq::from(5),
			Fq::from(11),
		]);
		let mut prove = SumCheck::new(poly);
		 prove.calculate_sum();
		assert_eq!(prove.sum, Fq::from(31))
	}

    #[test]
    fn test_sum_check_proof() {
        let poly = MultiLinearPolynomialEvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(7),
            Fq::from(3),
            Fq::from(3),
            Fq::from(6),
            Fq::from(11),
        ]);
        let mut prove = SumCheck::new(poly);
        prove.calculate_sum();
        let proof = prove.sum_check_proof();
        let verifer: bool = prove.verify(&proof);

        assert_eq!(verifer, true);
    }





}