use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    for i in 1..=h {
        for j in 1..=w {
            if i == 1 || i == h || j == 1 || j == w {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
