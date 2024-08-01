use crate::datastructure::Circuit;
pub fn label_to_binary_to_decimal(a: usize, b: usize, c: usize) -> usize {
    let a_shifted = a << 4;
    let b_shifted = b << 2;
    a_shifted | b_shifted | c
}

pub fn size_of_number_of_variable_at_each_layer(layer_index: usize) -> usize {
    if layer_index == 0 {
        return 1 << 3;
    }
    let layer_index_plus_one = layer_index + 1;
    let number_of_variable = layer_index + (2 * layer_index_plus_one);
    1 << number_of_variable
}

mod tests {
	use super::*;
	#[test]
	fn test_label_binary_and_to_decimal() {
		assert_eq!(label_to_binary_to_decimal(0, 0, 1), 1);
		assert_eq!(label_to_binary_to_decimal(1, 2, 3), 27);
	}

}