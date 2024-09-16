// use crate::interface::ComposedSumCheckProof;
use crate::util::transform_round_poly_to_uni_poly;
use ark_ff::{BigInteger, PrimeField};
use polynomial::composed::interface::ComposedMultilinearInterface;
use polynomial::composed::multilinear::ComposedMultiLinearPolynomial;
use polynomial::interface::UnivariatePolynomialTrait;
use polynomial::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;
use polynomial::multilinear::interface::MultiLinearPolynomialEvaluationFormTrait;
use polynomial::univariate_polynomial::univariate::UnivariatePolynomial;
use transcript::transcription::Transcript;

use crate::interface::ComposedSumCheckInterface;
#[derive(Debug, Clone)]
pub struct ComposedSumcheck<F: PrimeField> {
    pub polynomial: ComposedMultiLinearPolynomial<F>,
    pub sum: F,
}

pub struct ComposedSumcheckProof<F: PrimeField> {
    polynomial: ComposedMultiLinearPolynomial<F>,
    round_polys: Vec<Vec<F>>,
}

impl<F: PrimeField> ComposedSumcheck<F> {
	pub fn new(poly: ComposedMultiLinearPolynomial<F>) -> Self {
		Self {
			polynomial: poly,
			sum: Default::default(),
		}
	}
	pub fn calculate_sum(poly: &ComposedMultiLinearPolynomial<F>) -> F {
        poly.elementwise_product().iter().sum()
    }
	pub fn prove(&self) -> (ComposedSumcheckProof<F>, Vec<F>) {
        let mut transcript = Transcript::new();

        let mut current_poly: ComposedMultiLinearPolynomial<F> = self.polynomial.clone();
        let mut round_polys: Vec<Vec<F>> = vec![];
        let mut challenges: Vec<F> = vec![];

        for _ in 0..self.polynomial.number_of_variables() {
            let mut round_poly: Vec<F> = vec![];
            for i in 0..=current_poly.max_degree() {
                let round: F = current_poly
                    .partial_evaluation(F::from(i as u32), 0)
                   .elementwise_product()
                    .iter()
                    .sum::<F>();

                round_poly.push(round);
            }

            transcript.append(&vec_to_bytes(&round_poly));
            //get the random r
            let random_r: F = transcript.transform_challenge_to_field::<F>();
            challenges.push(random_r);
            round_polys.push(round_poly);

            current_poly = current_poly.partial_evaluation(random_r, 0);
        }

        (
            ComposedSumcheckProof {
                polynomial: self.polynomial.clone(),
                round_polys,
            },
            challenges,
        )
    }

	pub fn verify(&self, proof: &ComposedSumcheckProof<F>, sum: F) -> bool {
        let mut transcript = Transcript::new();

        let mut claimed_sum = sum;
        let mut challenges: Vec<F> = vec![];

        for round_poly in proof.round_polys.iter() {
            transcript.append(&vec_to_bytes(&round_poly));
            // genrate the challenge for this round
            let challenge: F = transcript.transform_challenge_to_field::<F>();
            challenges.push(challenge);

            let round_polys_uni: Vec<(F, F)> = transform_round_poly_to_uni_poly(&round_poly);
            let uni_poly: UnivariatePolynomial<F> =
                UnivariatePolynomial::interpolate(&round_polys_uni);

            let eval_p0_p1 = uni_poly.evaluate(F::zero()) + uni_poly.evaluate(F::one());
            if claimed_sum != eval_p0_p1 {
                return false;
            }

            // update the sum
            claimed_sum = uni_poly.evaluate(challenge);
        }

        proof.polynomial.evaluation(challenges.as_slice()) == claimed_sum
    }
}


