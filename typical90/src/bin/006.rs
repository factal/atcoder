use proconio::input;

fn main() {
    input! {
        n: u32,
        k: u32,
        s: String,
    }

    let mut table = vec![];

    for (i, c) in s.chars().enumerate() {
        table.push(IndexedChar::new(i as u32, c as u32, c));
    }

    table.sort_by(|a, b| a.weight.cmp(&b.weight));

    let mut ans = String::new();
    let mut cur_index  = 0;

    for item in &table {
        if item.index + k - 1 >= s.len() as u32 {
            continue;
        } else if (ans.len() as u32) < k && item.index >= cur_index {
            ans.push(item.c);
            cur_index = item.index;
        } else {
            continue;
        }
    }
    
    for item in table {
        println!("{} | {} | {}", item.c, item.index, item.weight);
    }

    println!("{}", ans);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct IndexedChar {
    index: u32,
    weight: u32,
    c: char,
}

impl IndexedChar {
    pub fn new(index: u32, weight: u32, c: char) -> Self {
        Self { index, weight, c }
    }
}