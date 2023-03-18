use proconio::input;


// find a special solution for ax + by = c (gcd(a, b) | c)
// return (x, y, d)
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (1, 0, a);
    }
    let (x, y, d) = ext_gcd(b, a % b);
    (y, x - a / b * y, d)
}

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }
    
    let g = gcd(b, c);

    let (x, y, d) = ext_gcd(b, c);

    let mut k = 

    print!("x: {}, y: {}", x, y);
}
