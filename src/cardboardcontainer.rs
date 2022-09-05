use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();
    let i = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    // writeln!(&mut w, "{}", );
    let mut min = std::usize::MAX;
    let devisors = devisors(i);
    dbg!(&devisors);

    for d1 in &devisors {
        for d2 in &devisors {
            if (i / d1) % d2 == 0 {
                let d3 = (i / d1) / d2;
                min = std::cmp::min(min, (d1 * d2) * 2 + (d1 * d3) * 2 + (d2 * d3) * 2);
            }
        }
    }

    writeln!(&mut w, "{}", min).unwrap();

    Ok(())
}

fn devisors(num: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in 1..=(std::cmp::max(num / 2, 1)) {
        if num % i == 0 {
            res.push(i);
        }
    }
    res.push(num);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cardboardcontainer_sampleinputs() {
        for mut file in std::fs::read_dir("input/cardboardcontainer")
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
