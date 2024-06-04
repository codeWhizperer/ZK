
// generate proof => generate challenges
// proof verification
use sha3::{Keccak256, Digest};
use ark_ff::PrimeField;
// Define data structure
#[derive(Debug,Clone)]
pub struct Transcript{
   pub hasher:Keccak256
}

// Hash function integration
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
    F::from_random_bytes(&self.hasher.finalize_reset()).unwrap()
   }

// todo!()-> implement transcript with blake2
}

#[cfg(test)]
mod tests{
    use super::*;

    // fn test_new_transcript(){
    //     let new_transcript = Transcript::new();
    //     let new_sha = Keccak256::new();
    //     // assert_eq!(new_transcript.clone().finalize(), new_sha.finalize(), "should be equal");
    // }
}