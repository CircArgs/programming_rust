use std::io;

fn main() {
    println!("Enter a space-separated line of numbers:");
    let mut numbers = String::new();
    io::stdin()
        .read_line(&mut numbers)
        .expect("Failed to read line!");
    let mut numbers: Vec<u64> = numbers
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert!(numbers.len() > 1, "Must enter two or more numbers!");
    let mut ret_gcd = numbers.pop().unwrap();
    let mut cur = numbers.pop();
    while cur.is_some() {
        let temp = cur.unwrap();
        ret_gcd = gcd(ret_gcd, temp);
        cur = numbers.pop();
    }

    println!("The gcd of these numbers is {}", ret_gcd);
}

fn gcd(a: u64, b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    let (a, b) = if b > a { (b, a) } else { (a, b) };
    if a % b == 0 {
        b
    } else {
        let mut r = a % b;
        let mut temp_r = b % r;
        while temp_r != 0 {
            let local_r = r % temp_r;
            r = temp_r;
            temp_r = local_r;
        }
        r
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
