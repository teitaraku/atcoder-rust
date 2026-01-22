use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let remainder = a * b % 2;
    if remainder == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
