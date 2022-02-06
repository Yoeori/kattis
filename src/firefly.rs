use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap().unwrap().split(' ').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let (n, h) = (first_line[0], first_line[1]);

    let mut stalagmites = vec![]; // from floor
    let mut stalactites = vec![]; // from ceiling

    for _ in 0..(n/2) {
        stalagmites.push(lines.next().unwrap().unwrap().parse::<usize>().unwrap());
        stalactites.push(lines.next().unwrap().unwrap().parse::<usize>().unwrap());
    }

    stalagmites.sort();
    stalactites.sort();

    let mut height_map = vec![0; h];

    let mut cursor = 0;
    for height in 0..h {
        while cursor < stalagmites.len() && stalagmites[cursor] <= height {
            cursor += 1;
        }
        height_map[height] += stalagmites.len() - cursor;
    }

    let mut cursor = 0;
    for height in (0..h).rev() {
        while cursor < stalactites.len() && stalactites[cursor] <= (h - height - 1) {
            cursor += 1;
        }
        height_map[height] += stalactites.len() - cursor;
    }

    let min = height_map.iter().min().unwrap();
    let count = height_map.iter().filter(|&x| x == min).count();
    writeln!(&mut w, "{} {}", min, count).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn firefly_sampleinputs() {
        for mut file in std::fs::read_dir("input/firefly")
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