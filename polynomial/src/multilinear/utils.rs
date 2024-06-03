use ark_ff::PrimeField;

pub fn pick_pairs_with_index(number_of_evaluations:usize, variable_index:usize) -> Vec<(usize,usize)>{
    assert!(number_of_evaluations % 2 == 0, "must be even");
    assert!(variable_index < number_of_evaluations /2 , "variable index must be less than number_of_evaluation /2");
    let mut result = Vec::new();
    let iterator = 1 << variable_index;
    for _ in 0..iterator{
        let mut rounds:Vec<(usize, usize)> = Vec::new();
        for y in 0..((number_of_evaluations / iterator) /2){
            rounds.push((
                y + result.len() * 2,
                ((number_of_evaluations / iterator) /2 + y + result.len() * 2)
            ));
        }
        result.extend(rounds);
    }
    result
}