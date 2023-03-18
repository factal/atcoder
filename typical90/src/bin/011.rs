use std;
use proconio::input;

fn main() {
    input! {
        n: usize,
        tasks: [(usize, usize, usize); n],
    }

    let mut tasks = tasks;
    tasks.sort_by(|a, b| a.0.cmp(&b.0));
    
    let mut dp = vec![vec![0; 5001]; n+1];

    for i in 0..n {
        let (d, c, s) = tasks[i];
        
        for j in 0..5001 {
            if j < c || d < j {
                dp[i+1][j] = dp[i][j];
            } else {
                dp[i+1][j] = cmp::max(dp[i][j], dp[i][j-c] + s);
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
