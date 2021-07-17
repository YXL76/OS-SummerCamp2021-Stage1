use std::io;

pub fn ex29<W: io::Write>(stdout: &mut W) {
    let people = 20;
    let cats = 30;
    let mut dogs = 15;

    if people < cats {
        writeln!(stdout, "Too many cats! The world is doomed!").unwrap();
    }
    if people > cats {
        writeln!(stdout, "Not many cats! The world is saved!").unwrap();
    }
    if people < dogs {
        writeln!(stdout, "The world is drooled on!").unwrap();
    }
    if people > dogs {
        writeln!(stdout, "The world is dry!").unwrap();
    }

    dogs += 5;

    if people >= dogs {
        writeln!(stdout, "People are greater than or equal to dogs.").unwrap();
    }
    if people <= dogs {
        writeln!(stdout, "People are less than or equal to dogs.").unwrap();
    }
    if people == dogs {
        writeln!(stdout, "People are dogs.").unwrap();
    }
}

fn main() {
    ex29(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex29_test() {
        let mut stdout = Vec::new();
        ex29(&mut stdout);

        assert_eq!(
            stdout,
            br"Too many cats! The world is doomed!
The world is dry!
People are greater than or equal to dogs.
People are less than or equal to dogs.
People are dogs.
"
        );
    }
}
