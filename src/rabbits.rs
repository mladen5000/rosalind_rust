

fn main() {
    println!("Hello, World!");
    // let result = fibonacci(32, 4);
    // let result = fibomemo(40, 3);
    let result = mortal_rabbits(85, 17);
    // let result = blah(85, 17);
    println!("{result:?}");
}

fn fibonacci(n: usize, k: usize) -> usize {
    if n == 1 || n == 2 {
        return 1;
    } else {
        return fibonacci(n - 1, k) + k * fibonacci(n - 2, k);
    }
}

fn fibomemo(n: usize, k: usize) -> usize {
    let mut memo = vec![1; n + 1];

    for i in 3..=(n as usize) {
        memo[i] = memo[i - 1] + k * memo[i - 2];
    }
    memo[n]
}

fn mortal_rabbits(n: usize, m: usize) -> usize {
    let mut memo = vec![1; n + 1];
    for i in 3..=n {
        memo[i] = memo[i - 1] + memo[i - 2] - if i > m { memo[i - m - 1] } else { 0 };
    }
    memo[n]
}

fn blah(n: usize, m: usize) -> Vec<usize> {
    let mut gs = vec![0; n];
    gs[0] = 1;
    for _ in 0..m {
        let sum = gs[1..].iter().sum();
        println!("{}", sum);
        gs.insert(0, sum);
        gs.pop();
    }
    println!("{}", gs.iter().sum::<usize>());
    gs
}

