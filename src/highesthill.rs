use std::error::Error;
use std::io::{self, BufRead, Write};
use std::cmp;

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let _first = lines.next();

    let mountains: Vec<u64> = lines.next().unwrap().split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
    
    // Create lookup left / right
    let mut lookup_left = vec![0u64; mountains.len()];
    lookup_left[0] = mountains[0];

    for (i, &x) in mountains.iter().enumerate().skip(1) {
        lookup_left[i] = if x >= mountains[i - 1] {
            lookup_left[i - 1]
        } else {
            x
        }
    }

    let mut lookup_right = vec![0u64; mountains.len()];
    lookup_right[0] = mountains[mountains.len() - 1];

    for (i, &x) in mountains.iter().rev().enumerate().skip(1) {
        lookup_right[i] = if x >= mountains[mountains.len() - 1 - i + 1] {
            lookup_right[i - 1]
        } else {
            x
        }
    }

    lookup_right.reverse();

    let mut max = 0;
    for (i, &x) in mountains.iter().enumerate() {
        if i == 0 || i == mountains.len() - 1 {
            continue;
        }

        let min_left = lookup_left[i];
        let min_right = lookup_right[i];
        
        if min_left <= x && min_right <= x {
            max = cmp::max(max, cmp::min(x - min_left, x - min_right));
        }
    }

    writeln!(&mut w, "{}", max).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highesthill_sampleinputs() {
        for mut file in std::fs::read_dir("input/highesthill")
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
