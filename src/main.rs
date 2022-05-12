use std::str::from_utf8;

fn main() {
    let p2 = "sss".chars().collect::<Vec<char>>()[0];
    // let p3 = "sss".to_string().as_ptr();
    let p3 = "sss".as_bytes();
    let p4 = from_utf8(p3).unwrap();
    println!("{:?}", p4);
}
