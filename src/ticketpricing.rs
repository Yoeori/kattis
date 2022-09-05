use std::collections::BinaryHeap;
use std::cmp::{self, Ordering};
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut writer: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap().split(' ').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>());

    let first_line = lines.next().unwrap();
    let (n, w) = (first_line[0], first_line[1]);

    let mut weeks: Vec<Vec<(usize, usize)>> = vec![vec![]; w + 1];

    for i in (0..=w).rev() {
        let line = lines.next().unwrap();
        for o in 0..line[0] {
            weeks[i].push((line[o + 1], line[line[0] + o + 1]));
        }
    }

    let (length, price_first_week) = longest_path(w, n, &weeks);
    writeln!(&mut writer, "{}\n{}", length, price_first_week).unwrap();

    Ok(())

    // Idea:
    // We need to find the longest path in a DAG, where nodes are available tickets and week number (N to 0 / W to 0)
    // Edges are total income (tickets sold * price) for each price, where tickets sold is min of available and expected sold.
}


fn longest_path(ori_weeks_left: usize, ori_tickets_left: usize, week_prices: &Vec<Vec<(usize, usize)>>) -> (isize, usize) {

    #[derive(Debug, PartialEq, Eq)]
    struct Vertex {
        weeks_left: usize,
        tickets_left: usize,
        total_income: isize
    }

    impl Ord for Vertex {
        fn cmp(&self, other: &Self) -> Ordering { 
            self.weeks_left.cmp(&other.weeks_left).then_with(|| self.total_income.cmp(&other.total_income).then_with(|| self.tickets_left.cmp(&other.tickets_left)))
        }
    }

    impl PartialOrd for Vertex {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
            Some(self.cmp(other))
         }
    }

    // We use a heap to keep track of discovered vertexes since not all will be discovered
    let mut heap: BinaryHeap<Vertex> = BinaryHeap::new();
    heap.push(Vertex {
        weeks_left: ori_weeks_left + 1,
        tickets_left: ori_tickets_left,
        total_income: 0
    });

    // Maybe change this to a HashMap if Vec is too big, especially since it's very sparse
    let mut dist = vec![vec![-std::isize::MAX; ori_tickets_left + 1]; ori_weeks_left + 2]; 
    dist[ori_weeks_left + 1][ori_tickets_left] = 0;

    let mut origin = vec![vec![std::usize::MAX; ori_tickets_left + 1]; ori_weeks_left + 1];

    while let Some(Vertex { weeks_left, tickets_left, total_income }) = heap.pop() {
        if weeks_left == 0 {
            return (total_income, origin[0][tickets_left]); // The first week -1 we encounter should contain the highest total_income;
        }

        if dist[weeks_left][tickets_left] > total_income {
            continue; // We have already found a longer path to this node
        }

        // If 0 tickets are left we follow an 'extra' edge to the final week.
        if tickets_left == 0 && weeks_left != ori_weeks_left + 1 {
            if dist[weeks_left][0] > dist[(weeks_left - 1)][0] {
                dist[weeks_left - 1][0] = dist[weeks_left][0];
                origin[weeks_left - 1][0] = origin[weeks_left][tickets_left];
                heap.push(Vertex {
                    weeks_left: weeks_left - 1,
                    tickets_left: 0,
                    total_income: dist[(weeks_left - 1)][0]
                });
            } else if dist[weeks_left][0] == dist[(weeks_left - 1)][0] {
                origin[weeks_left - 1][0] = cmp::min(origin[weeks_left][tickets_left], origin[weeks_left - 1][0]);
            }
            continue;
        }

        for &(price, tickets_sold_for_price) in week_prices[weeks_left - 1].iter() {
            let tickets_sold_for_price = cmp::min(tickets_sold_for_price, tickets_left);
            if dist[weeks_left - 1][(tickets_left - tickets_sold_for_price)] < dist[weeks_left][tickets_left] + (tickets_sold_for_price * price) as isize {
                dist[weeks_left - 1][(tickets_left - tickets_sold_for_price)] = dist[weeks_left][tickets_left] + (tickets_sold_for_price * price) as isize;

                if weeks_left == ori_weeks_left + 1 {
                    origin[weeks_left - 1][tickets_left - tickets_sold_for_price] = price;
                } else {
                    origin[weeks_left - 1][tickets_left - tickets_sold_for_price] = origin[weeks_left][tickets_left];
                }

                heap.push(Vertex {
                    weeks_left: weeks_left - 1,
                    tickets_left: tickets_left - tickets_sold_for_price,
                    total_income: dist[weeks_left - 1][tickets_left - tickets_sold_for_price]
                });
            } else if dist[weeks_left - 1][tickets_left - tickets_sold_for_price] == dist[weeks_left][tickets_left] + (tickets_sold_for_price * price) as isize {
                if weeks_left == ori_weeks_left + 1 {
                    origin[weeks_left - 1][tickets_left - tickets_sold_for_price] = cmp::min(origin[weeks_left - 1][tickets_left - tickets_sold_for_price], price);
                } else {
                    origin[weeks_left - 1][tickets_left - tickets_sold_for_price] = cmp::min(origin[weeks_left - 1][tickets_left - tickets_sold_for_price], origin[weeks_left][tickets_left]);
                }
            }
        }
    };

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticketpricing_sampleinputs() {
        for mut file in std::fs::read_dir("input/ticketpricing")
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
