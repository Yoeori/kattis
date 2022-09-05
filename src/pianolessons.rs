use std::error::Error;
use std::io::{self, BufRead, Write};
use std::cmp;

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());

    let first_line = lines.next().unwrap();
    let mut first_line = first_line.split(' ');

    let (n, m) = (first_line.next().unwrap().parse::<usize>().unwrap(), first_line.next().unwrap().parse::<usize>().unwrap());

    // Adjecency matrix, 0..n students, n..(n+m) timeslots, (n+m) source, (n+m)+1 sink
    let mut res_graph: Vec<Vec<i16>> = vec![vec![0; (n + m) + 2]; (n + m) + 2];

    for i in 0..n {
        res_graph[n + m][i] = 1;
    }

    for i in 0..m {
        res_graph[n + i][n + m + 1] = 1;
    }

    for (i, line) in lines.take(n).enumerate() {
        let xs = line.split(' ').map(|x| x.parse::<usize>().unwrap());
        for x in xs.skip(1) {
            res_graph[i][n + (x-1)] = 1;
        }
    }

    writeln!(&mut w, "{}", min_flow(&mut res_graph, n + m, n + m + 1)).unwrap();

    Ok(())
}

fn dfs(graph: &mut Vec<Vec<i16>>, visited: &mut Vec<bool>, s: usize, t: usize, min: i16) -> Option<i16> {
    visited[s] = true;

    if s == t {
        return Some(min);
    }

    for i in 0..graph.len() {
        if !visited[i] && graph[s][i] > 0 {
            if let Some(min_cap) = dfs(graph, visited, i, t, cmp::min(min, graph[s][i])) {
                graph[s][i] -= min_cap;
                graph[i][s] += min_cap;
                return Some(min_cap);
            }
        }
    }

    None
}

fn min_flow(graph: &mut Vec<Vec<i16>>, s: usize, t: usize) -> i16 {
    let mut flow = 0;

    while let Some(pushed_flow) = dfs(graph, &mut vec![false; graph.len()], s, t, std::i16::MAX) {
        flow += pushed_flow;
    }

    flow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pianolessons_sampleinputs() {
        for mut file in std::fs::read_dir("input/pianolessons")
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
