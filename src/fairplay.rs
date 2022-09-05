use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Debug, PartialEq, Eq)]
struct Player {
    billiards: isize,
    pool: isize
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { 
        self.billiards.cmp(&other.billiards).then_with(|| self.pool.cmp(&other.pool))
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { 
        Some(self.cmp(&other)) 
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap().split(' ').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>());
    let n = lines.next().unwrap()[0] as usize;

    if n % 2 != 0 {
        writeln!(&mut w, "impossible").unwrap();
        return Ok(())
    }

    let mut players = Vec::with_capacity(n);
    let mut billiard_total: isize = 0;
    let mut pool_total: isize = 0;
    for _ in 0..n {
        let line = lines.next().unwrap();
        players.push(Player {
            billiards: line[0],
            pool: line[1]
        });
        billiard_total += line[0];
        pool_total += line[1];
    }

    let exp_billiard = billiard_total / (n as isize / 2);
    let exp_pool = pool_total / (n as isize / 2);

    players.sort();

    for i in 0..(n / 2) {
        if !(players[i].billiards + players[n-i-1].billiards == exp_billiard &&  players[i].pool + players[n-i-1].pool == exp_pool) {
            writeln!(&mut w, "impossible").unwrap();
            return Ok(())
        }
    }

    writeln!(&mut w, "possible").unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fairplay_sampleinputs() {
        for mut file in std::fs::read_dir("input/fairplay")
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
