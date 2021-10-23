use std::ops::Add;

enum Step {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

#[derive(Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

impl Add<Step> for Point {
    type Output = Self;

    fn add(self, step: Step) -> Self::Output {
        match step {
            Step::Up(count) => Point::new(self.x, self.y + count),
            Step::Down(count) => Point::new(self.x, self.y - count),
            Step::Left(count) => Point::new(self.x - count, self.y),
            Step::Right(count) => Point::new(self.x + count, self.y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_plus_step() {
        let point = Point::new(0, 0);
        let step = Step::Up(5);
        assert_eq!(point + step, Point::new(0, 5));
    }
}
