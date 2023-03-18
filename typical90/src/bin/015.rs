use proconio::input;


static MOD: i64 = 1_000_000_007;

fn main() {
    fn ex_gcd(a: i64, b: i64) -> (i64, i64) {
        if b == 0 {
            return (1, 0);
        }
        let (x, y) = ex_gcd(b, a % b);
        (y, x - (a / b) * y)
    }
    
    let mut fact_memo: Vec<i64> = vec![1; 100_001];
    let mut fact_inv_memo: Vec<i64> = vec![1; 100_001];

    for i in 1..100_001 {
        fact_memo[i] = fact_memo[i - 1] * i as i64 % MOD;
        fact_inv_memo[i] = ex_gcd(MOD, fact_memo[i]).1;
    }

    let comb = |n: i64, r: i64| -> i64 {
        let n = n as usize; let r = r as usize;
        (fact_memo[n] * ((fact_inv_memo[r] * fact_inv_memo[n - r]) % MOD)) % MOD
    };

    input! {
        n: i64
    }
    
    for k in 1..=n {
        let mut ans = 0;
        for i in 1..=n/k {
            ans += comb(n - (k - 1) * (i - 1), i);
        }
        println!("{}", ans % MOD);
    }
}
