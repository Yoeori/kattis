use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let graph: HashMap<u16, Vec<u16>> = (1000..=9999).filter(|&n| is_prime(n)).map(|n| (n, neighbours(n))).collect();

    for _ in 0..(lines.next().unwrap().unwrap().parse::<u16>().unwrap()) {
        let line = lines.next().unwrap().unwrap();
        let mut ns = line.split(' ');
        let res = dijkstra(ns.next().unwrap().parse().unwrap(), ns.next().unwrap().parse().unwrap(), &graph);

        match res {
            Some(res) => println!("{}", res),
            None => println!("Impossible")
        }
    }

}

fn dijkstra(from: u16, to: u16, graph: &HashMap<u16, Vec<u16>>) -> Option<u16> {
    // Dijkstra
    #[derive(Eq, PartialEq)]
    struct Edge {
        cost: u16,
        vertex: u16
    }

    impl Ord for Edge {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost).then_with(|| other.vertex.cmp(&self.vertex))
        }
    }

    impl PartialOrd for Edge {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut dist: HashMap<u16, u16> = graph.keys().map(|&x| (x, std::u16::MAX)).collect();
    dist.insert(from, 0);

    let mut prev: HashMap<u16, Option<u16>> = graph.keys().map(|&x| (x, None)).collect();

    let mut heap: BinaryHeap<Edge> = BinaryHeap::new();
    heap.push(Edge {
        cost: 0,
        vertex: from
    });

    while let Some(Edge { cost, vertex }) = heap.pop() {
        if vertex == to {
            return Some(cost);
        }

        let v_dist = *dist.get(&vertex).unwrap();

        if cost > v_dist { continue; }

        for neighbour in graph.get(&vertex).unwrap() {
            if v_dist + 1 < *dist.get(neighbour).unwrap() {
                dist.insert(*neighbour, v_dist + 1);
                prev.insert(*neighbour, Some(vertex));
                heap.push(Edge {
                    cost: v_dist + 1,
                    vertex: *neighbour
                });
            }
        }
    }

    None
}

fn neighbours(from: u16) -> Vec<u16> {
    let mut neighbours: Vec<u16> = vec![];

    for i in 0..4 {
        for x in 0..=9 {
            let n = ((from / 10u16.pow(i+1)) * 10u16.pow(i+1)) + (x * 10u16.pow(i)) + (from % 10u16.pow(i));
            if n != from && n >= 1000 && is_prime(n) {
                neighbours.push(n);
            }
        }
    }

    neighbours
}

fn is_prime(n: u16) -> bool {
    // Can be done a lot more efficient with some kind of seiving but why bother
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}