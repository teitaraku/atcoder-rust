use proconio::input;

/// ABC087B - Coins
///
/// まずは大きい金額の硬貨から最小の枚数で支払う場合を考える
/// その後、100円硬貨を50円に置き換えていき、置き換えられなくなったら500円硬貨を100円に置き換える
/// この操作を繰り返して組み合わせを求める
fn main() {
    input! {
        a: i32, // num of 500 coins
        b: i32, // num of 100 coins
        c: i32, // num of 50 coins
        x: i32, // x=50n
    }
    let mut count = 0;
    // 初期値
    let mut l = std::cmp::min(x / 500, a);
    let mut m = std::cmp::min((x - l * 500) / 100, b);
    let mut n = std::cmp::min((x - l * 500 - m * 100) / 50, c);

    if 500 * l + 100 * m + 50 * n != x {
        println!("{}", 0);
        return;
    }

    loop {
        // 組み合わせ追加判定
        if a >= l && b >= m && c >= n {
            count = count + 1;
        }
        if 0 < m && n + 2 <= c {
            // 100円を50円に置き換え
            m = m - 1;
            n = n + 2;
        } else if 0 < l {
            // 500円を100円に置き換え
            l = l - 1;
            m = (x - 500 * l) / 100;
            n = (x - 500 * l) % 100 / 50;
        } else {
            // 置き換え不可で終了
            break;
        }
    }
    println!("{}", count);
}
