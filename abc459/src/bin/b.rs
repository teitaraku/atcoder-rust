use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut ans = String::from("");
    for i in 0..n {
        let ch = s[i].chars().nth(0).unwrap();
        if ['a', 'b', 'c'].contains(&ch) {
            ans = ans + "2";
        } else if ['d', 'e', 'f'].contains(&ch) {
            ans = ans + "3";
        } else if ['g', 'h', 'i'].contains(&ch) {
            ans = ans + "4";
        } else if ['j', 'k', 'l'].contains(&ch) {
            ans = ans + "5";
        } else if ['m', 'n', 'o'].contains(&ch) {
            ans = ans + "6";
        } else if ['p', 'q', 'r', 's'].contains(&ch) {
            ans = ans + "7";
        } else if ['t', 'u', 'v'].contains(&ch) {
            ans = ans + "8";
        } else if ['w', 'x', 'y', 'z'].contains(&ch) {
            ans = ans + "9";
        }
    }
    println!("{}", ans);
}
