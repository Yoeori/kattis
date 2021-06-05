use std::{
    error::Error,
    io::{self, BufRead},
};

// Kattis problem: chineseremainder
fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    for l in stdin.lock().lines().map(|l| l.unwrap()).skip(1) {
        let mut seq = l.split(' ');
        let (x, k) = test_case(
            seq.next().unwrap().parse()?,
            seq.next().unwrap().parse()?,
            seq.next().unwrap().parse()?,
            seq.next().unwrap().parse()?,
        );
        println!("{} {}", x, k)
    }

    Ok(())
}

fn test_case(a: i128, n: i128, b: i128, m: i128) -> (i128, i128) {
    let k = n * m;

    // Chinese remainder theorem
    // x = a (mod n) & x = b (mod m) => x = m * m^-1 in mod(n) * a + n * n^-1 in mod(m) * b
    (
        (m * modulus_inverse(m, n).unwrap() * a + n * modulus_inverse(n, m).unwrap() * b) % k,
        k,
    )
}

fn extended_gcd(mut x: i128, mut y: i128) -> (i128, i128, i128) {
    let (mut a0, mut a1, mut b0, mut b1) = (1, 0, 0, 1);

    while y != 0 {
        let (q, r) = (x / y, x % y);
        let (c, d) = (a0 - q * a1, b0 - q * b1);

        x = y;
        y = r;
        a0 = a1;
        a1 = c;
        b0 = b1;
        b1 = d;
    }

    (x, a0, b0)
}

fn modulus_inverse(m_i: i128, modulus: i128) -> Option<i128> {
    let (gcd, a, _) = extended_gcd(m_i, modulus);

    if gcd == 1 {
        Some((a % modulus + modulus) % modulus)
    } else {
        None
    }
}
