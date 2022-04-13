use std::ops::{Neg, Div, Mul, Sub, Add};
use std::error::Error;
use std::io::{self, BufRead, Write};
use std::cmp;

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: isize,
    y: isize
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<isize> for &Point {
    type Output = Point;

    fn mul(self, rhs: isize) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Div<isize> for &Point {
    type Output = Point;

    fn div(self, rhs: isize) -> Point {
        Point {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Point {
    fn cross(&self, other: &Self) -> isize {
        self.x * other.y - self.y * other.x
    }

    fn orient(a: &Point, b: &Point, c: &Point) -> isize {
        (b - a).cross(&(c - a))
    }

    fn dist(a: &Point, b: &Point) -> isize {
        (a.x - b.x).pow(2) + (a.y - b.y).pow(2)
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines().map(|x| x.unwrap());

    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut points: Vec<Point> = lines.take(n).map(|x| {
        let mut line_iter = x.split(' ');
        Point {
            x: line_iter.next().unwrap().parse::<isize>().unwrap(),
            y: line_iter.next().unwrap().parse::<isize>().unwrap()
        }
    }).collect();

    let hull = graham(&mut points);
    let mut max_d = 0;
    for i in 0..hull.len() {
        for j in (i + 1)..hull.len() {
            max_d = cmp::max(Point::dist(hull[i], hull[j]), max_d);
        }
    }

    writeln!(&mut w, "{:.10}", (max_d as f64).sqrt()).unwrap();

    Ok(())
}

fn graham(points: &mut Vec<Point>) -> Vec<&Point> {
    points.sort();
    points.dedup();

    if points.len() == 1 {
        return vec![&points[0], &points[0]];
    }

    let mut s: Vec<&Point> = vec![];

    for p in points.iter() {
        while s.len() >= 2 && Point::orient(s[s.len() - 2], s[s.len() - 1], p) > 0 {
            s.pop();
        }
        s.push(p);
    }

    let mut hull = s;
    let mut s: Vec<&Point> = vec![];

    for p in points.iter().rev() {
        while s.len() >= 2 && Point::orient(s[s.len() - 2], s[s.len() - 1], p) > 0 {
            s.pop();
        }
        s.push(p);
    }

    hull.extend(&s[1..s.len()-1]);

    hull
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roberthood_sampleinputs() {
        for mut file in std::fs::read_dir("input/roberthood")
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
                (std::str::from_utf8(&output_writer).unwrap().trim().parse::<f64>().unwrap() - std::fs::read_to_string(&file).unwrap().trim().parse::<f64>().unwrap()).abs() < 10.0f64.powi(-6)
            );
        }
    }
}
