use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {

    let mut lines = input.lines().map(|x| x.unwrap());
    let n: usize = lines.next().unwrap().parse().unwrap();

    for query in lines.take(n).map(|x| x.parse::<u64>().unwrap()) {
        writeln!(&mut w, "{}", (8 * mod_pow(9, query - 1, 1000000007)) % 1000000007).unwrap();
    }

    // Too lazy to do my own research, please find me a recurrance relation!
    // dbg!((1..=9).filter(|x| !contains_nine(*x)).count());
    // dbg!((10..=99).filter(|x| !contains_nine(*x)).count());
    // dbg!((100..=999).filter(|x| !contains_nine(*x)).count());
    // dbg!((1000..=9999).filter(|x| !contains_nine(*x)).count());

    Ok(())
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
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

#[allow(dead_code)]
fn contains_nine(mut dig: usize) -> bool {
    while dig != 0 {
        if dig % 10 == 9 {
            return true;
        }
        dig /= 10;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nine_sampleinputs() {
        for mut file in std::fs::read_dir("input/nine")
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
