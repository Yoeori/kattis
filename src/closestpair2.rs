use std::error::Error;
use std::io::{self, BufRead, Write};
use std::cmp;

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

type IntSize = i64;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: IntSize,
    y: IntSize,
    s: String
}

impl Point {
    fn dist(a: &Point, b: &Point) -> IntSize {
        (a.x - b.x).pow(2) + (a.y - b.y).pow(2)
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());

    while let Ok(n) = lines.next().unwrap().parse::<usize>() {
        if n == 0 {
            break;
        }

        let mut points: Vec<Point> = (&mut lines).take(n).map(|l| {
            let mut ps = (&l).split(' ');
            let x = parse_float(&ps.next().unwrap());
            let y = parse_float(&ps.next().unwrap());

            Point { x, y, s: l }
        }).collect();

        points.sort_by_key(|e| e.x);
        
        let (_, a, b) = min_distance(&points);
        writeln!(&mut w, "{} {}", a.s, b.s).unwrap();
    }

    Ok(())
}

fn parse_float(inp: &str) -> IntSize {
    let mut ans: IntSize = 0;
    let mut minus = false;
    let mut count = 0;
    let mut after_dot = false;
    for c in inp.chars() {
        match c {
            '-' => minus = true,
            '.' => after_dot = true,
            c => {
                ans *= 10;
                ans += c.to_digit(10).unwrap() as IntSize;
                if after_dot {
                    count += 1;
                }
            }
        }
    }

    if count == 0 {
        ans *= 100;
    } else if count == 1 {
        ans *= 10;
    }

    if minus {
        -ans
    } else {
        ans
    }
}

fn min_distance_full<'a>(points: &'a [Point]) -> (IntSize, &'a Point, &'a Point) {
    let mut ans = (Point::dist(&points[0], &points[1]), &points[0], &points[1]);
    for i in 0..points.len() {
        for o in (i+1)..points.len() {
            let c = (Point::dist(&points[i], &points[o]), &points[i], &points[o]);
            ans = cmp::min(ans, c);
        }
    }

    ans
}

fn min_distance<'a>(points: &'a [Point]) -> (IntSize, &'a Point, &'a Point) {

    if points.len() <= 3 {
        return min_distance_full(points);
    }

    let mid = points.len() / 2;
    let mut ans = cmp::min(min_distance(&points[0..mid]), min_distance(&points[mid..]));

    let delta = (ans.0 as f64).sqrt();

    let p_mid = points[mid].x;
    let mut strip = vec![];

    // Points to the right of the mid line
    for p in points[mid..].iter() {
        if ((p.x - p_mid).abs() as f64) < delta {
            strip.push(p);
        } else {
            break;
        }
    }

    // Points to the left of the mid line
    for p in points[0..mid].iter().rev() {
        if ((p_mid - p.x).abs() as f64) < delta {
            strip.push(p);
        } else {
            break;
        }
    }

    strip.sort_by_key(|c| c.y);

    for i in 0..strip.len() {
        for o in i+1..cmp::min(strip.len(), i+8) {
            ans = cmp::min(ans, (Point::dist(&strip[i], &strip[o]), &strip[i], &strip[o]))
        }
    }

    ans
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closestpair2_sampleinputs() {
        for mut file in std::fs::read_dir("input/closestpair2")
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