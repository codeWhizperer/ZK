//////////////////////////////
//IMPORTS
////////////////////////////

use std::fmt::{Display};
use ark_ff::PrimeField;



//////////////////////////////
//MULTILINEARMONOMIAL
////////////////////////////

#[derive(Debug,Clone, PartialEq)]
pub struct MultilinearMonomial<F:PrimeField>{
    pub  coefficient: F,
   pub variables: Vec<bool>,
    }


    
    impl <F:PrimeField>MultilinearMonomial<F>{
        pub fn new(coefficient:F,variables:Vec<bool>) -> Self{
            assert!(
                variables.len() > 0,
                "Length of variables must be greater than zero"
            );
           MultilinearMonomial{coefficient, variables}
        }
     pub  fn add(self, rhs:Self) -> MultilinearPolynomial<F>{
            let mut result = MultilinearPolynomial::new(vec![]);
            if self.variables == rhs.variables{
                result.terms.push(
                    MultilinearMonomial::new(self.coefficient + rhs.coefficient, self.variables.clone())
                );
            }else{
                result.terms.push(self);
                result.terms.push(rhs)
            }
            result
        }
    pub fn mul(self, mut rhs:Self) -> MultilinearMonomial<F>{
    let mut  new_variables = self.variables.clone();
    new_variables.append(&mut rhs.variables);
    MultilinearMonomial::new(self.coefficient * rhs.coefficient, new_variables)
    }
    
    } 



    
 /////////////////////////////////////////////////////////
//MULTILINEAR POLYNOMAL DEREVIED FROM COMBINING MONOMIALS
/////////////////////////////////////////////////////////

#[derive(Debug,Clone, PartialEq)]
pub struct MultilinearPolynomial<F:PrimeField>{
   pub terms: Vec<MultilinearMonomial<F>>
    }


impl <F:PrimeField>MultilinearPolynomial<F>{
    pub fn new(terms:Vec<MultilinearMonomial<F>>) -> Self{
       MultilinearPolynomial{terms}
    }
// add multilinearPolynomial
// todo!()
// multiply multilinearpolynomial
// todo!()
// interpolation


// partial
// pub fn partial_eval(&self, eval_points: F) -> Self {
//     let mut res: MultilinearPolynomial<F> = MultilinearPolynomial { terms: vec![] };

//     for (i, j) in MultilinearPolynomial::pick_pairs_with_index(&self.terms) {
//         let y1 = &self.terms[i].coefficient;
//         let y2 = &self.terms[j].coefficient;
//         // r.y1 + (1-r).y2 
//         let y = (eval_points * y2) + ((F::from(1_u8) - eval_points) * y1);

//         res.terms.push(MultilinearMonomial {
//             coefficient: y,
//             variables: vec![false],
//         })
//     }

//     res
// }
pub fn partial_eval(&self, eval_points: &[Option<F>]) -> MultilinearPolynomial<F> {
    let mut result = MultilinearPolynomial { terms: Vec::new() };

    'outer: for term in &self.terms {
        let mut new_coefficient = term.coefficient;
        let mut new_variables = Vec::with_capacity(term.variables.len());

        for (var_included, eval_point) in term.variables.iter().zip(eval_points.iter()) {
            match eval_point {
                Some(value) if *var_included => {
                    new_coefficient *= *value;
                },
                None => {
                    new_variables.push(*var_included);
                },
                _ => continue 'outer,
            }
        }

        result.terms.push(MultilinearMonomial {
            coefficient: new_coefficient,
            variables: new_variables,
        });
    }

    result
}

pub fn evaluate(&self, eval_points: Vec<F>) -> F {
    let mut evaluation_result = F::zero(); 

    for term in &self.terms {
        let mut variable_result = term.coefficient; 

        for (index, &include_var) in term.variables.iter().enumerate() {
            if include_var { 
                if index >= eval_points.len() {
                    panic!("out of bound")
                }
                variable_result *= eval_points[index]; 
            }
        }

        evaluation_result += variable_result; 
    }

    evaluation_result
}

pub fn pick_pairs_with_index(terms: &Vec<MultilinearMonomial<F>>)-> Vec<(usize, usize)>{
    let length = terms.len();
    let mut pairs = Vec::with_capacity(length / 2);
    for i in 0..(length /2){
        let j = i + length /2;
        pairs.push((i,j));
    }
    pairs
}


} 





// impl<F: PrimeField> Display for MultilinearPolynomial<F> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         let mut first = true;

//         for i in 0..self.terms.len() {
//             if first {
//                 first = false;
//             } else {
//                 write!(f, " + ")?;
//             }

//             write!(f, "{}", self.terms[i].coefficient)?;

//             for (j, var) in self.terms[i].variables.iter().enumerate() {
//                 if *var {
//                     write!(f, "{}", (b'a' + j as u8) as char)?;
//                 }
//             }
//         }
//         Ok(())
//     }
// }

impl<F: PrimeField> Display for MultilinearMonomial<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.coefficient == F::zero() {
            return Ok(());
        }
        write!(f, "{}{:?}", self.coefficient, self.variables)
    }
}

impl<F: PrimeField> Display for MultilinearPolynomial<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.terms.len() {
            if i == 0 {
                if self.terms[i].coefficient == F::zero() {
                    continue;
                }
                write!(f, "{}", self.terms[i]).unwrap();
                continue;
            }
            if self.terms[i].coefficient == F::zero() {
                continue;
            }
            write!(f, " + {}", self.terms[i]).unwrap();
        }
        Ok(())
    }
}

