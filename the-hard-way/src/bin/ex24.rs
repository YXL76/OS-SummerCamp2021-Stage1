use std::io;

fn secret_formula(started: i32) -> (i32, i32, i32) {
    let jelly_beans = started * 500;
    let jars = jelly_beans / 1000;
    let crates = jars / 100;
    (jelly_beans, jars, crates)
}
pub fn ex24<W: io::Write>(stdout: &mut W) {
    writeln!(stdout, "Let's practice everything.").unwrap();
    writeln!(
        stdout,
        "You\'d need to know \'bout escapes with \\ that do \n newlines and \t tabs."
    )
    .unwrap();

    let poem = "
\tThe lovely world
with logic so firmly planted
cannot discern \n the needs of love
nor comprehend passion from intuition
and requires an explanation
\n\t\twhere there is none.
";

    writeln!(stdout, "--------------").unwrap();
    writeln!(stdout, "{}", poem).unwrap();
    writeln!(stdout, "--------------").unwrap();

    let five = 10 - 2 + 3 - 6;
    writeln!(stdout, "This should be five: {}", five).unwrap();

    let start_point = 10000;
    let (beans, jars, crates) = secret_formula(start_point);

    writeln!(stdout, "With a starting point of: {}", start_point).unwrap();
    writeln!(
        stdout,
        "We'd have {} beans, {} jars, and {} crates.",
        beans, jars, crates
    )
    .unwrap();

    let start_point = start_point / 10;
    writeln!(stdout, "We can also do that this way:").unwrap();
    writeln!(
        stdout,
        "We'd have {} beans, {} jars, and {} crates.",
        secret_formula(start_point).0,
        secret_formula(start_point).1,
        secret_formula(start_point).2
    )
    .unwrap();
}

fn main() {
    ex24(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex24_test() {
        let mut stdout = Vec::new();
        ex24(&mut stdout);

        assert_eq!(
            stdout,
            br"Let's practice everything.
You'd need to know 'bout escapes with \ that do 
 newlines and 	 tabs.
--------------

	The lovely world
with logic so firmly planted
cannot discern 
 the needs of love
nor comprehend passion from intuition
and requires an explanation

		where there is none.

--------------
This should be five: 5
With a starting point of: 10000
We'd have 5000000 beans, 5000 jars, and 50 crates.
We can also do that this way:
We'd have 500000 beans, 500 jars, and 5 crates.
"
        );
    }
}
