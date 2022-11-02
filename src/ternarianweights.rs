use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap().parse().unwrap());
    let n = lines.next().unwrap();

    for _ in 0..n {
        let mut m = lines.next().unwrap();

        let mut left: Vec<u64> = vec![];
        let mut right: Vec<u64> = vec![];

        let mut modi = 1;
        let mut rest = 0;

        while m != 0 {
            if m % 3 == 0 {
                if rest > 0 {
                    rest = 0;
                    right.push(modi);
                }
            } else if m % 3 == 1 {
                if rest > 0 {
                    left.push(modi);
                } else {
                    right.push(modi);
                }
            } else if m % 3 == 2 {
                if rest == 0 {
                    left.push(modi);
                }
                rest += 1;
            }
    
            m /= 3;
            modi *= 3;
        }

        if rest > 0 {
            right.push(modi);
        }

        left.reverse();
        right.reverse();

        writeln!(&mut w, "left pan: {}", left.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")).unwrap();
        writeln!(&mut w, "right pan: {}", right.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")).unwrap();
        writeln!(&mut w, "").unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ternarianweights_sampleinputs() {
        for mut file in std::fs::read_dir("input/ternarianweights")
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
