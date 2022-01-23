use std::error::Error;
use std::io::{self, BufRead, Write};
use std::collections::HashSet;

struct Node {
    forced_edge: Option<usize>,
    neighbours: Vec<usize>
}

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut input_reader = input.lines().map(|l| {
        let l = l.unwrap();
        let mut l = l.split(' ');
        (l.next().unwrap().parse::<isize>().unwrap(), l.next().unwrap().parse::<usize>().unwrap())
    });

    let (n, m) = input_reader.next().unwrap();

    let mut nodes: Vec<Node> = (1..=n).map(|_| Node {
        forced_edge: None,
        neighbours: vec![]
    }).collect();

    for _ in 0..m {
        let (a, b) = input_reader.next().unwrap();
        if a > 0 {
            (*nodes.get_mut((a - 1) as usize).unwrap()).neighbours.push(b);
        } else {
            (*nodes.get_mut((-a - 1) as usize).unwrap()).forced_edge = Some(b);
        }
    }

    let (mut visited, found_final) = explore(&nodes, 1);
    let mut rest_nodes: HashSet<usize> = HashSet::new();
    if let Some(found_final) = found_final {
        rest_nodes.insert(found_final);
    }

    for n1 in visited.clone() {
        for n2 in nodes.get(n1 - 1).unwrap().neighbours.iter() {
            if !visited.contains(n2) {
                let (new_visited, found_final) = explore(&nodes, *n2);
                if let Some(found_final) = found_final {
                    rest_nodes.insert(found_final);
                }
                visited.extend(new_visited);
            }
        }
    }

    writeln!(&mut w, "{}", rest_nodes.len()).unwrap();

    Ok(())
}

fn explore(nodes: &Vec<Node>, mut pos: usize) -> (HashSet<usize>, Option<usize>) {
    let mut visited: HashSet<usize> = HashSet::new();
    visited.insert(pos);

    while let Some(Node { forced_edge: Some(new_pos), .. }) = nodes.get(pos - 1) {
        if visited.contains(new_pos) {
            // Loop, return without final position
            return (visited, None);
        }

        pos = *new_pos;
        visited.insert(pos);
    }

    (visited, Some(pos))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn faultyrobot_sampleinputs() {
        for mut file in std::fs::read_dir("input/faultyrobot")
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
                std::fs::read_to_string(&file).unwrap().trim()
            );
        }
    }
}
