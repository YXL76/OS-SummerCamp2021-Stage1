use std::io;

pub fn ex4<W: io::Write>(stdout: &mut W) {
    let cars = 100;
    let space_in_a_car = 4.0;
    let drivers = 30;
    let passengers = 90;
    let cars_not_driven = cars - drivers;
    let cars_driven = drivers;
    let carpool_capacity = cars_driven as f64 * space_in_a_car;
    let average_passengers_per_car = passengers / cars_driven;

    writeln!(stdout, "There are {} cars available.", cars).unwrap();
    writeln!(stdout, "There are only {} drivers available.", drivers).unwrap();
    writeln!(
        stdout,
        "There will be {} empty cars today.",
        cars_not_driven
    )
    .unwrap();
    writeln!(
        stdout,
        "We can transport {:.1} people today.",
        carpool_capacity
    )
    .unwrap();
    writeln!(stdout, "We have {} to carpool today.", passengers).unwrap();
    writeln!(
        stdout,
        "We need to put about {} in each car.",
        average_passengers_per_car
    )
    .unwrap();
}

fn main() {
    ex4(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex4_test() {
        let mut stdout = Vec::new();
        ex4(&mut stdout);

        assert_eq!(
            stdout,
            br"There are 100 cars available.
There are only 30 drivers available.
There will be 70 empty cars today.
We can transport 120.0 people today.
We have 90 to carpool today.
We need to put about 3 in each car.
"
        );
    }
}
