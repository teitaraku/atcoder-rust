use proconio::input;

fn main() {
    input! {
        input: String,
    }
    let squares: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    println!("{}", squares.iter().sum::<u32>());
}
