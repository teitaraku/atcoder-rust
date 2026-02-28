use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    }
    let max_len = s.iter().map(|x| x.len()).max().unwrap();
    for x in s {
        let num_dots = (max_len - x.len()) / 2;
        let dots = ".".repeat(num_dots);
        println!("{}{}{}", dots, x, dots);
    }
}
