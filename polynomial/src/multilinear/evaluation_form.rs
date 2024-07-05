use ark_ff::{PrimeField,BigInteger};

use crate::multilinear::interface::MultiLinearPolynomialEvaluationFormTrait;

use super::utils::pick_pairs_with_index;
use std::ops::{Add,AddAssign};

#[derive(Debug, Clone, PartialEq,Default)]
pub struct MultiLinearPolynomialEvaluationForm<F:PrimeField>{
  pub  number_of_variables: usize,
  pub  evaluations: Vec<F>
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

fn generate_variable_names(&self) -> Vec<String>{
    (0..self.number_of_variables).map(|i|(b'a' + i as u8) as char).map(|c|c.to_string()).collect()
}

 fn zero(num_vars: usize) -> Self {
   let addictive = MultiLinearPolynomialEvaluationForm::new(vec![F::zero(); 1 << num_vars]);
   addictive
}
fn to_bytes(&self) -> Vec<u8> {
    let mut m_ploy_bytes = Vec::new();

    for eval in &self.evaluations {
        let big_int = eval.into_bigint().to_bytes_be();
        m_ploy_bytes.extend_from_slice(&big_int);
    }

    m_ploy_bytes
}

fn evaluate(&self, point: &Vec<F>) -> Option<F> {
    let mut eval_result = None;
    let mut eval_polynomial = self.clone();

    for i in 0..point.len() {
        eval_polynomial = eval_polynomial.partial_evaluation(point[i], 0);
        eval_result = Some(eval_polynomial.evaluations[0]);
    }

    eval_result
}
}



impl<F: PrimeField> Add for MultiLinearPolynomialEvaluationForm<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new_evaluations = Vec::new();
        if self.number_of_variables != other.number_of_variables {
            panic!("The number of variables in the two polynomials must be the same");
        }

        for i in 0..self.evaluations.len() {
            new_evaluations.push(self.evaluations[i] + other.evaluations[i]);
        }

        Self::new(new_evaluations)
    }
}

impl<F: PrimeField> AddAssign for MultiLinearPolynomialEvaluationForm<F> {
    fn add_assign(&mut self, other: Self) {
        // if self.number_of_variables != other.number_of_variables {
        //     panic!("The number of variables in the two polynomials must be the same");
        // }

        for i in 0..self.evaluations.len() {
            self.evaluations[i] += other.evaluations[i];
        }
    }
}



#[cfg(test)]
mod test{
   use std::vec;

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
        let expected_polynomial = MultiLinearPolynomialEvaluationForm::new(vec![Fq::from(-2), Fq::from(21)]);
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

#[test]
fn test_partial_evaluation_2(){
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

    let evaluations2 = vec![
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
let points = vec![Fq::from(2), Fq::from(3), Fq::from(1)];

let evaluation_result = MultiLinearPolynomialEvaluationForm::full_evaluation(&new_poly, &points);

assert_eq!(evaluation_result, Fq::from(39));

let poly2 = MultiLinearPolynomialEvaluationForm::new(evaluations2);
let point2 = vec![Fq::from(3), Fq::from(2)];
let partial_eval = MultiLinearPolynomialEvaluationForm::partial_evaluation(&poly2, Fq::from(3), 1);
let expected = MultiLinearPolynomialEvaluationForm::full_evaluation(&partial_eval, &point2);
assert_eq!(expected, Fq::from(72));
}
    
}
 