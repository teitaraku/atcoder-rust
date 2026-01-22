use proconio::input;

fn main() {
    input! {
        n: u32,
        an: [u32; n], // 2^32-1 > 10^9 のため u32 を利用する
    }
    let mut count = 0;
    let mut nums = an;
    loop {
        if nums.iter().map(|n| n % 2).any(|n| n > 0) {
            break;
        }
        nums = nums.iter().map(|n| n / 2).collect::<Vec<u32>>();
        count = count + 1;
    }
    println!("{}", count);
}
