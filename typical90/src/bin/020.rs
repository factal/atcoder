use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u32,
        c: u64,
    }
    
    let tmp = c.pow(b);

    if a < tmp {
        println!("Yes");
    } else {
        println!("No");
    }
}
