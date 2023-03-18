use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    
    let mut a = a;
    a.sort();
    let mut b = b;
    b.sort();

    let mut ans = 0;

    for i in 0..n {
        ans += (a[i] - b[i]).abs();
    }

    println!("{}", ans);
}
