use proconio::input;

fn main() {
    input! {
        s: String,
    }
    // この順番でマッチさせれば手戻りなく進められるはず
    let patterns = [
        "dreameraser",
        "dreamerase",
        "dreamer",
        "dream",
        "eraser",
        "erase",
    ];
    let mut t = String::new();
    'outer: loop {
        for p in patterns {
            let temp = t.clone() + p;
            if s.starts_with(&temp) {
                t = temp;
                continue 'outer;
            }
        }
        // for でマッチするものがなければ抜ける
        break;
    }
    if s == t {
        println!("YES");
    } else {
        println!("NO");
    }
}
