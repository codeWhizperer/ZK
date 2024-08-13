use crate::{composed_sumcheck::{ComposedSumCheck}, interface::{ComposedSumCheckInterface}};
use ark_ff::PrimeField;
use polynomial::{composed::{interface::ComposedMultilinearInterface, multilinear::ComposedMultiLinearPolynomial}, interface::UnivariatePolynomialTrait, UnivariatePolynomial};
use transcript::transcription::Transcript;
use crate::util::transform_round_poly_to_uni_poly;
#[derive(Debug,Clone)]
pub struct MultiComposedSumCheck<F: PrimeField> {
	pub poly: Vec<ComposedMultiLinearPolynomial<F>>,
	pub sum: F,
}

#[derive(Debug, Clone)]
pub struct MultiComposedSumCheckProof<F: PrimeField> {
	pub poly: Vec<ComposedMultiLinearPolynomial<F>>,
	pub round_polys: Vec<UnivariatePolynomial<F>>,
}
impl <F:PrimeField> MultiComposedSumCheck<F>{
    pub fn new(poly: Vec<ComposedMultiLinearPolynomial<F>>) -> Self{
        MultiComposedSumCheck{poly, sum:Default::default()}
    }

    pub fn calculate_sum(poly: &Vec<ComposedMultiLinearPolynomial<F>>) -> F{
        let mut total_sum = F::zero();

        for p in poly{
            let mut sum = ComposedSumCheck::new(p.clone());
            total_sum += sum.calculate_sum();
        }
        total_sum
    }

    pub fn prove(&self) -> (MultiComposedSumCheckProof<F>, Vec<F>) {
        let mut transcript = Transcript::new();

        let mut current_poly = self.poly.clone();
        let mut round_polys = vec![];
        let mut challenges: Vec<F> = vec![];

        for _ in 0..self.poly[0].number_of_variables() {
            let mut round_poly = UnivariatePolynomial::zero();

            for p in current_poly.iter() {
                let mut round_i_poly_vec = Vec::new();
                for i in 0..=p.max_degree() {
                    let round: F = p
                        .partial_evaluation(F::from(i as u32), 0)
                        .elementwise_product()
                        .iter()
                        .sum::<F>();

                    round_i_poly_vec.push(round);
                }

                let round_i_poly = UnivariatePolynomial::interpolate(
                    &transform_round_poly_to_uni_poly(&round_i_poly_vec),
                );
                round_poly = round_poly + round_i_poly;
            }

            transcript.append(&round_poly.to_bytes());
            let random_r: F = transcript.transform_challenge_to_field::<F>();

            let mut new_poly = Vec::new();

            for i in 0..current_poly.len() {
                new_poly.push(current_poly[i].partial_evaluation(random_r, 0));
            }

            current_poly = new_poly;

            challenges.push(random_r);
            round_polys.push(round_poly);
        }

        (
            MultiComposedSumCheckProof {
                poly: self.poly.clone(),
                round_polys,
            },
            challenges,
        )
    }


    // verify

    pub fn verify(&self, proof: &MultiComposedSumCheckProof<F>, sum: F) -> bool {
        let mut transcript = Transcript::new();

        let mut claimed_sum = sum;
        let mut challenges: Vec<F> = vec![];

        for round_poly in proof.round_polys.iter() {
            transcript.append(&round_poly.to_bytes());
            // genrate the challenge for this round
            let challenge: F = transcript.transform_challenge_to_field::<F>();
            challenges.push(challenge);

            let eval_p0_p1 = round_poly.evaluate(F::zero()) + round_poly.evaluate(F::one());
            if claimed_sum != eval_p0_p1 {
                return false;
            }

            // update the sum
            claimed_sum = round_poly.evaluate(challenge);
        }

        let mut poly_pe_sum = F::zero();
        for round_poly in proof.poly.iter() {
            poly_pe_sum += round_poly.evaluation(&challenges.as_slice())
        }

        poly_pe_sum == claimed_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::sumcheck;

    use super::*;
    use ark_ff::MontConfig;
    use ark_ff::{Fp64, MontBackend};
    use polynomial::multilinear::evaluation_form::MultiLinearPolynomialEvaluationForm;
    use polynomial::multilinear::interface::MultiLinearPolynomialEvaluationFormTrait;
    // use polynomial::interface::MLETrait;
    // use polynomial::MLE;

    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    struct FqConfig;
    type Fq = Fp64<MontBackend<FqConfig, 1>>;

    #[test]
    fn test_sum_calculation() {
        let mle1 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(0), Fq::from(1), Fq::from(2), Fq::from(3)]);
        let mle2 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(1)]);
        let composedmle1 = ComposedMultiLinearPolynomial::new(vec![mle1]);
        let composedmle2 = ComposedMultiLinearPolynomial::new(vec![mle2]);

        let multi_composed_vec_1 = vec![composedmle1, composedmle2];
        let sum_1 = MultiComposedSumCheck::calculate_sum(&multi_composed_vec_1);
        assert_eq!(sum_1, Fq::from(7));

        let mle3 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
        let mle4 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(0), Fq::from(3), Fq::from(0), Fq::from(3)]);
        let composedmle3 = ComposedMultiLinearPolynomial::new(vec![mle3]);
        let composedmle4 = ComposedMultiLinearPolynomial::new(vec![mle4]);

        let multi_composed_vec_2 = vec![composedmle3, composedmle4];
        let sum_2 = MultiComposedSumCheck::calculate_sum(&multi_composed_vec_2);
        assert_eq!(sum_2, Fq::from(8));
    }

    #[test]
    fn test_multi_composed_sumcheck_proof() {
        let poly1 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
        let poly2 = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(0), Fq::from(3), Fq::from(0), Fq::from(3)]);

        let composed_1 = ComposedMultiLinearPolynomial::new(vec![poly1]);
        let composed_2 = ComposedMultiLinearPolynomial::new(vec![poly2]);

        let multi_composed = vec![composed_1, composed_2];


        let sum = MultiComposedSumCheck::calculate_sum(&multi_composed);
        let sumcheck = MultiComposedSumCheck::new(multi_composed);

        let (proof, _) = sumcheck.prove();
        let verify = sumcheck.verify(&proof, sum);
        assert!(verify);
    }
}