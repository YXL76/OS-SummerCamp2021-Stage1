use std::io;

fn cheese_and_crackers<W: io::Write>(stdout: &mut W, cheese_count: u32, boxes_of_crackers: u32) {
    writeln!(stdout, "You have {} cheeses!", cheese_count).unwrap();
    writeln!(stdout, "You have {} boxes of crackers!", boxes_of_crackers).unwrap();
    writeln!(stdout, "Man that's enough for a party!").unwrap();
    writeln!(stdout, "Get a blanket.").unwrap();
    writeln!(stdout).unwrap();
}

pub fn ex19<W: io::Write>(stdout: &mut W) {
    writeln!(stdout, "We can just give the function numbers directly:").unwrap();
    cheese_and_crackers(stdout, 20, 30);

    writeln!(stdout, "OR, we can use variables from our script:").unwrap();
    let amount_of_cheese = 10;
    let amount_of_crackers = 50;
    cheese_and_crackers(stdout, amount_of_cheese, amount_of_crackers);

    writeln!(stdout, "We can even do math inside too:").unwrap();
    cheese_and_crackers(stdout, 10 + 20, 5 + 6);

    writeln!(stdout, "And we can combine the two, variables and math:").unwrap();
    cheese_and_crackers(stdout, amount_of_cheese + 100, amount_of_crackers + 1000);
}

fn main() {
    ex19(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex19_test() {
        let mut stdout = Vec::new();
        ex19(&mut stdout);

        assert_eq!(
            stdout,
            br"We can just give the function numbers directly:
You have 20 cheeses!
You have 30 boxes of crackers!
Man that's enough for a party!
Get a blanket.

OR, we can use variables from our script:
You have 10 cheeses!
You have 50 boxes of crackers!
Man that's enough for a party!
Get a blanket.

We can even do math inside too:
You have 30 cheeses!
You have 11 boxes of crackers!
Man that's enough for a party!
Get a blanket.

And we can combine the two, variables and math:
You have 110 cheeses!
You have 1050 boxes of crackers!
Man that's enough for a party!
Get a blanket.

"
        );
    }
}
