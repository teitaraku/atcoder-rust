use proconio::input;

fn main() {
    input! {
        n: i32,
        points: [(i32, i32, i32);n],
    }
    let mut is_succeeded = true;
    let (mut tp, mut xp, mut yp) = (0, 0, 0);
    for p in points {
        let (ti, xi, yi) = p;
        let td = ti - tp;
        let xd = xi - xp;
        let yd = yi - yp;
        let distance = xd.abs() + yd.abs();
        // 距離的に到達不可
        if td < distance {
            is_succeeded = false;
            break;
        }
        // 同じ場所に留まる必要がないか
        if (distance % 2) != (td % 2) {
            // 距離が偶数の時、奇数の時間では到達できない
            // 距離が奇数の時、偶数の時間では到達できない
            is_succeeded = false;
            break;
        }
        tp = ti;
        xp = xi;
        yp = yi;
    }
    if is_succeeded {
        println!("Yes");
    } else {
        println!("No");
    }
}
