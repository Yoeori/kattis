use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, io::{self, BufRead}};

#[derive(Debug, Clone)]
struct Intersection {
    i: u64,
    distance_to_firestation: u64,
    roads: Vec<(u64, u64)> // Intersection / length
}

impl Default for Intersection {
    fn default() -> Self {
        Self { i: Default::default(), distance_to_firestation: std::u64::MAX, roads: Default::default() }
    }
}

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let test_cases: u8 = lines.next().unwrap().unwrap().parse().unwrap();

    lines.next(); // Skip empty line

    for _ in 0..test_cases {

        // Steps
        let line = lines.next().unwrap().unwrap();
        let mut l1 = line.split(' ');

        let n_fire_stations: u64 = l1.next().unwrap().parse().unwrap();
        let n_intersections: u64 = l1.next().unwrap().parse().unwrap();

        let mut intersections: HashMap<u64, Intersection> = (1..=n_intersections).map(|i| (i, Intersection {
            i,
            ..Default::default()
        })).collect();

        let mut firestations: Vec<u64> = vec![];

        for _ in 0..n_fire_stations {
            firestations.push(lines.next().unwrap().unwrap().parse().unwrap());
        }

        while let Some(Ok(line)) = lines.next() {
            if line == "" {
                break;
            }

            let mut splitted = line.split(' ');
            let from: u64 = splitted.next().unwrap().parse().unwrap();
            let to: u64 = splitted.next().unwrap().parse().unwrap();
            let length: u64 = splitted.next().unwrap().parse().unwrap();

            intersections.get_mut(&from).unwrap().roads.push((to, length));
            intersections.get_mut(&to).unwrap().roads.push((from, length));
        }

        for firestation in firestations {
            if firestation <= n_intersections {
                relax_firestation(firestation, &mut intersections);
            }
        }

        let mut min_distance = std::u64::MAX;
        let mut min_firestation = 0;

        for i in 1..=n_intersections {
            let mut intersections_clone = intersections.clone();
            relax_firestation(i, &mut intersections_clone);
            let distance = max_distance(&intersections_clone);

            if distance < min_distance || (distance == min_distance && i < min_firestation) {
                min_distance = distance;
                min_firestation = i;
            }
        }

        println!("{}\n", min_firestation);
    }
}

fn max_distance(intersections: &HashMap<u64, Intersection>) -> u64 {
    intersections.values().map(|int| int.distance_to_firestation).max().unwrap()
}

fn relax_firestation(firestation: u64, intersections: &mut HashMap<u64, Intersection>) {
    #[derive(Debug, PartialEq, Eq)]
    struct FoundCost {
        cost: u64,
        intersection: u64
    }

    impl Ord for FoundCost {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost).then_with(|| other.intersection.cmp(&self.intersection))
        }
    }

    impl PartialOrd for FoundCost {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut heap: BinaryHeap<FoundCost> = BinaryHeap::new();
    heap.push(FoundCost { cost: 0, intersection: firestation });

    while let Some(FoundCost { cost, intersection }) = heap.pop() {
        
        let inter = intersections.get(&intersection).unwrap();

        // Cannot be relaxed since distance to firestation is already lower
        if cost > inter.distance_to_firestation { continue; }

        for (to_intersection, distance) in &inter.roads {
            let to_inter = intersections.get(to_intersection).unwrap();
            let next_cost = cost + distance;

            if next_cost < to_inter.distance_to_firestation {
                heap.push(FoundCost {
                    cost: next_cost,
                    intersection: *to_intersection
                });
            }
        }

        intersections.get_mut(&intersection).unwrap().distance_to_firestation = cost;
    }
}