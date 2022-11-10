use std::cmp;
use std::collections::BTreeSet;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());

    let mut takelook: Vec<usize> = vec![];

    for testcase in 0..(lines.next().unwrap().parse::<usize>().unwrap()) {
        // Create view from left and right
        let n = lines.next().unwrap().parse::<usize>().unwrap();
        let heights: Vec<usize> = lines.next().unwrap().split(' ').map(|x| x.parse().unwrap()).collect();
        let mut from_left: Vec<usize> = vec![heights[0]; heights.len()];
        let mut next_highest: Vec<usize> = vec![0; heights.len()];
        let mut from_right: Vec<usize> = vec![heights[n - 1]; heights.len()];
        let mut next_highest_set: BTreeSet<usize> = BTreeSet::new();
        next_highest_set.insert(heights[0]);

        for (i, &height) in heights.iter().enumerate().skip(1) {
            from_left[i] = cmp::max(from_left[i - 1], height);
            next_highest[i] = *next_highest_set.range(height..).next().unwrap_or(&0);
            next_highest_set.insert(height);
        }

        for (i, &height) in heights.iter().enumerate().rev().skip(1) {
            from_right[i] = cmp::max(from_right[i + 1], height);
        }

        for i in 1..(n-1) {
            if heights[i] < from_left[i - 1] && next_highest[i] < from_right[i + 1] {
                takelook.push(testcase + 1);
                break;
            }
        }
    }

    writeln!(&mut w, "{}", takelook.len()).unwrap();
    for look in takelook {
        writeln!(&mut w, "{}", look).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn findmyfamily_sampleinputs() {
        for mut file in std::fs::read_dir("input/findmyfamily")
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
