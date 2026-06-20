use proconio::input;

fn main() {
    input! {
        x: f32,
        y: f32,
    }
    let ratio = x / y;
    if ratio == 16.0 / 9.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
