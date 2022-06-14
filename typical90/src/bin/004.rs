use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h]
    }

    let mut b = vec![0i32;h];
    let mut c = vec![0i32;w];

    for i in 0..h {
        for j in 0..w {
            b[i] += a[i][j];
            c[j] += a[i][j];
        }
    }

    for i in 0..h{
        let mut l:Vec<String>=vec![];
        for j in 0..w{
            l.push((b[i]+c[j]-a[i][j]).to_string());
        }
        println!("{}",l.join(" "));
    }
}