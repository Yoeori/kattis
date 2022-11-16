use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

const BASES: &[u32] = &[2, 8, 10, 16];

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    for inp in lines.take(n) {
        let mut primes = 0;
        let poss: Vec<u64> = BASES.iter().map(|base| str_to_digit_with_base(&inp, *base)).filter(|x| x.is_some()).map(|x| x.unwrap()).collect();

        for &n in poss.iter() {
            if is_prime(n) {
                primes += 1;
            }
        }

        let div = gcd(primes.clone(), poss.len() as u64);
        if div != 0 {
            writeln!(&mut w, "{}/{}", primes / div, (poss.len() as u64) / div).unwrap();
        } else {
            writeln!(&mut w, "0/1").unwrap();
        }
    }

    Ok(())
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }

    a
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    let max = ((n as f64).sqrt() as u64) + 1;
    for x in 2..=std::cmp::min(n, max) {
        if n != x && n % x == 0 {
            return false;
        }
    }
    return true;
}

fn str_to_digit_with_base(input: &str, base: u32) -> Option<u64> {
    let mut res = 0;
    for c in input.chars() {
        res *= base as u64;
        if let Some(dig) = c.to_digit(base) {
            res += dig as u64;
        } else {
            return None
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes2_sampleinputs() {
        for mut file in std::fs::read_dir("input/primes2")
            .unwrap()
            .filter(|f| f.is_ok() && f.as_ref().unwrap().path().extension().unwrap() == "in")
            .map(|f| f.unwrap().path())
        {
            let mut output_writer: Vec<u8> = Vec::new();
            solve(
                std::fs::read_to_string(&file).unwrap().as_bytes(),
                &mut output_writer,
            )
            .unwrap();
            file.set_extension("ans");
            assert_eq!(
                std::str::from_utf8(&output_writer).unwrap().trim(),
                std::fs::read_to_string(&file).unwrap().trim(),
                "file: {:?}", &file
            );
        }
    }
}
