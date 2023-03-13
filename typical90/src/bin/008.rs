use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    }

    let md = 1_000_000_007;

    let atc = "atcoder";
    let mut dp = vec![vec![0; 8]; n+1];

    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..8 {
            dp[i+1][j] += dp[i][j];

            if j < 7 && s.as_bytes()[i] == atc.as_bytes()[j] {
                dp[i+1][j+1] += dp[i][j];
            }
            dp[i+1][j] %= md;
        }
    }

    println!("{}", dp[n][7]);
}
