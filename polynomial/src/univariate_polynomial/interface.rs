use ark_ff::PrimeField;
pub trait PolynomialInterface<F: PrimeField> {
    type Point;

    /// Return the total degree of the polynomial
    fn degree(&self) -> usize;

    /// Evaluates `self` at the given `point` in `Self::Point`.
    fn evaluate(&self, point: &Self::Point) -> F;

    /// Checks if the polynomial is zero
    fn is_zero(&self) -> bool;
}

pub trait UnivariantPolynomialInterface<F: PrimeField>: PolynomialInterface<F> {
    /// This function returs an array of co-efficents of this polynomial
    fn coefficients(&self) -> &[F];
    /// This function createsa new polynomial from a list of coefficients slice
    fn from_coefficients_slice(coeffs: &[F]) -> Self;
    /// This function creates a new polynomial from a list of coefficients vector
    fn from_coefficients_vec(coeffs: Vec<F>) -> Self;
    /// This function is used to create a new univariate polynomial using an interpolation
    fn interpolate(point_ys: Vec<F>, domain: Vec<F>) -> Self;
}