use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize);n],
        m: usize,
        s: [String;m],
    }
    // 計算量削減のための前処理
    // 次の辞書を作っておくことで、制約を満たす文字列が存在するかどうかわかるようにする
    // (長さ, 位置) -> 該当する文字列
    let mut char_set: HashMap<(usize, usize), HashSet<char>> = HashMap::new();
    for ss in &s {
        for (pos, ch) in ss.chars().enumerate() {
            char_set.entry((ss.len(), pos)).or_default().insert(ch);
        }
    }
    let empty: HashSet<char> = HashSet::new();
    // M個の文字列に対して上からチェック
    for j in 0..m {
        // 脊椎の長さは N でなければならない
        if s[j].len() != n {
            println!("No");
            continue;
        }
        let mut matched = true;
        // i 番目に文字列長 ai で bi 文字目が j 番目の文字列と一致するかを調べる
        for i in 0..n {
            let si = s[j].chars().nth(i).unwrap_or('!');
            let map = char_set.get(&(ab[i].0, ab[i].1 - 1)).unwrap_or(&empty);
            if !map.contains(&si) {
                matched = false;
                break;
            }
        }
        if matched {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
