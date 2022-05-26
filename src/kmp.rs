use std::cmp::max;
use std::rc::Rc;
use std::time::Instant;

fn max_pub_sub_string() {
    let src = "0abab";
    let v_src = src.chars().collect::<Vec<char>>();
    let mut dp = vec![vec![0; src.len()]; src.len()];
    let mut next_t = vec![0; src.len()];

    for i in 1..src.len() {
        for j in 1..i {
            if v_src[i] == v_src[j] {
                dp[i][j] = max(1, dp[i - 1][j - 1] + 1);
                next_t[i] = max(dp[i][j], next_t[i]);
            }
        }
    }
    // for i in dp.iter() {
    //     println!("{:?}", *i);
    // }
}
fn kmp() {
    let src = "aabba";
    let v_src = src.chars().collect::<Vec<char>>();
    let mut dp = vec![0; src.len() + 1];

    let mut j = 0;
    for i in 1..v_src.len() {
        // println!("----------");
        // println!("{:?}", dp);
        // println!("{:?}", (i, j));

        if v_src[i] == v_src[j] {
            dp[i] = dp[j] + 1;
            j += 1;
        } else {
            while dp[i] > 0 && v_src[i] != v_src[j] {
                j = dp[j - 1];
            }
            dp[i] = j
        }
        println!("{:?}", dp);
    }
}

fn main() {
    // let p2 = vec![kmp(), max_pub_sub_string()];
    let now = Instant::now();
    kmp();
    println!("{:?}", Instant::now().duration_since(now));
    let now = Instant::now();
    max_pub_sub_string();
    println!("{:?}", Instant::now().duration_since(now));
}
