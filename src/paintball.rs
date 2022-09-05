use std::error::Error;
use std::io::{self, BufRead, Write};
use std::cmp;

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap()).map(|x| {
        let mut line_iter = x.split(' ');
        (line_iter.next().unwrap().parse::<usize>().unwrap(), line_iter.next().unwrap().parse::<usize>().unwrap())
    });

    let (n, m) = lines.next().unwrap();

    // Adjecency matrix, 0..n incoming, n..2*n receiving, 2*n source, 2*n+1 sink
    let mut res_graph: Vec<Vec<i16>> = vec![vec![0; n * 2 + 2]; n * 2 + 2];

    for i in 0..n {
        res_graph[2*n][i] = 1;
        res_graph[n + i][2*n+1] = 1;
    }

    for _ in 0..m {
        let (a, b) = lines.next().unwrap();
        res_graph[a - 1][n + b - 1] = 1;
        res_graph[b - 1][n + a - 1] = 1;
    }

    let res = min_flow(&mut res_graph, 2*n, 2*n+1);
    if res != n as i16 {
        writeln!(&mut w, "Impossible").unwrap();
    } else {
        'outloop: for i in 0..n {
            for o in 0..n {
                if res_graph[n + o][i] == 1 {
                    writeln!(&mut w, "{}", o + 1).unwrap();
                    continue 'outloop;
                }
            }
        }
    }

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
    fn paintball_sampleinputs() {
        for mut file in std::fs::read_dir("input/paintball")
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
