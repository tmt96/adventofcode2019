use core::ops::Add;
use core::ops::Sub;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    fn manhattan_len(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Planet {
    location: Vector3,
    velocity: Vector3,
}

impl Planet {
    fn new(x: i64, y: i64, z: i64) -> Self {
        let velocity = Vector3 { x: 0, y: 0, z: 0 };
        let location = Vector3 { x, y, z };
        Self { location, velocity }
    }

    fn update_location(&self) -> Self {
        Self {
            location: self.location + self.velocity,
            velocity: self.velocity,
        }
    }

    fn apply_gravity(&self, other: &Self) -> Self {
        let diff_loc = self.location - other.location;
        let gravity = Vector3 {
            x: diff_loc.x.signum(),
            y: diff_loc.y.signum(),
            z: diff_loc.z.signum(),
        };
        Self {
            location: self.location,
            velocity: self.velocity - gravity,
        }
    }

    fn potential_energy(&self) -> i64 {
        self.location.manhattan_len()
    }

    fn kinetic_energy(&self) -> i64 {
        self.velocity.manhattan_len()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn take_one_turn(planets: &[Planet]) -> Vec<Planet> {
    planets
        .iter()
        .enumerate()
        .map(|(ind, planet)| {
            (0..planets.len())
                .filter(|&i| i != ind)
                .fold(*planet, |planet, i| planet.apply_gravity(&planets[i]))
        })
        .map(|planet| planet.update_location())
        .collect()
}

fn simulate(planets: &[Planet], steps: i64) -> i64 {
    (0..steps)
        .fold(planets.to_vec(), |planets, _| take_one_turn(&planets))
        .iter()
        .map(|planet| planet.total_energy())
        .sum()
}

fn part1(planets: &[Planet]) -> i64 {
    simulate(planets, 1000)
}

fn main() {
    let planets = vec![
        Planet::new(3, -6, 6),
        Planet::new(10, 7, -9),
        Planet::new(-3, -7, 9),
        Planet::new(-8, 0, 4),
    ];
    println!("part 1: {}", part1(&planets));
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        let planets = vec![
            Planet::new(-8, -10, 0),
            Planet::new(5, 5, 10),
            Planet::new(2, -7, 3),
            Planet::new(9, -8, -3),
        ];
        assert_eq!(simulate(&planets, 100), 1940)
    }

    #[test]
    fn test2() {
        let planets = vec![
            Planet::new(-1, 0, 2),
            Planet::new(2, -10, -7),
            Planet::new(4, -8, 8),
            Planet::new(3, 5, -1),
        ];
        assert_eq!(simulate(&planets, 10), 179)
    }
}
