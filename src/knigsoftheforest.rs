use std::collections::{HashMap, BinaryHeap};
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Part {
    strength: usize,
    first_year: usize,
    is_karl: bool
}

impl Part {
    fn from_line(line: &str) -> Self {
        let mut line = line.split(' ').map(|x| x.parse::<usize>().unwrap());
        Part {
            first_year: line.next().unwrap(),
            strength: line.next().unwrap(),
            is_karl: false
        }
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let first_line = lines.next().unwrap();
    let mut first_line = first_line.split(' ').map(|x| x.parse::<usize>().unwrap());
    let (k, n) = (first_line.next().unwrap(), first_line.next().unwrap());

    let mut participants: BinaryHeap<Part> = BinaryHeap::new();
    let mut future_participants: HashMap<usize, Part> = HashMap::new();

    let mut karl = Part::from_line(&lines.next().unwrap());
    karl.is_karl = true;
    if karl.first_year == 2011 {
        participants.push(karl);
    } else {
        future_participants.insert(karl.first_year, karl);
    }

    for _ in 0..(n + k - 2) {
        let part = Part::from_line(&lines.next().unwrap());
        if part.first_year == 2011 {
            participants.push(part);
        } else {
            future_participants.insert(part.first_year, part);
        }
    }

    for i in 0..n {
        // Pop greatest
        let part = participants.pop().unwrap();
        if part.is_karl {
            writeln!(&mut w, "{}", i + 2011).unwrap();
            return Ok(());
        }

        // Add next participant
        if i != n - 1 {
            participants.push(future_participants.remove(&(i + 2011 + 1)).unwrap());
        }
    }

    writeln!(&mut w, "unknown").unwrap();


    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knigsoftheforest_sampleinputs() {
        for mut file in std::fs::read_dir("input/knigsoftheforest")
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
