use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Color {
    Red,
    Black,
    None,
    Both,
}

impl Color {
    fn next(&self) -> Self {
        match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
            Color::None => panic!("No next known"),
            Color::Both => Color::Both,
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    neighbours: Vec<usize>
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut input_reader = input.lines().map(|l| {
        let l = l.unwrap();
        let mut l = l.split(' ');
        (
            l.next().unwrap().parse::<usize>().unwrap(),
            l.next().unwrap().parse::<usize>().unwrap(),
        )
    });

    let (n, m) = input_reader.next().unwrap();
    let mut nodes: Vec<Node> = (0..n)
        .map(|_| Node {
            neighbours: vec![],
        })
        .collect();

    for _ in 0..m {
        let (a, b) = input_reader.next().unwrap();
        (*nodes.get_mut(a - 1).unwrap()).neighbours.push(b - 1);
        (*nodes.get_mut(b - 1).unwrap()).neighbours.push(a - 1);
    }

    writeln!(&mut w, "{}", count_connected_components(&nodes)).unwrap();

    Ok(())
}

fn count_connected_components(nodes: &Vec<Node>) -> usize {
    let mut visited = vec![false; nodes.len()];
    let mut color = vec![Color::None; nodes.len()];
    let mut found_both_colors = false;

    let mut count = 0;

    for i in 0..nodes.len() {
        if !visited[i] {
            count += 1;
            color[i] = Color::Black;
            if dfs(i, nodes, &mut visited, &mut color) {
                found_both_colors = true;
            }
        }
    }
    
    if found_both_colors {
        count - 1
    } else {
        count
    }
    
}

fn dfs(node: usize, nodes: &Vec<Node>, visited: &mut Vec<bool>, color: &mut Vec<Color>) -> bool {
    let mut to_visit = vec![node];
    let mut found_both = false;

    while let Some(cur) = to_visit.pop() {
        if !visited[cur] {
            visited[cur] = true;
            for &neighbour in nodes[cur].neighbours.iter() {
                color[neighbour] = if color[cur] == color[neighbour] {
                    found_both = true;
                    Color::Both
                } else {
                    color[cur].next()
                };

                if !visited[neighbour] {
                    to_visit.push(neighbour);
                }
            }
        }
    }

    found_both
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hoppers_sampleinputs() {
        for mut file in std::fs::read_dir("input/hoppers")
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
