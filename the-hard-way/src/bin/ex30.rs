use std::io;

pub fn ex30<W: io::Write>(stdout: &mut W) {
    let people = 30;
    let cars = 40;
    let buses = 15;

    if cars > people {
        writeln!(stdout, "We should take the cars.").unwrap();
    } else if cars < people {
        writeln!(stdout, "We should not take the cars.").unwrap();
    } else {
        writeln!(stdout, "We can't decide.").unwrap();
    }

    if buses > cars {
        writeln!(stdout, "That's too many buses.").unwrap();
    } else if buses < cars {
        writeln!(stdout, "Maybe we could take the buses.").unwrap();
    } else {
        writeln!(stdout, "We still can't decide.").unwrap();
    }

    if people > buses {
        writeln!(stdout, "Alright, let's just take the buses.").unwrap();
    } else {
        writeln!(stdout, "Fine, let's stay home then.").unwrap();
    }
}

fn main() {
    ex30(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex30_test() {
        let mut stdout = Vec::new();
        ex30(&mut stdout);

        assert_eq!(
            stdout,
            br"We should take the cars.
Maybe we could take the buses.
Alright, let's just take the buses.
"
        );
    }
}
