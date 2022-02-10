use std::collections::HashSet;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Connection {
    Edge(usize), Arc(usize)
}

impl Connection {
    fn node(&self) -> usize {
        match self {
            Connection::Edge(n) => *n,
            Connection::Arc(n) => *n
        }
    }

}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut input_reader = input.lines().map(|l| {
        let l = l.unwrap();
        let mut l = l.split(' ');
        (l.next().unwrap().parse::<usize>().unwrap(), l.next().unwrap().parse::<usize>().unwrap())
    });

    let (n, m) = input_reader.next().unwrap();
    let mut nodes: Vec<HashSet<Connection>> = vec![HashSet::new(); n];

    // Create graph
    for _ in 0..m {
        let (a, b) = input_reader.next().unwrap();
        (*nodes.get_mut(a-1).unwrap()).insert(Connection::Edge(b-1));
        (*nodes.get_mut(b-1).unwrap()).insert(Connection::Edge(a-1));
    }

    let mut frontier: HashSet<usize> = HashSet::new();
    frontier.insert(0);

    while frontier.len() < n {
        let mut a = None;
        let mut b = None;

        // Could be improved a lot by keeping a bit more pointers, but since n & m are very limited this isn't really necessary.
        'outerfor: for &front in frontier.iter() {
            for edge in nodes.get(front).unwrap().iter() {
                if let Connection::Edge(neighbour) = edge {
                    if !frontier.contains(neighbour) {
                        a = Some(front);
                        b = Some(*neighbour);
                        break 'outerfor;
                    }
                }
            }

        }

        if let Some(a) = a {
            let b = b.unwrap();

            nodes.get_mut(a).unwrap().remove(&Connection::Edge(b));
            nodes.get_mut(b).unwrap().remove(&Connection::Edge(a));
            nodes.get_mut(a).unwrap().insert(Connection::Arc(b));
            frontier.insert(b);

            if let Some(path) = dfs(&nodes, &mut HashSet::new(), b, a) {
                for (i, conn) in path {
                    if let Connection::Edge(j) = conn {
                        nodes.get_mut(i).unwrap().remove(&Connection::Edge(j));
                        nodes.get_mut(j).unwrap().remove(&Connection::Edge(i));

                        nodes.get_mut(i).unwrap().insert(Connection::Arc(j));
                        frontier.insert(j);
                    }
                }
            } else {
                // Unreachable, thus this node cannot be inserted
                frontier.remove(&b); // For some future me or person looking at this code: _this line was the source of all my pain_
                break;
            }
        } else {
            // There's no a-b pair but we haven't added all notes to frontier yet
            break;
        }   
    }

    if frontier.len() < n {
        writeln!(&mut w, "NO").unwrap();
    } else {
        writeln!(&mut w, "YES").unwrap();
        for (a, node) in nodes.iter().enumerate() {
            for conn in node {
                if let Connection::Arc(b) = conn {
                    writeln!(&mut w, "{} {}", a + 1, b + 1).unwrap();
                } else if let Connection::Edge(b) = conn {
                    if *b > a {
                        writeln!(&mut w, "{} {}", a + 1, b + 1).unwrap();
                    }
                }
            }
        }
    }

    // Main idea of algorithm:
    // 1. Find edge s.t. a in frontier and b is not, turn edge into arch s.t. a -> b
    // 2. Find path from a to b (dfs), and set all edges along paths to be arc, add all found nodes to frontier
    // 3. Repeat step 1, until all nodes are in frontier or no valid edge can be found.

    Ok(())
}

fn dfs(nodes: &Vec<HashSet<Connection>>, visited: &mut HashSet<usize>, from: usize, to: usize) -> Option<Vec<(usize, Connection)>> {
    visited.insert(from);

    for conn in nodes.get(from).unwrap() {
        if conn.node() == to {
            return Some(vec![(from, conn.clone())]);
        } else if !visited.contains(&conn.node()) {
            if let Some(mut path) = dfs(nodes, visited, conn.node(), to) {
                path.push((from, conn.clone()));
                return Some(path);
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn onewayroads_sampleinputs() {
        for mut file in std::fs::read_dir("input/onewayroads")
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
            println!("{:?}:\n{}\n", file, std::str::from_utf8(&output_writer).unwrap().trim());
            // assert_eq!(
            //     std::str::from_utf8(&output_writer).unwrap().trim(),
            //     std::fs::read_to_string(&file).unwrap().trim(),
            //     "file: {:?}",
            //     file
            // );
        }
    }
}
