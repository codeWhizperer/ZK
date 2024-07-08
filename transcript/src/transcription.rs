use sha3::{Keccak256, Digest};
use ark_ff::{PrimeField};
#[derive(Debug,Clone,Default)]
pub struct Transcript{
   pub hasher:Keccak256
}

impl Transcript {
    pub fn new() -> Self{
        Self { hasher: Keccak256::new()  }
    }

    pub fn append(&mut self, new_data: &[u8]){
        self.hasher.update(new_data)
    }

   pub fn sample_challenge(&mut self)-> [u8; 32]{
   let  update_data =  self.hasher.finalize_reset();
   self.hasher.update(&update_data);
   let mut result = [0_u8;32];
   result.copy_from_slice(&update_data);
   result
   } 

   pub fn transform_challenge_to_field<F:PrimeField>(&mut self) -> F{
    F::from_be_bytes_mod_order(&self.sample_challenge())

   }
}



#[cfg(test)]
mod tests{
    use super::*;
    #[test]
      fn test_new_transcript(){
        let new_transcript = Transcript::new().hasher;
        let new_keccak_hash = Keccak256::new();
        assert_eq!(new_transcript.clone().finalize(), new_keccak_hash.finalize(), "should be equal");
    }

    #[test]
fn test_append(){
    let mut new_transcript = Transcript::new();
    new_transcript.append(b"Hello world");
    let mut result = Keccak256::new();
    result.update(b"Hello world");
    assert_eq!(new_transcript.hasher.finalize(), result.finalize());
}

#[test]
fn test_sample_challenge(){
    let mut new_transcript = Transcript::new();
    new_transcript.append(b"Hello world");
    let challenge = new_transcript.sample_challenge();

    assert_eq!(challenge.len(),32,"should be 32 byte length"); 
}

// #[test]
// fn test_transform_challenge_to_field(){
//     let mut new_transcript = Transcript::new();
//     new_transcript.append(b"field data");
//     let field_element = new_transcript.transform_challenge_to_field();
//     assert_ne!(field_element, Fp256::ONE, "Field element should not be zero");
// }
}