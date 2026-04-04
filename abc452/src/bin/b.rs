use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    for j in 1..=h {
        for i in 1..=w {
            if j == 1 || j == h || i == 1 || i == w {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
