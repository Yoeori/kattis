use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap().split(' ').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>());

    let _ = lines.next().unwrap()[0] as usize;

    // Collection containing (comp_time, team_time) tuples
    let mut times: Vec<(isize, isize)> = lines.next().unwrap().into_iter().zip(lines.next().unwrap()).filter(|&(_, t)| t != -1).collect();
    times.sort_by(|(_, t1), (_, t2)| t2.cmp(t1));

    writeln!(&mut w, "{}", unbounded_binary_search(1, std::usize::MAX, |to_check: usize| {
        let mut pointer = std::i128::MAX;
        for &(comp_time, team_time) in &times {
            pointer = std::cmp::min((team_time as i128) * (to_check as i128), pointer);
            pointer -= comp_time as i128;
        }

        if pointer < 0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    })).unwrap();

    Ok(())
}

fn unbounded_binary_search(mut min: usize, mut max: usize, comparitor: impl Fn(usize) -> Ordering) -> usize {
    while (max - min) > 1 {
        let mid = max / 2 + min / 2;

        match comparitor(mid) {
            Ordering::Less => max = mid,
            Ordering::Greater => min = mid + 1,
            Ordering::Equal => return mid
        }
    }

    if comparitor(min) == Ordering::Greater {
        max
    } else {
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implementationirregularities_sampleinputs() {
        for mut file in std::fs::read_dir("input/implementationirregularities")
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
