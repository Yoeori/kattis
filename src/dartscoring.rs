use std::ops::{Neg, Div, Mul, Sub, Add};
use std::error::Error;
use std::cmp::Ordering;
use std::io::{self, BufRead, Write};

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64
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

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Div<f64> for &Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Point {
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
    pub fn square(&self) -> f64 {
        return self.x * self.x + self.y * self.y;
    }

    pub fn abs(&self) -> f64 {
        self.square().sqrt()
    }

    pub fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn orient(a: &Point, b: &Point, c: &Point) -> f64 {
        (b - a).cross(&(c - a))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
        let res = self.x.partial_cmp(&other.x);
        if res.is_none() || res.is_some() && res.unwrap() == Ordering::Equal {
            return self.y.partial_cmp(&other.y);
        }
        res
    }
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let lines = input.lines().map(|x| x.unwrap());

    for line in lines {
        let mut xs = line.split(' ').map(|x| x.parse::<f64>().unwrap());
        let mut points = vec![];

        while let Some(x) = xs.next() {
            points.push(Point {
                x,
                y: xs.next().unwrap()
            })
        }

        let hull = graham(&mut points);
        let mut total: f64 = (hull[0] - hull[hull.len() - 1]).abs();

        for w in hull.windows(2) {
            total += (w[0] - w[1]).abs();
        }

        writeln!(&mut w, "{:.10}", 100.0 * (points.len() as f64 / (total + 1.0))).unwrap();
    }

    Ok(())
}

fn graham(points: &mut Vec<Point>) -> Vec<&Point> {
    points.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut s: Vec<&Point> = vec![];

    for p in points.iter() {
        while s.len() >= 2 && Point::orient(s[s.len() - 2], s[s.len() - 1], p) > 0. {
            s.pop();
        }
        s.push(p);
    }

    let mut hull = s;
    let mut s: Vec<&Point> = vec![];

    for p in points.iter().rev() {
        while s.len() >= 2 && Point::orient(s[s.len() - 2], s[s.len() - 1], p) > 0. {
            s.pop();
        }
        s.push(p);
    }

    hull.extend(s);

    hull
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dartscoring_sampleinputs() {
        for mut file in std::fs::read_dir("input/dartscoring")
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
