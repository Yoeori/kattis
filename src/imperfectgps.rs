use std::cmp;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();

    let first_line: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let (n, t_step) = (first_line[0], first_line[1]);

    let path: Vec<(f64, f64, usize)> = lines
        .take(n)
        .map(|l| l.unwrap())
        .map(|l| {
            let mut pathiter = l.split(' ').map(|x| x.parse::<isize>().unwrap());
            (
                pathiter.next().unwrap() as f64,
                pathiter.next().unwrap() as f64,
                pathiter.next().unwrap() as usize,
            )
        })
        .collect();

    let mut gps_path: Vec<(f64, f64, usize)> = Vec::with_capacity(path.len());

    let mut t_cur = 0;
    let mut p_cur = 0;

    loop {
        while t_cur > path[p_cur + 1].2 {
            // Get correct line segment
            p_cur += 1;
        }

        let dx = path[p_cur + 1].0 - path[p_cur].0;
        let dy = path[p_cur + 1].1 - path[p_cur].1;
        let perc = ((t_cur - path[p_cur].2) as f64) / ((path[p_cur + 1].2 - path[p_cur].2) as f64);

        gps_path.push((path[p_cur].0 + dx * perc, path[p_cur].1 + dy * perc, t_cur));

        if t_cur == path[n - 1].2 {
            break;
        }

        t_cur = cmp::min(path[n - 1].2, t_cur + t_step);
    }

    let orig_len = path_len(&path);
    let gps_len = path_len(&gps_path);

    writeln!(&mut w, "{}", ((orig_len - gps_len) / orig_len) * 100.0).unwrap();
    Ok(())
}

fn path_len(path: &Vec<(f64, f64, usize)>) -> f64 {
    let mut total = 0.0;
    for window in path.windows(2) {
        total += ((window[0].0 - window[1].0).powi(2) + (window[0].1 - window[1].1).powi(2)).sqrt();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imperfectgps_sampleinputs() {
        for mut file in std::fs::read_dir("input/imperfectgps")
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
            assert!(
                (std::str::from_utf8(&output_writer).unwrap().trim().parse::<f64>().unwrap() - std::fs::read_to_string(&file).unwrap().trim().parse::<f64>().unwrap()).abs() < 10.0f64.powi(-5)
            );
        }
    }
}
