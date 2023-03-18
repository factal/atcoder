use proconio::input;

// union find
struct UF {
    p: Vec<usize>
}

impl UF {
    fn root(&mut self, x: usize) -> usize {
        if self.p[x] == x {
            x
        } else {
            self.p[x] = self.root(self.p[x]);
            self.p[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);

        if x == y {
            return;
        }

        self.p[y] = x;
    }
}

enum Query {
    Paint(usize, usize),
    Judge(usize, usize, usize, usize),
}

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut queries: Vec<Query> = vec![];

    for _ in 0..q {
        input! {
            t: usize
        }
        
        if t == 1 {
            input! {
                r: usize,
                c: usize
            }

            queries.push(Query::Paint(r, c));
        } else {
            input! {
                r1: usize,
                c1: usize,
                r2: usize,
                c2: usize
            }

            queries.push(Query::Judge(r1, c1, r2, c2));

        }
    }

    let mut uf = UF {
        p: (0..(h * w + 2)).collect()
    };

    let mut table = vec![vec![false; w+2]; h+2];

    for query in queries {
        match query {
            Query::Paint(r, c) => {
                table[r][c] = true;
                let i = r * w + c;
                if table[r-1][c] {
                    uf.unite(i, i - w - 2);
                } else if table[r+1][c] {
                    uf.unite(i, i + w + 2);
                } else if table[r][c-1] {
                    uf.unite(i, i - 1);
                } else if table[r-1][c] {
                    uf.unite(i, i + 1);
                }
            },
            Query::Judge(r1, c1, r2, c2) => {
                let i1 = (r1 - 1) * w + (c1 - 1);
                let i2 = (r2 - 1) * w + (c2 - 1);

                if uf.root(i1) == uf.root(i2) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
        }
    }
    for u in &uf.p {
        print!("{} ", u);
    }
}
