use std::error::Error;
use std::collections::HashSet;
use std::cmp;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let n: usize = lines.next().unwrap().parse().unwrap();

    let members: Vec<(u64, Vec<usize>)> = lines.take(n).map(|line| {
        let mut ns = line.split(' ').map(|x| x.parse::<usize>().unwrap());
        (ns.next().unwrap() as u64, ns.skip(1).collect())
    }).collect();

    // We need to find the root member
    let mut root_possibilities: HashSet<usize> = (0..n).collect();
    for (_, subordinates) in &members {
        for subordinate in subordinates {
            root_possibilities.remove(subordinate);
        }
    }

    let root = *root_possibilities.iter().next().unwrap();

    let result = dfs(&members, root);
    writeln!(&mut w, "{}", cmp::min(result.pick, result.skip_below)).unwrap();

    Ok(())
}

struct DFSResult {
    pick: u64,
    skip_below: u64,
    skip_above: u64
}

fn dfs(tree: &Vec<(u64, Vec<usize>)>, cur: usize) -> DFSResult {
    let mut pick = tree[cur].0;
    let mut skip_above = 0;
    let mut promote = std::u64::MAX;

    if tree[cur].1.is_empty() {
        return DFSResult {
            pick,
            skip_below: promote,
            skip_above
        }
    }

    for &neighbour in &tree[cur].1 {
        let res = dfs(tree, neighbour);
        pick += res.skip_above;
        skip_above += cmp::min(res.pick, res.skip_below);
        promote = cmp::min(promote, cmp::max(0, res.pick.checked_sub(res.skip_below).unwrap_or(0)))
    }
 
    DFSResult {
        pick,
        skip_below: skip_above + promote,
        skip_above
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn citrusintern_sampleinputs() {
        for mut file in std::fs::read_dir("input/citrusintern")
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
