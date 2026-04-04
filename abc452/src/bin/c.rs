use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut ab = vec![];
    for _ in 0..n {
        input! {
            ai: usize,
            bi: usize,
        }
        ab.push((ai, bi));
    }
    input! {
        m: usize,
        s: [String;m],
    }
    for j in 0..m {
        let mut matched = false;
        for i in 0..n {
            matched = s.iter().filter(|ss| ss.len() == ab[i].0).any(|ss| {
                ss.chars().nth(ab[i].1 - 1).unwrap_or(' ') == s[j].chars().nth(i).unwrap()
            });
            if !matched {
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
