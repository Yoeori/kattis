use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Clone)]
struct Element {
    set: u64,
    min: usize,
    len: usize,
    pref: [(u64, usize); 50],   // (mask, length from start)
    posf: [(u64, usize); 50]
}

impl Default for Element {
    fn default() -> Self {
        Element {
            set: 0,
            min: std::usize::MAX,
            len: 0,
            pref: [(0, 0); 50],
            posf: [(0, 0); 50]
        }
    }
}

struct SegmentTree {
    tree: Vec<Element>,
    k: usize
}

impl SegmentTree {
    fn new(data: &[u8], k: usize) -> Self {
        let mut tree = vec![Element::default(); 2 * (2.0_f64.powf((data.len() as f64).log2().ceil())) as usize];
        let tree_size = tree.len();

        for i in 0..data.len() {
            tree[i + tree_size / 2] = Element {
                set: 1 << (data[i] - 1),
                min: if k == 1 { 1 } else { std::usize::MAX },
                len: 1,
                ..Default::default()
            };
            tree[i + tree_size / 2].pref[0] = (tree[i + tree_size / 2].set, 1);
            tree[i + tree_size / 2].posf[0] = (tree[i + tree_size / 2].set, 1);
        }

        // Build tree
        let mut segmenttree = SegmentTree {
            tree,
            k
        };

        for idx in (1..(tree_size / 2)).rev() {
            segmenttree.tree[idx] = segmenttree.merge(idx);
        }

        segmenttree
    }

    fn query(&self) -> usize {
        self.tree[1].min
    }

    fn index(&self, idx: usize) -> usize {
        self.tree.len() / 2 + idx
    }

    fn merge(&self, idx: usize) -> Element {

        let left = &self.tree[idx * 2];
        let right = &self.tree[idx * 2 + 1];

        let exp_v = (1 << self.k) - 1;

        let mut element = Element {
            set: left.set | right.set,
            min: std::cmp::min(left.min, right.min),
            len: left.len + right.len,
            ..Default::default()
        };

        if element.set == 0 {
            return element;
        }

        if right.set == 0 {
            element.posf = left.posf.clone();
            element.pref = left.pref.clone();
            return element;
        }

        // Calculate pref (untill full)
        // 1. Copy prefix from left.pre
        // 2. Copy prefix from right.pre masked with best last left.pre if we find more k's
        let mut prefix_index = 0;
        for &(mask, length) in left.pref.iter().take_while(|x| x.0 != 0) {
            element.pref[prefix_index] = (mask, length);
            prefix_index += 1;
        }

        for &(mask, length) in right.pref.iter().take_while(|x| x.0 != 0) {
            if prefix_index < 50 && mask | element.pref[prefix_index - 1].0 > element.pref[prefix_index - 1].0 {
                element.pref[prefix_index] = (mask | element.pref[prefix_index - 1].0, length + left.len);
                prefix_index += 1;
            }
        }

        // Calculate posf (untill full)
        let mut postfix_index = 0;
        for &(mask, length) in right.posf.iter().take_while(|x| x.0 != 0) {
            element.posf[postfix_index] = (mask, length);
            postfix_index += 1;
        }

        for &(mask, length) in left.posf.iter().take_while(|x| x.0 != 0) {
            if postfix_index < 50 && mask | element.posf[postfix_index - 1].0 > element.posf[postfix_index - 1].0 {
                element.posf[postfix_index] = (mask | element.posf[postfix_index - 1].0, length + right.len);
                postfix_index += 1;
            }
        }

        if element.set == exp_v {
            let mut min = std::usize::MAX;

            // Go through each combination of prefixes and postfixes to find the smallest subset with all k's
            for pre in 0..50 {
                for post in 0..50 {
                    if left.posf[post].0 | right.pref[pre].0 == exp_v {
                        min = std::cmp::min(left.posf[post].1 + right.pref[pre].1, min);
                    }
                }
            }

            element.min = std::cmp::min(min, element.min);
        }

        element
    }

    fn set(&mut self, mut idx: usize, v: u8) {
        idx = self.index(idx);
        let old = self.tree[idx].set;
        self.tree[idx].set = 1 << (v - 1);
        self.tree[idx].min = if self.k == 1 { 1 } else { std::usize::MAX };

        if self.tree[idx].set == old {
            return;
        }

        self.tree[idx].pref[0] = (self.tree[idx].set, 1);
        self.tree[idx].posf[0] = (self.tree[idx].set, 1);

        idx >>= 1;
        while idx > 0 {
            self.tree[idx] = self.merge(idx);
            idx >>= 1;
        }
    }
}


fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    // Idea: create a segment tree of sets (union of leafs) and keep track of smallest subarray
    //      Speedup: use u64, since the first 50 bits can indicate in set or not.

    // To find the smallest subarray on a node in the tree there are three options (if subarray is indeed in a node):
    //  - The smallest sub array is in the left tree
    //  - The smallest sub array is in the right tree
    //  - The smallest sub array is in the middle of the two leafs
    //    - Keep track of prefix / postfix indicating min length to get x k's to be able to calculate this

    let mut lines = input.lines().map(|x| x.unwrap()).map(|l| l.split(' ').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>());
    let first_line = lines.next().unwrap();
    let (_, k, m) = (first_line[0], first_line[1], first_line[2]);

    let xs: Vec<u8> = lines.next().unwrap().into_iter().map(|x| x as u8).collect();
    let mut tree = SegmentTree::new(&xs, k);

    for query in lines.take(m) {
        if query[0] == 1 {
            tree.set(query[1] - 1, query[2] as u8);
        } else if query[0] == 2 {
            let ans = tree.query();
            if ans != std::usize::MAX {
                writeln!(&mut w, "{}", ans).unwrap();
            } else {
                writeln!(&mut w, "-1").unwrap();
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nekameleoni_sampleinputs() {
        for mut file in std::fs::read_dir("input/nekameleoni")
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
