use std::error::Error;
use std::collections::VecDeque;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {

    let mut lines = input.lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut lines = lines.map(|l| l.unwrap()).map(|l| {
        let mut it = l.split(' ').map(|x| x.parse::<usize>().unwrap());
        (it.next().unwrap(), it.next().unwrap())
    });

    'mainloop: for _ in 0..t {
        let (c, b) = lines.next().unwrap();

        if b == 0 {
            writeln!(&mut w, "1").unwrap(); // If there are no connection then we can one-color the graph
            continue 'mainloop;
        }

        let mut graph: Vec<Vec<usize>> = vec![vec![]; c];

        for _ in 0..b {
            let (a, b) = lines.next().unwrap();
            graph[a].push(b);
            graph[b].push(a);
        }

        if check_bipartite(&graph, 0).0 {
            writeln!(&mut w, "2").unwrap();
            continue 'mainloop;
        }

        let mut found_coloring = false;
        for i in 1..(1 << c - 1) {
            let (bip1, tc1) = check_bipartite(&graph, i);
            let (bip2, tc2) = check_bipartite(&graph, !i);

            if bip1 && bip2 {
                // We found a three or four colored graph, check how many
                found_coloring = true;

                if !tc1 || !tc2 {
                    writeln!(&mut w, "3").unwrap();
                    continue 'mainloop;
                }
            }
        }

        if found_coloring {
            writeln!(&mut w, "4").unwrap();
        } else {
            writeln!(&mut w, "many").unwrap();
        }

    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Color {
    None, Red, Black
}

impl Color {
    fn next(&self) -> Self {
        match self {
            Color::None => panic!("Can't switch an uncolored color."),
            Color::Red => Color::Black,
            Color::Black => Color::Red
        }
    }
}

/// Mask filters away vertices contained in the mask
fn check_bipartite(graph: &Vec<Vec<usize>>, mask: u32) -> (bool, bool) {
    let mut color = vec![Color::None; graph.len()];
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut used_two_colors = false;

    for i in 0..graph.len() {
        if (mask >> i) & 1 == 1 {
            continue; // We skip this vertex
        }

        if color[i] == Color::None {
            queue.push_back(i);
            color[i] = Color::Red;

            while let Some(v) = queue.pop_front() {
                let c = color[v].clone();

                for &neighbour in &graph[v] {
                    if (mask >> neighbour) & 1 == 1 {
                        continue;
                    }

                    if color[neighbour] == c {
                        return (false, false);
                    }

                    if color[neighbour] == Color::None {
                        color[neighbour] = c.next();
                        used_two_colors = true;
                        queue.push_back(neighbour);
                    }
                }
            }
        }
    }

    return (true, used_two_colors);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapcolouring_sampleinputs() {
        for mut file in std::fs::read_dir("input/mapcolouring")
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
