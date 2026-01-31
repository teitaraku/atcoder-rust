use proconio::input;

/// テストケースは通ったけどジャッジ結果にWAとTLEがあったので見直し中に時間切れ
fn main() {
    input! {
        t: usize,
    }
    let mut r = vec![];
    for _ in 0..t {
        input! {
            n_i: usize,
            r_i: [i32;n_i],
        }
        r.push(r_i);
    }
    for i in 0..t {
        let mut count = 0;
        let minimum = r[i].iter().min().unwrap_or(&0);
        let mut min_index: Vec<usize> = vec![];
        for j in 0..r[i].len() {
            if r[i][j] == *minimum {
                min_index.push(j);
            }
        }
        let mut moved = false;
        for x in 0..r[i].len() {
            let diff = min_index;
            if 0 < x {
                let diff = r[i][x] - r[i][x - 1];
                if 1 < diff {
                    let moves = diff - 1;
                    r[i][x] = r[i][x] - moves;
                    count += moves;
                    moved = true;
                }
            }
            if x + 1 < r[i].len() {
                let diff = r[i][x] - r[i][x + 1];
                if 1 < diff {
                    let moves = diff - 1;
                    r[i][x] = r[i][x] - moves;
                    count += moves;
                    moved = true;
                }
            }
        }
        println!("{}", count);
    }
}
