use ark_ff::PrimeField;
use crate::{datastructure::{Circuit, CircuitEvaluation, GKRProof}, interfaces::GKRProtocolInterface, utils::{w_mle, perform_layer_one_prove_sumcheck}};
use transcript::transcription::{self, Transcript};
use polynomial::multilinear::{evaluation_form::MultiLinearPolynomialEvaluationForm, interface::MultiLinearPolynomialEvaluationFormTrait};
use polynomial::composed::{multilinear::ComposedMultiLinearPolynomial,interface::ComposedMultilinearInterface};
// use sum_check::composed_sumcheck::ComposedSumcheckProof;
use sum_check::multi_composed_sumcheck::{MultiComposedSumcheckProver, ComposedSumcheckProof};

pub struct GKRProtocol;

impl GKRProtocol{
    pub fn prove<'a, F: PrimeField>(circuit: &'a Circuit, input: &'a Vec<F>) -> GKRProof<F> {
        let mut transcript = Transcript::new();
        let mut sumcheck_proofs: Vec<ComposedSumcheckProof<F>> = Vec::new();
        let mut wb_s: Vec<F> = Vec::new();
        let mut wc_s: Vec<F> = Vec::new();
        let mut r_b: Vec<F> = Vec::new();
        let mut r_c: Vec<F> = Vec::new();

        let mut alpha: F = F::zero();
        let mut beta: F = F::zero();

        let circuit_eval = circuit.evaluate(input);
        let mut circuit_eval_layer_zero_pad = circuit_eval.layers[0].clone();
        circuit_eval_layer_zero_pad.push(F::zero());

        let w_0_mle = w_mle(circuit_eval_layer_zero_pad.to_vec());
        transcript.append(&w_0_mle.to_bytes());

        let n_r: Vec<F> = transcript.sample_n_as_field_element(w_0_mle.number_of_variables);
        let mut claimed_sum: F = w_0_mle.evaluation(&n_r);

        let (add_mle_1, mul_mle_1) = circuit.add_i_mul_ext::<F>(0);
        let w_1_mle = w_mle(circuit_eval.layers[1].to_vec());

        let (claimed, alpha, bta, rb, rc) = perform_layer_one_prove_sumcheck(
            &add_mle_1,
            &mul_mle_1,
            &w_1_mle,
            &n_r,
            &claimed_sum,
            &mut transcript,
            &mut sumcheck_proofs,
            &mut wb_s,
            &mut wc_s,
        );

        claimed_sum = claimed;
        alpha = alpha;
        beta = bta;
        r_b = rb;
        r_c = rc;

        for layer_index in 2..circuit_eval.layers.len() {
            let (add_mle, mult_mle) = circuit.add_i_mul_ext::<F>(layer_index - 1);

            let add_rb_bc = add_mle.partial_evaluations(&r_b, &vec![0; r_b.len()]);
            let mul_rb_bc = mult_mle.partial_evaluations(&r_b, &vec![0; r_b.len()]);

            let add_rc_bc = add_mle.partial_evaluations(&r_c, &vec![0; r_b.len()]);
            let mul_rc_bc = mult_mle.partial_evaluations(&r_c, &vec![0; r_b.len()]);
            let w_i_mle = w_mle(circuit_eval.layers[layer_index].to_vec());

            let wb = w_i_mle.clone();
            let wc = w_i_mle;

            let wb_add_wc = wb.add_distinct(&wc);
            let wb_mul_wc = wb.mul_distinct(&wc);

            // alpha * add(r_b, b, c) + beta * add(r_c, b, c)
            let add_alpha_beta = (add_rb_bc * alpha) + (add_rc_bc * beta);
            // alpha * mul(r_b, b, c) + beta * mult(r_c, b, c)
            let mul_alpha_beta = (mul_rb_bc * alpha) + (mul_rc_bc * beta);

            let fbc_add_alpha_beta = ComposedMultiLinearPolynomial::new(vec![add_alpha_beta, wb_add_wc]);
            let fbc_mul_alpha_beta = ComposedMultiLinearPolynomial::new(vec![mul_alpha_beta, wb_mul_wc]);

            let (sumcheck_proof, challenges) = MultiComposedSumcheckProver::prove(
                &vec![fbc_add_alpha_beta, fbc_mul_alpha_beta],
                &claimed_sum,
            )
            .unwrap();

            transcript.append(&sumcheck_proof.to_bytes());
            sumcheck_proofs.push(sumcheck_proof);

            let (b, c) = challenges.split_at(&challenges.len() / 2);

            let eval_wb = wb.evaluation(&b);
            let eval_wc = wc.evaluation(&c);
            wb_s.push(eval_wb);
            wc_s.push(eval_wc);

            r_b = b.to_vec();
            r_c = c.to_vec();

            alpha = transcript.transform_challenge_to_field::<F>();
            beta = transcript.transform_challenge_to_field::<F>();

            claimed_sum = alpha * eval_wb + beta * eval_wc;
        }

        GKRProof {
            sumcheck_proofs,
            wb_s,
            wc_s,
            w_0_mle,
        }
    }
    fn verify<F: PrimeField>(circuit: &Circuit, input: &[F], proof: &crate::datastructure::GKRProof<F>) -> bool {
        todo!()
    }
}