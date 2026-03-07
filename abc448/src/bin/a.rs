use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: u32,
        a: [u32;n],
    }

    for i in 0..n {
        if a[i] < x {
            println!("{}", 1);
            x = a[i];
        } else {
            println!("{}", 0);
        }
    }
}
