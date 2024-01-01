use itertools::Itertools;

fn main() {
    // Initialize
    let n = 6;

    // Retrieve the total number of permutations
    let total_permutations = count_permutations(n);
    println!("\n{total_permutations}");

    // Generate all possible permutations
    let vec_of_permutations = list_permutations(n);
    print_permutations(vec_of_permutations);
}

fn print_permutations(vec_of_permutations: Vec<Vec<usize>>) {
    for row in &vec_of_permutations {
        for i in row {
            print!("{} ", i);
        }
        println!("");
        // println!("{:?}", row)
    }
}

fn list_permutations(n: usize) -> Vec<Vec<usize>> {
    let master_vec: Vec<_> = (1..=n).permutations(n).collect();
    master_vec
}

/// Given n <= 7, Calculate the total number of permutations of n items
/// n! / (n - r)!, where r = n, so n! / (n -n)! = n!
fn count_permutations(n: usize) -> usize {
    let total_permutations = (1..=n).fold(1, |acc, x| acc * x);
    // Tabulate each possible permutation
    total_permutations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_permutations() {
        // Test case 1: n = 0
        assert_eq!(list_permutations(0), vec![vec![]]);

        // Test case 2: n = 1
        assert_eq!(list_permutations(1), vec![vec![1]]);

        // Test case 3: n = 3
        assert_eq!(
            list_permutations(3),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ]
        );

        // Test case 4: n = 4
        assert_eq!(
            list_permutations(4),
            vec![
                vec![1, 2, 3, 4],
                vec![1, 2, 4, 3],
                vec![1, 3, 2, 4],
                vec![1, 3, 4, 2],
                vec![1, 4, 2, 3],
                vec![1, 4, 3, 2],
                vec![2, 1, 3, 4],
                vec![2, 1, 4, 3],
                vec![2, 3, 1, 4],
                vec![2, 3, 4, 1],
                vec![2, 4, 1, 3],
                vec![2, 4, 3, 1],
                vec![3, 1, 2, 4],
                vec![3, 1, 4, 2],
                vec![3, 2, 1, 4],
                vec![3, 2, 4, 1],
                vec![3, 4, 1, 2],
                vec![3, 4, 2, 1],
                vec![4, 1, 2, 3],
                vec![4, 1, 3, 2],
                vec![4, 2, 1, 3],
                vec![4, 2, 3, 1],
                vec![4, 3, 1, 2],
                vec![4, 3, 2, 1],
            ]
        );
    }

    #[test]
    fn test_count_permutations() {
        // Test case 1: n = 0
        assert_eq!(count_permutations(0), 1);

        // Test case 2: n = 1
        assert_eq!(count_permutations(1), 1);

        // Test case 3: n = 5
        assert_eq!(count_permutations(5), 120);

        // Test case 4: n = 7
        assert_eq!(count_permutations(7), 5040);
    }
}
