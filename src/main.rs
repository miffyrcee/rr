use std::{ops::Index, sync::OnceState};

fn kmp() {
    let src = "aabba";
    let v_src = src.chars().collect::<Vec<char>>();
    let mut dp = vec![0; src.len() + 1];

    let mut stack = Vec::new();
    for i in 0..src.len() {
        loop {
            if !stack.is_empty() && v_src[stack[stack.len() - 1]] == v_src[i] {
                stack.pop();
            }
        }
        stack.push(i)
    }
}

fn main() {}
