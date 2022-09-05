use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

struct SegmentTree {
    tree: Vec<usize>
}

impl SegmentTree {
    #[allow(dead_code)]
    fn new(filled: usize, capacity: usize) -> Self {
        let mut tree = vec![0; 2 * (2.0_f64.powf((capacity as f64).log2().ceil())) as usize];
        let tree_size = tree.len();
        for i in 0..filled {
            tree[i + tree_size / 2] = 1
        }

        // Build tree
        for idx in (1..(tree.len()/2)).rev() {
            tree[idx] = tree[idx * 2 + 1] + tree[idx * 2];
        }

        SegmentTree {
            tree
        }
    }

    fn query(&self, l: usize, r: usize) -> usize {
        /// x is current vertex in tree, x & y is currently seen range of tree
        fn inner_query(tree: &[usize], l: usize, r: usize, v: usize, x: usize, y: usize) -> usize {
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

    fn set(&mut self, mut idx: usize, v: usize) {
        idx = self.index(idx);
        self.tree[idx] = v;
        idx >>= 1;
        while idx > 0 {
            self.tree[idx] = self.tree[idx * 2] + self.tree[idx * 2 + 1];
            idx >>= 1;
        }
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();

    for _ in 0..(lines.next().unwrap().unwrap().parse::<usize>().unwrap()) {
        let line = lines.next().unwrap().unwrap();
        let mut line_iter = line.split(' ');
        let (m, r) = (line_iter.next().unwrap().parse::<usize>().unwrap(), line_iter.next().unwrap().parse::<usize>().unwrap());

        let mut tree = SegmentTree::new(m, m + r);
        let mut index: Vec<usize> = (0..m).rev().collect();
        let mut index_pointer = m;
        let mut result: Vec<usize> = Vec::with_capacity(r);

        let requests = lines.next().unwrap().unwrap();
        for req in requests.split(' ').map(|x| x.parse::<usize>().unwrap()) {
            result.push(tree.query(index[req - 1] + 1, m + r));
            tree.set(index[req - 1], 0);
            tree.set(index_pointer, 1);
            index[req - 1] = index_pointer;
            index_pointer += 1;
        }

        let result: Vec<String> = result.into_iter().map(|x| x.to_string()).collect();
        writeln!(&mut w, "{}", result.join(" ")).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moviecollection_sampleinputs() {
        for mut file in std::fs::read_dir("input/moviecollection")
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
