use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [[usize; 2]; n]
    }

    // convex hull
    let mut p = p;
    p.sort();
    let asd

    for i in 0..n {
        while ch.len() >= 2 {
            let v1 = [ch[ch.len() - 1][0] - ch[ch.len() - 2][0], ch[ch.len() - 1][1] - ch[ch.len() - 2][1]];
            let v2 = [p[i][0] - ch[ch.len() - 1][0], p[i][1] - ch[ch.len() - 1][1]];
            if v1[0] * v2[1] - v1[1] * v2[0] > 0 {

    ch point in ch {
        println!("{} {}", point[0], point[1]);
    }
}
