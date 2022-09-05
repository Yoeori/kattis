use std::error::Error;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap().split(' ').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>());

    let first_line = lines.next().unwrap();
    let (n, m) = (first_line[0], first_line[1]);

    let mut intersections: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n];

    for _ in 0..m {
        let line = lines.next().unwrap();
        intersections[line[0]].insert(line[1], line[2]);
        intersections[line[1]].insert(line[0], line[2]);
    }

    let fastest_roads = dijkstra(1, 0, &intersections, false); // Compute fasted road from each intersection; (Ams to Delft)

    // Remove road from graph
    for (from, to) in fastest_roads.iter().enumerate() {
        if let Some(to) = to {
            intersections[from].remove(to);
        }
    }

    // Calculate shortest path
    let path = dijkstra(0, 1, &intersections, true);

    match path[1] {
        None => {
            writeln!(&mut w, "impossible").unwrap();
        }
        Some(prev) => {
            let mut full_path = vec![1];
            let mut prev = Some(prev);

            while let Some(vertex) = prev {
                full_path.push(vertex);
                prev = path[vertex];
            }

            full_path.reverse();
            let full_path: Vec<String> = full_path.iter().map(|&vertex| vertex.to_string()).collect();

            writeln!(&mut w, "{} {}", full_path.len(), full_path.join(" ")).unwrap();
        }
    }


    Ok(())
}

fn dijkstra(from: usize, to: usize, intersections: &Vec<HashMap<usize, usize>>, stop_when_found: bool) -> Vec<Option<usize>> {

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
    let mut dist = vec![std::usize::MAX; intersections.len()];
    dist[from] = 0;
    let mut prev: Vec<Option<usize>> = vec![None; intersections.len()];

    let mut heap: BinaryHeap<Conn> = BinaryHeap::new();
    heap.push(Conn {cost: 0, vertex: from});

    while let Some(Conn {cost, vertex}) = heap.pop() {
        if stop_when_found && vertex == to {
            break;
        }

        if cost > dist[vertex] {
            continue;
        }

        for (&road_to, &road_cost) in intersections.get(vertex).unwrap().iter() {
            if dist[vertex] + road_cost < dist[road_to] {
                dist[road_to] = dist[vertex] + road_cost;
                prev[road_to] = Some(vertex);
                heap.push(Conn {
                    cost: dist[road_to],
                    vertex: road_to
                });
            }
        }
    }

    prev
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detour_sampleinputs() {
        for mut file in std::fs::read_dir("input/detour")
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
