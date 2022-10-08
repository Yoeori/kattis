use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

const ASCII: &str = include_str!("timebomb.txt");

fn test_number(input: &Vec<String>, n: usize, i: usize) -> bool {
    for (k, line) in input.iter().enumerate() {
        if !(line[(i * 4)..((i * 4) + 3)] == ASCII[(40*k + n * 4)..(40*k + n * 4 + 3)]) {
            return false;
        }
    }
    true
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let lines = input.lines().map(|x| x.unwrap());

    let text: Vec<String> = lines.take(5).collect();
    let n = (text[0].len() + 1) / 4;

    let mut num = 0;
    'outerloop: for i in 0..n {
        num *= 10;
        for test in 0..10 {
            if test_number(&text, test, i) {
                num += test;
                continue 'outerloop;
            }
        }
    
        writeln!(&mut w, "BOOM!!").unwrap();
        return Ok(())
    }

    if num % 6 == 0 {
        writeln!(&mut w, "BEER!!").unwrap();
    } else {
        writeln!(&mut w, "BOOM!!").unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timebomb_sampleinputs() {
        for mut file in std::fs::read_dir("input/timebomb")
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
