use std::ops::Mul;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let inp = input.lines().next().unwrap().unwrap();

    let sol = if inp.len() > 10 {
        solve_estimate(ImpreciseBigNum {
            n: inp[0..10].parse().unwrap(),
            e: inp.len()
        })
    } else {
        solve_exact(inp.parse().unwrap())
    };

    writeln!(&mut w, "{}", sol).unwrap();

    Ok(())
}

fn solve_exact(inp: usize) -> usize {
    let mut c = 1;
    let mut f = 1;
    while f != inp {
        c += 1;
        f *= c;
    }

    c
}

fn solve_estimate(inp: ImpreciseBigNum) -> usize {
    let mut c = 1;
    let mut f = ImpreciseBigNum {
        n: 1.0,
        e: 0
    };

    while f.e < inp.e {
        c += 1;
        f = f * ImpreciseBigNum {
            n: c as f64,
            e: 0
        }
    }

    c - 1
}

#[derive(Debug)]
struct ImpreciseBigNum {
    n: f64,
    e: usize
}

impl Mul for ImpreciseBigNum {
    type Output = ImpreciseBigNum;
    fn mul(self, other: Self) -> Self { 
        let mut res = ImpreciseBigNum {
            n: self.n * other.n,
            e: self.e + other.e
        };

        while res.n >= 10.0 {
            res.n /= 10.0;
            res.e += 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inversefactorial_sampleinputs() {
        for mut file in std::fs::read_dir("input/inversefactorial")
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
