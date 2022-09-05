use std::error::Error;
use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();

    // Loop through test cases
    for _ in 0..(lines.next().unwrap().unwrap().parse().unwrap()) {

        let m: usize = lines.next().unwrap().unwrap().parse().unwrap();

        let mut nodes: Vec<(usize, f32, f32)> = Vec::with_capacity(m);
        let mut edges: Vec<(usize, usize, f32)> = Vec::with_capacity(m * (m - 1) / 2);

        // 1. Create connected graph of islands
        for i in 0..m {
            let line = lines.next().unwrap().unwrap();
            let mut line = line.split(' ');
            let (x, y) = (line.next().unwrap().parse::<f32>().unwrap(), line.next().unwrap().parse::<f32>().unwrap());
            for &(o, x2, y2) in nodes.iter() {
                edges.push((o, i, ((x - x2).powi(2) + (y - y2).powi(2)).sqrt()));
            }
            nodes.push((i, x, y));
        }

        // 2. Find MST over graph (using kruskal's algorithm)
        edges.sort_by(|e1, e2| e1.2.partial_cmp(&e2.2).unwrap_or(Ordering::Equal));

        let mut parent: Vec<usize> = (0..m).collect();
        let mut cost = 0.0;

        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        for edge in edges.iter() {
            if find(&mut parent, edge.0) != find(&mut parent, edge.1) {
                cost += edge.2;
                let parent_1 = find(&mut parent, edge.0);
                let parent_2 = find(&mut parent, edge.1);
                parent[parent_1] = parent_2;
            }
        }

        // 3. Output summed length of MST
        writeln!(&mut w, "{}", cost).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn islandhopping_sampleinputs() {
        for mut file in std::fs::read_dir("input/islandhopping")
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
            let mut output = std::str::from_utf8(&output_writer).unwrap().trim().lines().map(|x| x.parse::<f32>().unwrap());
            for answer in std::fs::read_to_string(&file).unwrap().trim().lines().map(|x| x.parse::<f32>().unwrap()) {
                assert!((answer - output.next().unwrap()).abs() < 10.0f32.powi(-4), "file: {:?}", file);
            }
        }
    }
}
