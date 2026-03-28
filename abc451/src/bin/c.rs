use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut command = vec![];
    let mut height = vec![];
    for _ in 0..q {
        input! {
            ci: usize,
            hi: usize,
        }
        command.push(ci);
        height.push(hi);
    }

    let mut trees = vec![];
    for i in 0..q {
        if command[i] == 1 {
            trees.push(height[i]);
        } else {
            trees.retain(|&h| height[i] < h);
        }
        println!("{}", trees.len());
    }
}
