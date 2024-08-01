use polynomial::multilinear::{evaluation_form::MultiLinearPolynomialEvaluationForm, interface::MultiLinearPolynomialEvaluationFormTrait};
use ark_ff::PrimeField;


pub struct Wire<F:PrimeField>{
  pub  add_i: MultiLinearPolynomialEvaluationForm<F>,
  pub  mul_i: MultiLinearPolynomialEvaluationForm<F>,
  pub  w_b: MultiLinearPolynomialEvaluationForm<F>,
  pub  w_c: MultiLinearPolynomialEvaluationForm<F>
}


impl <F:PrimeField>  Wire<F>{
pub fn new(add_i: MultiLinearPolynomialEvaluationForm<F>, mul_i: MultiLinearPolynomialEvaluationForm<F>, w_b:MultiLinearPolynomialEvaluationForm<F>, w_c:MultiLinearPolynomialEvaluationForm<F>)-> Self{
    Self { add_i, mul_i, w_b, w_c }
}

pub fn evaluate_wire(&self, point: &[F]) -> Option<F>{
    let add_i_evaluation: F = self.add_i.evaluation(point);
    let mul_i_evaluation: F = self.mul_i.evaluation(point);
    let w_b_evaluation: F = self.w_b.evaluation(point);
    let w_c_evaluation: F = self.w_c.evaluation(point);
    Some(add_i_evaluation * (w_b_evaluation + w_c_evaluation) + mul_i_evaluation * (w_b_evaluation * w_c_evaluation))
}

pub fn partial_evaluate_wire(&self, point: F, variable_index:usize) -> Self{
 let add_i_partial: MultiLinearPolynomialEvaluationForm<F> = self.add_i.partial_evaluation(point, variable_index);
 let mul_i_partial: MultiLinearPolynomialEvaluationForm<F> = self.mul_i.partial_evaluation(point, variable_index);
 let w_b_partial: MultiLinearPolynomialEvaluationForm<F> = self.w_b.partial_evaluation(point, variable_index);
 let w_c_partial: MultiLinearPolynomialEvaluationForm<F> = self.w_c.partial_evaluation(point, variable_index);

 Self { add_i: add_i_partial, mul_i: mul_i_partial, w_b: w_b_partial, w_c: w_c_partial }

}

// pub fn partial_evaluate_wires(&self, points: &[F], variable_indices: &[usize]) -> Self {
//   if points.len() != variable_indices.len() {
//       panic!(
//           "The length of points and variable_indices should be the same: {}, {}",
//           points.len(),
//           variable_indices.len()
//       );
//   }

//   let mut evaluation_structure = self.clone();

//   for i in 0..points.len() {
//       evaluation_structure = evaluation_structure.partial_evaluate_wire(points[i], variable_indices[i]);
//   }

//   evaluation_structure
// }

}



