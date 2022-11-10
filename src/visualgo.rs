use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());

    let first_line = lines.next().unwrap();
    let mut first_line = first_line.split(' ').map(|x| x.parse::<usize>().unwrap());
    let (v, e) = (first_line.next().unwrap(), first_line.next().unwrap());

    let mut map: Vec<Vec<(usize, usize)>> = vec![vec![]; v];
    
    for _ in 0..e {
        let line = lines.next().unwrap();
        let mut line = line.split(' ').map(|x| x.parse::<usize>().unwrap());
        map[line.next().unwrap()].push((line.next().unwrap(), line.next().unwrap()));
    }

    let last_line = lines.next().unwrap();
    let mut last_line = last_line.split(' ').map(|x| x.parse::<usize>().unwrap());
    let (s, t) = (last_line.next().unwrap(), last_line.next().unwrap());

    writeln!(&mut w, "{}", dijkstra(s, t, &map)).unwrap();

    Ok(())
}

fn dijkstra(from: usize, to: usize, map: &Vec<Vec<(usize, usize)>>) -> usize {
    #[derive(Eq, PartialEq)]
    struct Conn {
        cost: usize,
        vertex: usize
    }

    impl Ord for Conn {
        fn cmp(&self, other: &Self) -> Ordering { 
            other.cost.cmp(&self.cost).then_with(|| self.vertex.cmp(&other.vertex))
        }
    }

    impl PartialOrd for Conn {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
            Some(self.cmp(other))
         }
    }
    
    // Dijkstra
    let mut dist = vec![std::usize::MAX; map.len()];
    dist[from] = 0;

    // Keep track of how many paths there are to X;
    let mut paths = vec![0; map.len()];
    paths[from] = 1;

    let mut relaxed = vec![false; map.len()];

    let mut heap: BinaryHeap<Conn> = BinaryHeap::new();
    heap.push(Conn {cost: 0, vertex: from});

    while let Some(Conn {cost, vertex}) = heap.pop() {
        if cost > dist[to] {
            break;
        }

        if cost > dist[vertex] {
            continue;
        }

        if relaxed[vertex] {
            continue;
        }
        relaxed[vertex] = true;

        for &(path_to, path_cost) in map.get(vertex).unwrap() {
            if dist[vertex] + path_cost < dist[path_to] {
                dist[path_to] = dist[vertex] + path_cost;
                paths[path_to] = paths[vertex];
                heap.push(Conn {
                    cost: dist[path_to],
                    vertex: path_to
                });
            } else if dist[vertex] + path_cost == dist[path_to] {
                paths[path_to] = paths[path_to] + paths[vertex];
            } 
        }
    }

    paths[to]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visualgo_sampleinputs() {
        for mut file in std::fs::read_dir("input/visualgo")
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
