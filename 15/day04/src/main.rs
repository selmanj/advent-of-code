const KEY: &str = "ckczppom";

fn main() {
    let mut num = 1;

    let mut found1: Option<i32> = None;
    let mut found2: Option<i32> = None;
    loop {
        let digest = md5::compute(format!("{}{}", KEY, num));
        let hex = format!("{:x}", digest);
        if hex.chars().take(5).all(|c| c == '0') && found1.is_none() {
            found1 = Some(num);
        }
        if hex.chars().take(6).all(|c| c == '0') && found2.is_none() {
            found2 = Some(num);
        }
        if found1.is_some() && found2.is_some() {
            break;
        }
        num += 1;
    }
    println!("{}", found1.unwrap());
    println!("{}", found2.unwrap());
}
