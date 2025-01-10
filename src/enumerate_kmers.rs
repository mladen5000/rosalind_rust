fn permutations_with_repeats(k: usize) -> Vec<Vec<char>> {
    let hashset = ['A', 'B', 'C', 'D', 'E', 'F'];
    if k == 0 {
        return vec![Vec::new()];
    }

    let mut result = vec![];

    for index in 0..hashset.len() {
        for mut perm in permutations_with_repeats(k - 1) {
            perm.insert(0, hashset[index].clone());
            result.push(perm);
        }
    }
    result
}
fn main() {
    let k = 3;
    let all_permutations = permutations_with_repeats(k);
    for each_perm in all_permutations.iter() {
        for c in each_perm {
            print!("{c}");
        }
        println!();
    }
    println!("{}", all_permutations.len());
}
