use std::error::Error;
use std::io::{self, BufRead, Write};
use std::cmp;

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap().split(' ').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>());
    lines.next().unwrap();
    let c = lines.next().unwrap(); // cost of dishes

    lines.next().unwrap();
    let s = lines.next().unwrap(); // orders
    let max_order = s.iter().max().unwrap();
    let (items, ambi) = longest_path(&c, *max_order);

    for order_price in s {
        if items[order_price].is_none() {
            writeln!(&mut w, "Impossible").unwrap();
        } else if ambi[order_price] {
            writeln!(&mut w, "Ambiguous").unwrap();
        } else {
            let mut cur = order_price;
            let mut used_items_indexes: Vec<usize> = vec![];
            while let Some(index) = items[cur] {
                used_items_indexes.push(index);
                cur = cur - c[index];
            }
            used_items_indexes.reverse();
            let used_items_indexes: Vec<String> = used_items_indexes.into_iter().map(|x| (x + 1).to_string()).collect();
            writeln!(&mut w, "{}", used_items_indexes.join(" ")).unwrap();
        }

    }

    // DAG-approach (since we have repetition knapsack)
    // Find longest path in DAG with length order_price
    // Keep track of ambiguity by noting if vertex is reached more than once with current value

    Ok(())
}

fn longest_path(items: &Vec<usize>, cap: usize) -> (Vec<Option<usize>>, Vec<bool>) {
    let mut dist: Vec<usize> = vec![0; cap + 1];
    let mut prev: Vec<Option<usize>> = vec![None; cap + 1];
    let mut ambi: Vec<bool> = vec![false; cap + 1];

    for vertex in 0..cap {
        if dist[vertex] > 0 || vertex == 0 {
            let min_i = prev[vertex].unwrap_or(0);
            for (i, &item_cost) in items.iter().enumerate().filter(|(i, _)| *i >= min_i) {
                if vertex + item_cost <= cap && dist[vertex + item_cost] < dist[vertex] + item_cost {
                    dist[vertex + item_cost] = dist[vertex] + item_cost;
                    prev[vertex + item_cost] = Some(i);
                    ambi[vertex + item_cost] = ambi[vertex + item_cost] || ambi[vertex];
                } else if vertex + item_cost <= cap && dist[vertex + item_cost] == dist[vertex] + item_cost {
                    ambi[vertex + item_cost] = true;
                    prev[vertex + item_cost] = Some(cmp::min(i, prev[vertex + item_cost].unwrap()));
                }   
            }
        }
    }

    (prev, ambi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_sampleinputs() {
        for mut file in std::fs::read_dir("input/orders")
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
