use std::io;

fn add<W: io::Write>(stdout: &mut W, a: i32, b: i32) -> i32 {
    writeln!(stdout, "ADDING {} + {}", a, b).unwrap();
    a + b
}

fn subtract<W: io::Write>(stdout: &mut W, a: i32, b: i32) -> i32 {
    writeln!(stdout, "SUBTRACTING {} - {}", a, b).unwrap();
    a - b
}

fn multiply<W: io::Write>(stdout: &mut W, a: i32, b: i32) -> i32 {
    writeln!(stdout, "MULTIPLYING {} * {}", a, b).unwrap();
    a * b
}

fn divide<W: io::Write>(stdout: &mut W, a: i32, b: i32) -> i32 {
    writeln!(stdout, "DIVIDING {} / {}", a, b).unwrap();
    a / b
}

pub fn ex21<W: io::Write>(stdout: &mut W) {
    writeln!(stdout, "Let's do some math with just functions!").unwrap();

    let age = add(stdout, 30, 5);
    let height = subtract(stdout, 78, 4);
    let weight = multiply(stdout, 90, 2);
    let iq = divide(stdout, 100, 2);

    writeln!(
        stdout,
        "Age: {}, Height: {}, Weight: {}, IQ: {}",
        age, height, weight, iq
    )
    .unwrap();

    writeln!(stdout, "Here is a puzzle.").unwrap();

    let what = divide(stdout, iq, 2);
    let what = multiply(stdout, weight, what);
    let what = subtract(stdout, height, what);
    let what = add(stdout, age, what);

    writeln!(stdout, "That becomes:  {} Can you do it by hand?", what).unwrap();
}

fn main() {
    ex21(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex21_test() {
        let mut stdout = Vec::new();
        ex21(&mut stdout);

        assert_eq!(
            stdout,
            br"Let's do some math with just functions!
ADDING 30 + 5
SUBTRACTING 78 - 4
MULTIPLYING 90 * 2
DIVIDING 100 / 2
Age: 35, Height: 74, Weight: 180, IQ: 50
Here is a puzzle.
DIVIDING 50 / 2
MULTIPLYING 180 * 25
SUBTRACTING 74 - 4500
ADDING 35 + -4426
That becomes:  -4391 Can you do it by hand?
"
        );
    }
}
