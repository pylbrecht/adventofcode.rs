use std::fs;

pub fn run() {
    let content = fs::read_to_string("input/day01.txt").unwrap();

    let total: isize = content
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .map(|mass| calc_required_fuel(mass))
        .sum();

    println!("{}", total);

    let total: isize = content
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .map(|mass| calc_total_required_fuel(mass))
        .sum();

    println!("{}", total);
}

fn calc_required_fuel(mass: usize) -> isize {
    (((mass / 3) as f32).floor() - 2.) as isize
}

fn calc_total_required_fuel(mass: usize) -> isize {
    let mut required_fuel = calc_required_fuel(mass);
    let mut total = 0;
    while required_fuel > 0 {
        total += required_fuel;
        required_fuel = calc_required_fuel(required_fuel as usize);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(calc_required_fuel(12), 2);
        assert_eq!(calc_required_fuel(14), 2);
        assert_eq!(calc_required_fuel(1969), 654);
        assert_eq!(calc_required_fuel(100756), 33583);
    }

    #[test]
    fn part2() {
        assert_eq!(calc_total_required_fuel(12), 2);
        assert_eq!(calc_total_required_fuel(14), 2);
        assert_eq!(calc_total_required_fuel(1969), 966);
        assert_eq!(calc_total_required_fuel(100756), 50346);
    }
}
