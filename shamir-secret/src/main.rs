use ark_ff::PrimeField;
use interface::UnivariatePolynomialTrait;
use polynomial::*;
use std::vec::Vec;

pub fn create_shamir_secret<F: PrimeField + Copy>(
    threshold: usize,
    members: usize,
    secret: F,
) -> (Vec<F>, Vec<F>) {
    let mut rng = rand::thread_rng();
    let mut coefficients = vec![secret];
    for _ in 1..threshold {
        coefficients.push(F::rand(&mut rng))
    }

    let poly = UnivariatePolynomial::new(coefficients);

    let mut shares_x = Vec::with_capacity(members);
    let mut shares_y = Vec::with_capacity(members);
    for i in 1..members {
        let x = F::from(i as u64);
        let y = poly.evaluate(x);
        shares_x.push(x);
        shares_y.push(y);
    }
    (shares_x, shares_y)
}

pub fn recover_secret<F: PrimeField>(shares_x: Vec<F>, shares_y: Vec<F>) -> F {
    let poly = UnivariatePolynomial::interpolate(shares_x, shares_y);
    let secret = poly.evaluate(&F::zero());
    secret
}

fn main() {
    use ark_ff::{Fp64, MontBackend, MontConfig};
    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;
    let threshold = 3;
    let members = 5;
    let secret = Fq::from(123u64);

    let (shares_x, shares_y) = create_shamir_secret::<Fq>(threshold, members, secret);
    let recovered_secret = recover_secret(shares_x, shares_y);

    println!("Recovered secret: {:?}", recovered_secret);
    println!("secret: {:?}", secret);

}

#[cfg(test)]
mod tests {
    use super::*;

    use ark_ff::{Fp64, MontBackend, MontConfig};
    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    pub struct FqConfig;
    pub type Fq = Fp64<MontBackend<FqConfig, 1>>;

    #[test]
    fn test_shamir_secret_sharing() {
        let threshold = 3;
        let members = 5;
        let secret = Fq::from(123u64);

        let (shares_x, shares_y) = create_shamir_secret::<Fq>(threshold, members, secret);
        assert_eq!(shares_x.len(), members - 1);
        assert_eq!(shares_y.len(), members - 1);
    }
    #[test]
    fn test_shamir_secret_recover() {
        let threshold = 3;
        let members = 5;
        let secret = Fq::from(123u64);

        let (shares_x, shares_y) = create_shamir_secret::<Fq>(threshold, members, secret);

        let x = shares_x[1..].to_vec();
        let y = shares_y[1..].to_vec();

        let recovered_secret = recover_secret(x, y);

        println!("Recovered secret: {:?}", recovered_secret);
        println!("secret: {:?}", secret);

        assert!(secret == recovered_secret);
        println!("{:?}", shares_x);
        println!("{:?}", shares_y);
    }
}
