use ark_ff::PrimeField;

use crate::multilinear::interface::MultiLinearPolynomialEvaluationFormTrait;

use super::utils::pick_pairs_with_index;


#[derive(Debug, Clone, PartialEq)]
struct MultiLinearPolynomialEvaluationForm<F:PrimeField>{
    number_of_variables: usize,
    evaluations: Vec<F>
}


impl<F:PrimeField> MultiLinearPolynomialEvaluationFormTrait<F> for MultiLinearPolynomialEvaluationForm<F>{
  fn new(evaluations: Vec<F>) -> Self {
    let number_of_variables =  (evaluations.len() as f64).log2() as usize;
    assert_eq!(evaluations.len(), 1 << number_of_variables ,"Num of evaluation must be equal 2^number_of_variable");
    Self { number_of_variables, evaluations }
}  

fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> Self {
    let new_evaluation:&Vec<F> = &self.evaluations;
    let mut result:Vec<F> = Vec::with_capacity(self.evaluations.len() /2);

    for (i, j) in pick_pairs_with_index(self.evaluations.len(), variable_index){
        let y_1 = &new_evaluation[i];
        let y_2 = &new_evaluation[j];

        let result_y:F = (evaluation_point * y_2) + ((F::one() - evaluation_point) * y_1);
        result.push(result_y);
    }
    Self { number_of_variables: self.number_of_variables - 1, evaluations: result }
}

fn full_evaluation(&self, evaluation_point: &[F]) -> F{
    assert_eq!(evaluation_point.len(), self.number_of_variables, "must be equal");
    let mut polynomial_result:F = F::one();
    let mut evaluation_result = self.clone();

    for i in 0..evaluation_point.len(){
        evaluation_result = evaluation_result.partial_evaluation(evaluation_point[i], 0);
    }
    polynomial_result = evaluation_result.evaluations[0];
    polynomial_result
}

}


#[cfg(test)]
mod test{
   use crate::multilinear::interface::MultiLinearPolynomialEvaluationFormTrait;
use crate::multilinear::{evaluation_form::MultiLinearPolynomialEvaluationForm};
   use ark_ff::MontConfig;
    use ark_ff::{Fp64, MontBackend};
    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    struct FqConfig;
    type Fq = Fp64<MontBackend<FqConfig, 1>>;

    #[test]
    fn test_new_multilinear_polynomial(){
    let evaluations = vec![Fq::from(3),Fq::from(1),Fq::from(2),Fq::from(5)];
    let new_poly = MultiLinearPolynomialEvaluationForm::new(evaluations);
    assert_eq!(new_poly.evaluations.len(), 1 << new_poly.number_of_variables);
    }

    #[test]
    fn test_partial_evaluation(){
        let evaluations = vec![Fq::from(3),Fq::from(1),Fq::from(2),Fq::from(5)];
        let polynomial = MultiLinearPolynomialEvaluationForm::new(evaluations);
        let evaluation_point = Fq::from(5);
        let new_polynomial = MultiLinearPolynomialEvaluationForm::partial_evaluation(&polynomial, evaluation_point,0);
        let expected_polynomial = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(15), Fq::from(4)]);
        assert_eq!(new_polynomial, expected_polynomial);
    }

    #[test]
    fn test_full_evaluation(){
        let evaluations = vec![
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(3),
        Fq::from(0),
        Fq::from(0),
        Fq::from(2),
        Fq::from(5)];
    let polynomial = MultiLinearPolynomialEvaluationForm::new(evaluations);
    let evaluation_points = vec![Fq::from(2), Fq::from(3), Fq::from(4)];
    let evaluation_result = MultiLinearPolynomialEvaluationForm::full_evaluation(&polynomial, &evaluation_points);
    assert_eq!(evaluation_result, Fq::from(48));
    }
}