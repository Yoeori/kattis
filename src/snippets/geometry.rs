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

    fn is_perp(&self, other: &Self) -> bool {
        self.dot(other) == 0.0
    }

    fn angle(&self, other: &Self) -> f64 {
        let cos_theta = self.dot(other) / self.abs() / other.abs();
        f64::max(-1.0, f64::min(1.0, cos_theta)).acos()
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
    fn from_offset(v: Vector<T>, c: T) -> Self {
        Self {
            v, c
        }
    }

    fn from_equation(a: T, b: T, c: T) -> Self {
        Self {
            v: Vector { x: b, y: -a },
            c
        }
    }

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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn segment_distance() {
        let s1 = LineSegment(&Point { x: 0.0 , y: 0.0 }, &Point { x: 1.0 , y: 1.0 });
        let s2 = LineSegment(&Point { x: 0.0 , y: 1.0 }, &Point { x: 1.0 , y: 0.0 });
        assert_eq!(s1.proper_intersect(&s2).unwrap(), Point { x: 0.5, y: 0.5 });
    }
}