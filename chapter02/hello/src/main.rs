fn main() {
    println!("gcd of {} and {} is {}", 1071, 1029, gcd(1071,1029));
}

fn gcd(mut n: u64, mut m: u64) -> u64{
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}