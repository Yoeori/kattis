use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

enum Operation {
    Update {
        l: usize,
        r: usize,
        d: isize
    },
    Query {
        idx: usize
    }
}

impl Operation {
    fn from_string(inp: &str) -> Self {
        match &inp[0..1] {
            "!" => {
                let mut input = inp.split(' ');
                input.next();
                Operation::Update {
                    l: input.next().unwrap().parse().unwrap(),
                    r: input.next().unwrap().parse().unwrap(),
                    d: input.next().unwrap().parse().unwrap(),
                }
            },
            "?" => {
                Operation::Query {
                    idx: inp[2..].parse().unwrap()
                }
            }
            _ => unreachable!()
        }
    }
}

struct SegmentTree {
    tree: Vec<isize>
}

impl SegmentTree {
    #[allow(dead_code)]
    fn new(data: &[isize]) -> Self {
        let mut tree = vec![0; 2 * (2.0_f64.powf((data.len() as f64).log2().ceil())) as usize];
        let tree_size = tree.len();
        for i in 0..data.len() {
            tree[i + tree_size / 2] = data[i]
        }

        SegmentTree {
            tree
        }
    }

    fn empty(size: usize) -> Self {
        SegmentTree {
            tree: vec![0; 2 * (2.0_f64.powf((size as f64).log2().ceil())) as usize]
        }
    }

    fn query(&self, mut idx: usize) -> isize {
        let mut sum = 0;
        idx = self.index(idx);

        while idx > 0 {
            sum += self.tree[idx];
            idx >>= 1;
        }

        sum
    }

    fn index(&self, idx: usize) -> usize {
        self.tree.len() / 2 + idx
    }

    fn update_range(&mut self, l: usize, r: usize, d: isize) {
        /// x is current vertex in tree, x & y is currenlty seen range of tree
        fn inner_update_range(tree: &mut [isize], l: usize, r: usize, v: usize, x: usize, y: usize, d: isize) {
            if r < x || l > y {
                return;
            }

            if l <= x && y <= r {
                tree[v] += d;
                return;
            }

            let mid = (x + y) / 2;
            inner_update_range(tree, l, r, 2*v, x, mid, d);
            inner_update_range(tree, l, r, 2*v + 1, mid + 1, y, d);
        }
        let n = self.tree.len() / 2;
        inner_update_range(&mut self.tree, l, r, 1, 0, n-1, d);
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();

    let first_line: Vec<usize> = lines.next().unwrap().unwrap().split(' ').map(|c| c.parse::<usize>().unwrap()).collect();
    let (n, k, q) = (first_line[0], first_line[1], first_line[2]);

    let mut segment_tree = SegmentTree::empty(n);
    for operation in lines.map(|o| Operation::from_string(&o.unwrap())).take(k + q) {
        match operation {
            Operation::Update { l, r, d } => segment_tree.update_range(l, r-1, d),
            Operation::Query { idx } => writeln!(&mut w, "{}", segment_tree.query(idx - 1)).unwrap()
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uib_pointsofsnow_sampleinputs() {
        for mut file in std::fs::read_dir("input/uib.pointsofsnow")
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

    #[test]
    fn segmenttree_test() {
        for size in 10..=16 {
            let mut tree = SegmentTree::new(&vec![5; size]); // Initial data is 5.

            // Check initial values
            for idx in 0..size {
                assert_eq!(tree.query(idx), 5);
            }

            tree.update_range(0, size-1, 1); // Add two to entire tree

            // Check updated value
            for idx in 0..size {
                assert_eq!(tree.query(idx), 6);
            }

            // First and last value update
            tree.update_range(0, 0, -1);
            assert_eq!(tree.query(0), 5);

            tree.update_range(size - 1, size-1, -1);
            assert_eq!(tree.query(size - 1), 5);

            // Update rest
            tree.update_range(1, size - 2, -1);
            for idx in 0..size {
                assert_eq!(tree.query(idx), 5);
            }
        }
    }
}
