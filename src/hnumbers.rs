use std::error::Error;
use std::io::{self, BufRead, Write};


/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {

    // Steps:
    // 1. Generate a LL of all H-numbers
    // 2. Sieve method for filtering the list to obtain all primes
    // 3. Using this list generate list of all semi-primes
    // 4. Answer queries

    let mut primes: Vec<usize> = vec![];
    let mut sieve: Vec<bool> = vec![true; 250_001]; // 250_000 * 4 + 1 = 1_000_001
    for i in 1..sieve.len() {
        let p = i * 4 + 1;
        if sieve[i] {
            primes.push(p);
            let mut e = p * p;
            while e <= 1_000_001 {
                sieve[(e - 1) >> 2] = false;
                e += 4*p;
            }
        }
    }

    // Generate all H-semi-primes
    let mut semi_primes = vec![];
    'outerloop: for (i, p1) in primes.iter().enumerate() {
        for p2 in primes[i..].iter() {
            let x = p1 * p2;
            if x > 1_000_001 {
                continue 'outerloop;
            }
            semi_primes.push(x);
        }
    }

    semi_primes.sort();
    semi_primes.dedup();

    for query in input.lines().map(|x| x.unwrap()).map(|x| x.parse::<usize>().unwrap()) {
        if query == 0 {
            break;
        }

        writeln!(&mut w, "{} {}", query, binary_search(&semi_primes, query)).unwrap()
    }

    Ok(())
}

fn binary_search(inp: &Vec<usize>, search: usize) -> usize {

    let mut min = 0;
    let mut max = inp.len() - 1;

    while max - min > 1 {
        let mid = (max + min) / 2;

        if inp[mid] < search {
            min = mid;
        } else if inp[mid] > search {
            if mid == 0 {
                return 0
            } else {
                max = mid - 1;
            }
        } else if inp[mid] == search {
            return mid + 1;
        }
    }

    if inp[max] <= search {
        max + 1
    } else if inp[min] <= search {
        min + 1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hnumbers_sampleinputs() {
        for mut file in std::fs::read_dir("input/hnumbers")
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
                "file: {:?}",
                file
            );
        }
    }
}
