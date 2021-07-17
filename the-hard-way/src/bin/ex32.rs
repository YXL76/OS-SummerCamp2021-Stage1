use std::{fmt, io};

pub fn ex32<W: io::Write>(stdout: &mut W) {
    let the_count = [1, 2, 3, 4, 5];
    let fruits = ["apples", "oranges", "pears", "apricots"];

    enum Change<'a> {
        Str(&'a str),
        Number(i32),
    }
    impl<'a> fmt::Display for Change<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Self::Str(s) => write!(f, "{:?}", s),
                Self::Number(n) => write!(f, "{}", n),
            }
        }
    }
    let change = [
        Change::Number(1),
        Change::Str("pennies"),
        Change::Number(2),
        Change::Str("dimes"),
        Change::Number(3),
        Change::Str("quarters"),
    ];

    for number in the_count {
        writeln!(stdout, "This is count {}", number).unwrap();
    }

    for fruit in fruits {
        writeln!(stdout, "A fruit of type: {}", fruit).unwrap();
    }

    for i in change {
        writeln!(stdout, "I got {}", i).unwrap();
    }

    let mut elements = Vec::new();

    for i in 0..6 {
        writeln!(stdout, "Adding {} to the list.", i).unwrap();
        elements.push(i);
    }

    for i in elements {
        writeln!(stdout, "Element was: {}", i).unwrap();
    }
}

fn main() {
    ex32(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex32_test() {
        let mut stdout = Vec::new();
        ex32(&mut stdout);

        assert_eq!(
            stdout,
            br#"This is count 1
This is count 2
This is count 3
This is count 4
This is count 5
A fruit of type: apples
A fruit of type: oranges
A fruit of type: pears
A fruit of type: apricots
I got 1
I got "pennies"
I got 2
I got "dimes"
I got 3
I got "quarters"
Adding 0 to the list.
Adding 1 to the list.
Adding 2 to the list.
Adding 3 to the list.
Adding 4 to the list.
Adding 5 to the list.
Element was: 0
Element was: 1
Element was: 2
Element was: 3
Element was: 4
Element was: 5
"#
        );
    }
}
