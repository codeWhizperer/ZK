
pub fn pick_pairs_with_index(
    num_of_evaluations: usize,
    variable_index: usize,
) -> Vec<(usize, usize)> {
    assert!(num_of_evaluations % 2 == 0, "n must be even");
    assert!(
        variable_index < num_of_evaluations / 2,
        "variable_index must be less than n/2"
    );

    let mut result = Vec::new();
    let iters = 1 << variable_index;

    for _ in 0..iters {
        let mut round: Vec<(usize, usize)> = Vec::new();

        for y_1 in 0..((num_of_evaluations / iters) / 2) {
            round.push((
                y_1 + result.len() * 2,
                ((num_of_evaluations / iters) / 2) + y_1 + result.len() * 2,
            ));
        }

        result.extend(round);
    }

    result
}
pub fn compute_number_of_variables(n: u128) -> (u128, u128) {
    if n == 0 {
        return (0, 0);
    }
    if n == 1 {
        return (1, 2);
    }

    let mut log_base_2 = n.ilog2();
    let mut n_power_2 = 1 << log_base_2;

    if n != n_power_2 {
        log_base_2 += 1;
        n_power_2 = 1 << log_base_2;
    }

    (log_base_2 as u128, n_power_2)
}