fn vec_to_bytes<F: PrimeField>(poly: &Vec<F>) -> Vec<u8> {
	let mut bytes = Vec::new();
	for p in poly {
		bytes.extend_from_slice(&p.into_bigint().to_bytes_be());
	}
	bytes
}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::MontConfig;
    use ark_ff::{Fp64, MontBackend};
    use polynomial::interface::MLETrait;
	use polynomial::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;

    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    struct FqConfig;
    type Fq = Fp64<MontBackend<FqConfig, 1>>;

    #[test]
    fn test_sum_calculation() {
        let mle1 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(0), Fq::from(1), Fq::from(2), Fq::from(3)]);
        let mle2 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(1)]);
        let composedmle1 = ComposedMultiLinearPolynomial::new(vec![mle1, mle2]);
        // let mut prover = ComposedSumcheck::new(composedmle1);
        let sum = ComposedSumcheck::calculate_sum(&composedmle1);
        assert_eq!(sum, Fq::from(3));

        let mle1 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(3), Fq::from(3), Fq::from(5), Fq::from(5)]);
        let mle2 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(1)]);
        let composedmle2 = ComposedMultiLinearPolynomial::new(vec![mle1, mle2]);
        // let mut prover = ComposedSumcheck::new(composedmle2);
        let sum2 = ComposedSumcheck::calculate_sum(&composedmle2);
        assert_eq!(sum2, Fq::from(5));

        let mle1 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(0), Fq::from(1), Fq::from(2), Fq::from(3)]);
        let composedmle3 = ComposedMultiLinearPolynomial::new(vec![mle1]);
        // let mut prover = ComposedSumcheck::new(composedmle3);
        let sum3 = ComposedSumcheck::calculate_sum(&composedmle3);
        assert_eq!(sum3, Fq::from(6));

        let mle1 = MultiLinearPolynomialEvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(2),
            Fq::from(4),
        ]);
        let composedmle4 = ComposedMultiLinearPolynomial::new(vec![mle1]);
        // let mut prover = ComposedSumcheck::new(composedmle4);
        let sum4 = ComposedSumcheck::calculate_sum(&composedmle4);
        assert_eq!(sum4, Fq::from(12));
    }

    #[test]
    fn test_sum_check_proof() {
        // 2(a^2)b + 3ab
        // (2a + 3)(ab)
        // (2a + 0b + 3)(ab)
        // 00 - 3  | 00 - 0
        // 01 - 3  | 01 - 0
        // 10 - 5  | 10 - 0
        // 11 - 5  | 11 - 1
        let mle1 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(3), Fq::from(3), Fq::from(5), Fq::from(5)]);
        let mle2 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(1)]);
        let composedmle = ComposedMultiLinearPolynomial::new(vec![mle1, mle2]);
        let sumcheck = ComposedSumcheck::new(composedmle);
        let (proof, _challenges) = &sumcheck.prove();
        let sum = ComposedSumcheck::calculate_sum(&proof.polynomial);
        let verifer: bool = sumcheck.verify(proof, sum);
        assert_eq!(verifer, true);
    }

    #[test]
    fn test_sum_check_proof1() {
        let mle = MultiLinearPolynomialEvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(2),
            Fq::from(7),
            Fq::from(3),
            Fq::from(3),
            Fq::from(6),
            Fq::from(11),
        ]);
        let composedmle = ComposedMultiLinearPolynomial::new(vec![mle]);
        let sumcheck = ComposedSumcheck::new(composedmle);
        let (proof, _challenges) = &sumcheck.prove();
        let sum = ComposedSumcheck::calculate_sum(&proof.polynomial);
        let verifer: bool = sumcheck.verify(&proof, sum);
        assert_eq!(verifer, true);
    }

    #[test]
    fn test_sum_check_proof_2() {
        let mle = MultiLinearPolynomialEvaluationForm::new(vec![
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(1),
            Fq::from(1),
            Fq::from(1),
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
            Fq::from(0),
        ]);
        let composedmle = ComposedMultiLinearPolynomial::new(vec![mle]);
        let sumcheck = ComposedSumcheck::new(composedmle);
        let proof = sumcheck.prove();
        let sum = ComposedSumcheck::calculate_sum(&proof.0.polynomial);
        let verifer = sumcheck.verify(&proof.0, sum);

        assert_eq!(verifer, true);
    }

    #[test]
    fn test_sum_check_proof_3() {
        let mle = MultiLinearPolynomialEvaluationForm::new(vec![
            Fq::from(1),
            Fq::from(3),
            Fq::from(5),
            Fq::from(7),
            Fq::from(2),
            Fq::from(4),
            Fq::from(6),
            Fq::from(8),
            Fq::from(3),
            Fq::from(5),
            Fq::from(7),
            Fq::from(9),
            Fq::from(4),
            Fq::from(6),
            Fq::from(8),
            Fq::from(10),
        ]);
        let composedmle = ComposedMultiLinearPolynomial::new(vec![mle]);
        let sumcheck = ComposedSumcheck::new(composedmle);
        let proof = sumcheck.prove();
        let sum = ComposedSumcheck::calculate_sum(&proof.0.polynomial);
        let verifer = sumcheck.verify(&proof.0, sum);

        assert_eq!(verifer, true);
    }
}
