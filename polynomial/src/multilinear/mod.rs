//////////////////////////////
//IMPORTS
////////////////////////////

use std::fmt::{Display, Formatter, Result};
// what structure do i use to represent this in code?
use ark_ff::PrimeField;


//////////////////////////////
//MULTILINEARMONOMIAL
////////////////////////////

#[derive(Debug,Clone, PartialEq)]
pub struct MultilinearMonomial<F:PrimeField>{
    pub  evaluations: F,
   pub variables: Vec<bool>,
    }
    impl <F:PrimeField>MultilinearMonomial<F>{
        pub fn new(variables:Vec<bool>, evaluations:F) -> Self{
           MultilinearMonomial{variables, evaluations}
        }
     pub  fn add(self, rhs:Self) -> MultilinearPolynomial<F>{
            let mut result = MultilinearPolynomial::new(vec![]);
            if self.variables == rhs.variables{
                result.terms.push(
                    MultilinearMonomial::new(self.variables.clone(), self.evaluations + rhs.evaluations)
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
    MultilinearMonomial::new(new_variables, self.evaluations * rhs.evaluations)
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
    // multiply multilinearpolynomial



   // evaluate

pub   fn evaluations(&self, eval_points:Vec<F>) -> F{
    let mut evaluation_result = F::zero();
    for (_, term) in self.terms.iter().enumerate(){
        let mut variable_result = F::one();

        for(index, value) in term.variables.iter().enumerate(){
            if *value == true{
                variable_result *= eval_points[index]
            }
        }
        evaluation_result += term.evaluations * variable_result;
    }
    evaluation_result
   } 


} 





impl<F: PrimeField> Display for MultilinearPolynomial<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut first = true;

        for i in 0..self.terms.len() {
            if first {
                first = false;
            } else {
                write!(f, " + ")?;
            }

            write!(f, "{}", self.terms[i].evaluations)?;

            for (j, var) in self.terms[i].variables.iter().enumerate() {
                if *var {
                    write!(f, "{}", (b'a' + j as u8) as char)?;
                }
            }
        }
        Ok(())
    }
}

// impl<F: PrimeField + Display> Display for MultilinearMonomial<F> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         // Print the coefficient, handling special cases for 1 and -1 if desired
//         if self.evaluations != F::from(1u64) && self.evaluations != F::from(0u64) { // Assuming F::from is the way to convert u64 to F
//             write!(f, "{}", self.evaluations)?;
//         }

//         // Append variable representations
//         let mut variable_count = 0;
//         for (index, &is_present) in self.variables.iter().enumerate() {
//             if is_present {
//                 // To handle common variable naming, assuming 'a' starts and goes to 'z' and beyond
//                 write!(f, "{}", (b'a' + index as u8) as char)?;
//                 variable_count += 1;
//             }
//         }

//         // If no variables are present, just print the coefficient
//         if variable_count == 0 {
//             write!(f, "1")?;
//         }

//         Ok(())
//     }
// }
