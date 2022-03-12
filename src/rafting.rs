use std::error::Error;
use std::io::{self, BufRead, Write};

/// This file is based on the Handbook of geometry for competitive programmers by Victor Lecomte
use std::ops::{Div, Mul, Sub, Add, Neg};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output=T> + Copy> Add for &Point<T> {
    type Output = Point<T>;

    fn add(self, other: Self) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output=T> + Copy> Sub for &Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Self) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Mul<Output=T> + Copy> Mul<T> for &Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Point<T> {
        Point {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<T: Div<Output=T> + Copy> Div<T> for &Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: T) -> Point<T> {
        Point {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl<T: Neg<Output=T> + Copy> Neg for &Point<T> {
    type Output = Point<T>;

    fn neg(self) -> Point<T> {
        Point {
            x: -self.x,
            y: -self.y
        }
    }
}

impl<T: Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Copy> Point<T> {
    fn square(&self) -> T {
        return self.x * self.x + self.y * self.y;
    }

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    fn cross(&self, other: &Self) -> T {
        self.x * other.y - self.y * other.x
    }
}

impl Point<f64> {
    fn abs(&self) -> f64 {
        self.square().sqrt()
    }

    fn orient(a: &Point<f64>, b: &Point<f64>, c: &Point<f64>) -> f64 {
        (b - a).cross(&(c - a))
    }
}

pub type Vector<T> = Point<T>;

pub struct Line<T> {
    v: Vector<T>,
    c: T
}

impl<T: Neg<Output=T> + Sub<Output=T> + Add<Output=T> + Mul<Output=T> + Copy + PartialOrd> Line<T> {
    fn from_points(p: &Point<T>, q: &Point<T>) -> Self {
        Self {
            v: q - p,
            c: (q - p).cross(&p)
        }
    }

    fn side(&self, p: &Point<T>) -> T {
        self.v.cross(&p) - self.c
    }

    fn cmp_proj(&self, p: &Point<T>, q: &Point<T>) -> bool {
        self.v.dot(p) < self.v.dot(q)
    }
}

impl Line<f64> {
    fn dist(&self, p: &Point<f64>) -> f64 {
        self.side(p).abs() / self.v.abs()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LineSegment<'a, 'b, T>(pub &'a Point<T>, pub &'b Point<T>);

impl LineSegment<'_, '_, f64> {
    pub fn proper_intersect(&self, other: &LineSegment<f64>) -> Option<Point<f64>> {
        let oa = Point::orient(other.0, other.1, self.0);
        let ob = Point::orient(other.0, other.1, self.1);
        let oc = Point::orient(self.0, self.1, other.0);
        let od = Point::orient(self.0, self.1, other.1);

        if oa * ob < 0.0 && oc * od < 0.0 {
            Some(&(&(self.0 * ob) - &(self.1 * oa)) / (ob - oa))
        } else {
            None
        }
    }

    pub fn distance_to_point(&self, other: &Point<f64>) -> f64 {
        if self.0 != other {
            let line = Line::from_points(self.0, self.1);
            if line.cmp_proj(self.0, other) && line.cmp_proj(other, self.1) {
                return line.dist(other);
            }
        }
        f64::min((other - self.0).abs(), (other - self.1).abs())
    }

    pub fn distance_to_segment(&self, other: &LineSegment<f64>) -> f64 {
        if self.proper_intersect(other).is_some() {
            0.0
        } else {
            f64::min(self.distance_to_point(other.0), f64::min(self.distance_to_point(other.1), f64::min(other.distance_to_point(self.0), other.distance_to_point(self.1))))
        }
    }
}

/// main entry point of program when invoked directly in the terminal
#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    solve(io::stdin().lock(), io::stdout().lock())
}

fn solve(input: impl BufRead, mut w: impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();
    let testcases = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    for _ in 0..testcases {
        let n_inner = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let inner_points: Vec<Point<f64>> = (&mut lines).take(n_inner).map(|l| l.unwrap()).map(|l| {
            let mut pathiter = l.split(' ').map(|x| x.parse::<isize>().unwrap());
            Point { x: pathiter.next().unwrap() as f64, y: pathiter.next().unwrap() as f64 }
        }).collect();

        let n_outer = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let outer_points: Vec<Point<f64>> = (&mut lines).take(n_outer).map(|l| l.unwrap()).map(|l| {
            let mut pathiter = l.split(' ').map(|x| x.parse::<isize>().unwrap());
            Point { x: pathiter.next().unwrap() as f64, y: pathiter.next().unwrap() as f64 }
        }).collect();

        let mut inner_segments: Vec<LineSegment<f64>> = inner_points.windows(2).map(|x| LineSegment(&x[0], &x[1])).collect();
        inner_segments.push(LineSegment(&inner_points[0], &inner_points[n_inner-1]));

        let mut outer_segments: Vec<LineSegment<f64>> = outer_points.windows(2).map(|x| LineSegment(&x[0], &x[1])).collect();
        outer_segments.push(LineSegment(&outer_points[0], &outer_points[n_outer-1]));

        let mut min_distance = std::f64::MAX;
        for inner_segment in inner_segments.iter() {
            for outer_segment in outer_segments.iter() {
                min_distance = f64::min(inner_segment.distance_to_segment(outer_segment), min_distance);
            }
        }

        writeln!(&mut w, "{}", min_distance / 2.0).unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rafting_sampleinputs() {
        for mut file in std::fs::read_dir("input/rafting")
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
            for (l1, l2) in std::str::from_utf8(&output_writer).unwrap().trim().lines().zip(std::fs::read_to_string(&file).unwrap().trim().lines()) {
                assert!((l1.parse::<f64>().unwrap() - l2.parse::<f64>().unwrap()).abs() < 10.0f64.powi(-6));
            }
        }
    }
}
