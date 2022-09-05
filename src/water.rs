use std::error::Error;
use std::io::{self, BufRead, Write};
use std::collections::VecDeque;
use std::cmp;

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap()).map(|x| {
        let mut line_iter = x.split(' ');
        (line_iter.next().unwrap().parse::<usize>().unwrap(), line_iter.next().unwrap().parse::<usize>().unwrap(), line_iter.next().unwrap().parse::<usize>().unwrap())
    });

    let (n, p, k) = lines.next().unwrap();

    let mut res_graph: Vec<Vec<isize>> = vec![vec![0; n]; n];

    for _ in 0..p {
        let (a, b, c) = lines.next().unwrap();
        res_graph[a - 1][b - 1] = c as isize;
        res_graph[b - 1][a - 1] = c as isize;
    }

    let mut total = min_flow(&mut res_graph, 0, 1);
    writeln!(&mut w, "{}", total).unwrap();


    for _ in 0..k {
        let (a, b, c) = lines.next().unwrap();
        res_graph[a - 1][b - 1] += c as isize;
        res_graph[b - 1][a - 1] += c as isize;

        total += min_flow(&mut res_graph, 0, 1);
        writeln!(&mut w, "{}", total).unwrap();
    }


    Ok(())
}

fn bfs(graph: &mut Vec<Vec<isize>>, s: usize, mut t: usize) -> Option<isize> {
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(s);

    let mut prev: Vec<Option<(usize, isize)>> = vec![None; graph.len()];
    prev[s] = Some((s, std::isize::MAX));

    while let Some(v) = queue.pop_front() {
        for u in 0..graph.len() {
            if graph[v][u] == 0 || prev[u].is_some() {
                continue;
            }

            if prev[u].is_some() {
                continue;
            }

            prev[u] = Some((v, cmp::min(prev[v].unwrap().1, graph[v][u])));
            queue.push_back(u);

            if u == t {
                let mut from = t;
                let min_cap = prev[t].unwrap().1;
                while t != s {
                    t = prev[t].unwrap().0;
                    graph[t][from] -= min_cap;
                    graph[from][t] += min_cap;
                    from = t;
                }
                
                return Some(min_cap);
            }
        }
    }

    None
}

fn min_flow(graph: &mut Vec<Vec<isize>>, s: usize, t: usize) -> isize {
    let mut flow = 0;

    while let Some(pushed_flow) = bfs(graph, s, t) {
        flow += pushed_flow;
    }

    flow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_sampleinputs() {
        for mut file in std::fs::read_dir("input/water")
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
