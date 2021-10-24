use std::ops::Add;

pub fn run() {
}

#[derive(Debug)]
enum Step {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
            Step::Up(count) => Point::new(self.x, self.y + count as isize),
            Step::Down(count) => Point::new(self.x, self.y - count as isize),
            Step::Left(count) => Point::new(self.x - count as isize, self.y),
            Step::Right(count) => Point::new(self.x + count as isize, self.y),
        }
    }
}

struct Wire {
    path: Vec<Point>,
}

impl Wire {
    fn from_instructions(instructions: &str) -> Wire {
        let mut cursor = Point::new(0, 0);
        let mut path = vec![cursor];

        for instruction in instructions.split(',') {
            let direction = instruction.chars().nth(0).unwrap();
            let count = instruction.chars().nth(1).unwrap();

            let step = match direction {
                'U' => Step::Up(count.to_digit(10).unwrap()),
                'D' => Step::Down(count.to_digit(10).unwrap()),
                'L' => Step::Left(count.to_digit(10).unwrap()),
                'R' => Step::Right(count.to_digit(10).unwrap()),
                _ => panic!("unknown direction: {}", direction),
            };
            cursor = cursor + step;
            path.push(cursor);
        }

        Wire { path }
    }

    fn path(&self) -> &[Point] {
        &self.path
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

    #[test]
    fn create_wire_from_instructions() {
        let instructions = "U8,R4";
        let wire = Wire::from_instructions(instructions);
        assert_eq!(wire.path(), [Point::new(0, 0), Point::new(0, 8), Point::new(4, 8)]);
    }
}
