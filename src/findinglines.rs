use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead, Write};
use std::io::Read;

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn slope(a: &Point, b: &Point) -> f64 {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        if dy == 0 {
            std::f64::MAX
        } else {
            (dx as f64) / (dy as f64)
        }
    }
}

struct Random {
    random: File
}

impl Default for Random {
    fn default() -> Self {
        return Random {
            random: File::open("/dev/urandom").unwrap()
        }
     }
}

impl Random {
    fn from_range(&mut self, s: usize, e: usize) -> usize {
        let mut buf = [0u8; std::mem::size_of::<usize>()];
        self.random.read_exact(&mut buf).unwrap();
        let n = usize::from_be_bytes(buf);
        (n % (e - s + 1)) + s
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());
    let n = lines.next().unwrap().parse::<usize>()?;
    let p = lines.next().unwrap().parse::<usize>()?;

    let expect = if (n * p) % 100 != 0 {
        ((n * p) / 100) + 1
    } else {
        (n * p) / 100
    };

    // Base cases
    if n <= 2 || expect <= 2 {
        writeln!(&mut w, "possible").unwrap();
        return Ok(());
    }

    let points: Vec<Point> = (&mut lines).take(n).map(|l| {
        let mut ps = (&l).split(' ');
        Point { x: ps.next().unwrap().parse().unwrap(), y: ps.next().unwrap().parse().unwrap() }
    }).collect();

    // Source of rand, assumes access to /dev/urandom
    let mut rnd = Random::default();

    for _ in 0..200 {
        // Generate pair
        let p1 = &points[rnd.from_range(0, n - 1)];
        let mut p2 = p1;
        while p2 == p1 {
            p2 = &points[rnd.from_range(0, n - 1)];
        }

        // Check slope of pair
        let slope = Point::slope(p1, p2);
        let mut count = 2;
        for p3 in points.iter() {
            if p3 == p1 || p3 == p2 {
                continue;
            }

            if Point::slope(p1, p3) == slope {
                count += 1;
            }
        }

        if count >= expect {
            writeln!(&mut w, "possible").unwrap();
            return Ok(());
        }
    }

    writeln!(&mut w, "impossible").unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn findinglines_sampleinputs() {
        for mut file in std::fs::read_dir("input/findinglines")
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
