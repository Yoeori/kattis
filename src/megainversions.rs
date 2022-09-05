use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
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

        // Build tree
        for idx in (1..(tree.len()/2)).rev() {
            tree[idx] = tree[idx * 2 + 1] + tree[idx * 2];
        }

        SegmentTree {
            tree
        }
    }

    #[allow(dead_code)]
    fn empty(size: usize) -> Self {
        SegmentTree {
            tree: vec![0; 2 * (2.0_f64.powf((size as f64).log2().ceil())) as usize]
        }
    }

    fn query(&self, l: usize, r: usize) -> isize {
        /// x is current vertex in tree, x & y is currently seen range of tree
        fn inner_query(tree: &[isize], l: usize, r: usize, v: usize, x: usize, y: usize) -> isize {
            if r < x || l > y {
                0
            } else if l <= x && y <= r {
                tree[v]
            } else {
                let mid = (x + y) / 2;
                inner_query(tree, l, r, 2*v, x, mid) + inner_query(tree, l, r, 2*v + 1, mid + 1, y)
            }
        }

        let n = self.tree.len() / 2;
        inner_query(&self.tree, l, r, 1, 0, n-1)
    }

    fn index(&self, idx: usize) -> usize {
        self.tree.len() / 2 + idx
    }

    fn set(&mut self, mut idx: usize, v: isize) {
        idx = self.index(idx);
        self.tree[idx] = v;
        idx >>= 1;
        while idx > 0 {
            self.tree[idx] = self.tree[idx * 2] + self.tree[idx * 2 + 1];
            idx >>= 1;
        }
    }

    fn update(&mut self, idx: usize, d: isize) {
        self.set(idx, self.tree[self.index(idx)] + d)
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {

    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let sequence: Vec<usize> = lines.next().unwrap().unwrap().split(' ').map(|x| x.parse::<usize>().unwrap()).collect();

    // We build two segment trees: one containing all the remaining numbers (to the right of the cursor)
    // and one containing all the seen numbers (to the left of the cursor)

    let mut initial_right_tree = vec![0; n + 1];
    for &c in &sequence {
        initial_right_tree[c] += 1;
    }

    let mut left_tree = SegmentTree::empty(n + 1);
    let mut right_tree = SegmentTree::new(&initial_right_tree);
    let mut total: isize = 0;
    
    for c in sequence {
        total += left_tree.query(c+1, n) * right_tree.query(0, c-1);
        left_tree.update(c, 1);
        right_tree.update(c, -1);
    }

    writeln!(&mut w, "{}", total).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn megainversions_sampleinputs() {
        for mut file in std::fs::read_dir("input/megainversions")
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
