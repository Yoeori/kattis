use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

// Hashing constants
const P: i64 = 31;
const M: i64 = 1_000_000_009;

fn mod_pow(mut base: i64, mut exp: i64, modulus: i64) -> i64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn calc(i: usize, b: u32) -> i64 {
    ((mod_pow(P, i as i64, M) % M) * (b as i64)) % M
}

fn hash(word: &Vec<u32>) -> i64 {
    word.iter()
        .enumerate()
        .fold(0, |acc, (i, &b)| ((acc + calc(i, b)) % M))
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let _: usize = lines.next().unwrap().parse().unwrap();

    let d1: Vec<u32> = to_delta_vec(lines.next().unwrap().split(' ').map(|x| x.parse::<u32>().unwrap()).collect());
    let d2: Vec<u32> = to_delta_vec(lines.next().unwrap().split(' ').map(|x| x.parse::<u32>().unwrap()).collect());

    // We hash d1, then go through d2 and update the hash for each shift
    let d1_hash = hash(&d1);
    let mut d2_hash = hash(&d2);

    for i in (0..d2.len()).rev() {
        // Remove the last clock handle
        d2_hash -= calc(d2.len() - 1, d2[i]);
        d2_hash %= M;

        // Update hash
        d2_hash *= P;
        d2_hash %= M;

        // Add the last clock handle back in
        d2_hash += calc(0, d2[i]);
        d2_hash = (d2_hash + M) % M;

        if d2_hash == d1_hash {
            writeln!(&mut w, "possible").unwrap();
            return Ok(())
        }
    }

    writeln!(&mut w, "impossible").unwrap();

    Ok(())
}

fn to_delta_vec(mut inp: Vec<u32>) -> Vec<u32> {
    let mut res = Vec::with_capacity(inp.len() + 1);
    inp.sort();

    for w in inp.windows(2) {
        res.push(w[1] - w[0]);
    }

    res.push(360_000 - inp[inp.len() - 1] + inp[0]);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clockpictures_sampleinputs() {
        for mut file in std::fs::read_dir("input/clockpictures")
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
