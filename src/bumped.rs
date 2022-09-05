use std::collections::BinaryHeap;
use core::cmp::Ordering;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap().split(' ').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>());

    let first_line = lines.next().unwrap();
    let (n, m, f, s, t) = (first_line[0], first_line[1], first_line[2], first_line[3], first_line[4]);

    let mut cities: Vec<Vec<(usize, usize, usize)>> = vec![vec![]; n];

    for _ in 0..m {
        let line = lines.next().unwrap();
        cities[line[0]].push((line[0], line[1], line[2]));
        cities[line[1]].push((line[1], line[0], line[2]));
    }

    let mut flights: Vec<(usize, usize)> = vec![];

    for _ in 0..f {
        let line = lines.next().unwrap();
        flights.push((line[0], line[1]));
    }

    writeln!(&mut w, "{}", dijkstra(s, t, &cities, &flights)).unwrap();
    Ok(())
}

fn dijkstra(from: usize, to: usize, cities: &Vec<Vec<(usize, usize, usize)>>, flights: &Vec<(usize, usize)>) -> usize {

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
    let mut dist = vec![std::usize::MAX; cities.len()];
    dist[from] = 0;

    let mut heap: BinaryHeap<Conn> = BinaryHeap::new();
    heap.push(Conn {cost: 0, vertex: from});

    fn inner_dijkstra(heap: &mut BinaryHeap<Conn>, dist: &mut Vec<usize>, cities: &Vec<Vec<(usize, usize, usize)>>, to: usize) {
        while let Some(Conn {cost, vertex}) = heap.pop() {
            if vertex == to {
                // heap.drain();
                break;
            }

            if cost > dist[vertex] {
                continue;
            }

            for &(_, car_to, car_cost) in cities.get(vertex).unwrap() {
                if dist[vertex] + car_cost < dist[car_to] {
                    dist[car_to] = dist[vertex] + car_cost;
                    heap.push(Conn {
                        cost: dist[car_to],
                        vertex: car_to
                    });
                }
            }
        }
    }

    inner_dijkstra(&mut heap, &mut dist, &cities, to);

    for &(flight_from, flight_to) in flights {
        if dist[flight_to] > dist[flight_from] {
            dist[flight_to] = dist[flight_from];
            heap.push(Conn {
                cost: dist[flight_to], vertex: flight_to
            });
        }
    }

    inner_dijkstra(&mut heap, &mut dist, &cities, to);

    dist[to]
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bumped_sampleinputs() {
        for mut file in std::fs::read_dir("input/bumped")
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
