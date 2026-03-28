use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        q: usize,
    }

    let mut tree_map: BTreeMap<usize, usize> = BTreeMap::new();
    let mut total: usize = 0;

    for _ in 0..q {
        input! {
            c: usize,
            h: usize,
        }

        if c == 1 {
            *tree_map.entry(h).or_insert(0) += 1;
            total += 1;
        } else {
            // h+1 以上のキーを切り出し、残りは捨てる
            let keep = tree_map.split_off(&(h + 1));
            // 削除側を数えた方が効率的。残った方を数えると、次ループ以降も数えることになってしまう
            let removed: usize = tree_map.values().sum();
            total -= removed;
            tree_map = keep;
        }

        println!("{}", total);
    }
}
