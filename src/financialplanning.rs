use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap()).map(|x| {
        let mut spl = x.split(' ').map(|y| y.parse::<usize>().unwrap());
        (spl.next().unwrap(), spl.next().unwrap())
    });

    let (n, m) = lines.next().unwrap();

    // Find day of initial profit, initial profit and profit per day
    let mut investment_options: Vec<(usize, usize, usize)> = lines.take(n).map(|(pi, ci)| {
        let days = (ci / pi) + 1;
        (days, days * pi - ci, pi)
    }).collect();

    investment_options.sort_by_key(|(d, _, _)| -(*d as isize));

    let mut total_profits = 0;
    let mut profit_per_day = 0;
    let mut day = 1;

    while !investment_options.is_empty() {
        let simulate_to = investment_options[investment_options.len() - 1].0;
        total_profits += (simulate_to - day) * profit_per_day;

        // Check if achieved
        if total_profits >= m {
            total_profits -= (simulate_to - day) * profit_per_day;
            break;
        }

        day = simulate_to;

        while !investment_options.is_empty() && investment_options[investment_options.len() - 1].0 == day {
            let (_, init_prof, per_day) = investment_options.pop().unwrap();
            total_profits += init_prof;
            profit_per_day += per_day;
        }

        // Check if achieved
        if total_profits >= m {
            break;
        }
    }

    // Calculate remaining days
    if total_profits < m {
        let mut days_remaining = (m - total_profits) / profit_per_day;
        if (m - total_profits) % profit_per_day != 0 {
            days_remaining += 1;
        }
        day += days_remaining;
    }

    writeln!(&mut w, "{}", day).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn financialplanning_sampleinputs() {
        for mut file in std::fs::read_dir("input/financialplanning")
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
