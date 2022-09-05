use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|l| l.unwrap().split(' ').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>());

    let first_line = lines.next().unwrap();
    let (b, n, e) = (first_line[0], first_line[1], first_line[2]);

    let second_line = lines.next().unwrap();
    let (sb, sn, se) = (second_line[0], second_line[1], second_line[2]);

    let mut kayaks = lines.next().unwrap();
    kayaks.sort(); // We sort because we want the highest people to be devided first

    writeln!(&mut w, "{}", binary_search(2, (se * 2) * kayaks.iter().min().unwrap(), |test| {
        let (mut cb, mut cn, mut ce) = (b as isize, n as isize, e as isize);

        for &kayak in kayaks.iter() {
            // Minimal speed that needs to be reached by the rowers
            // Protip: do not use floating point for this calculation, because you _will_ fail
            let min_people_speed = if test % kayak == 0 {
                test / kayak
            } else {
                (test / kayak) + 1
            };

            if min_people_speed <= sb + sb {
                cb -= 2;
            } else if min_people_speed <= sb + sn {
                cb -= 1;
                cn -= 1;
            } else if min_people_speed <= sn + sn && min_people_speed <= sb + se {
                // Complicated logic:
                // Both n + n or b + e cofiguration is possible => prefer b + e if e's are available, otherwise n + n
                if ce > 0 {
                    ce -= 1;
                    cb -= 1;
                } else {
                    cn -= 2;
                }
            } else if min_people_speed <= sn + sn {
                cn -= 2;
            } else if min_people_speed <= sb + se {
                cb -= 1;
                ce -= 1;
            } else if min_people_speed <= sn + se {
                ce -= 1;
                cn -= 1;
            } else if min_people_speed <= se + se {
                ce -= 2;
            } else {
                // Impossible
                return Ordering::Less;
            }
        }

        if cb < 0 {
            cn += cb;
        }

        if cn < 0 {
            ce += cn;
        }

        if ce < 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    })).unwrap();

    Ok(())
}

fn binary_search(mut min: usize, mut max: usize, comparitor: impl Fn(usize) -> Ordering) -> usize {
    while (max - min) > 1 {
        let mid = (max + min) / 2;

        match comparitor(mid) {
            Ordering::Less => max = mid,
            Ordering::Greater => min = mid,
            Ordering::Equal => return mid
        }
    }

    if comparitor(max) == Ordering::Greater {
        max
    } else {
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kayaking_sampleinputs() {
        for mut file in std::fs::read_dir("input/kayaking")
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
