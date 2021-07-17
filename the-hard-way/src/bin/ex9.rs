use std::io;

pub fn ex9<W: io::Write>(stdout: &mut W) {
    let days = "Mon Tue Wed Thu Fri Sat Sun";
    let months = "Jan\nFeb\nMar\nApr\nMay\nJun\nJul\nAug";

    writeln!(stdout, "Here are the days:  {}", days).unwrap();
    writeln!(stdout, "Here are the months:  {}", months).unwrap();
    writeln!(
        stdout,
        "
There's something going on here.
With the three double-quotes.
We'll be able to type as much as we like.
Even 4 lines if we want, or 5, or 6.
"
    )
    .unwrap();
}

fn main() {
    ex9(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex9_test() {
        let mut stdout = Vec::new();
        ex9(&mut stdout);

        assert_eq!(
            stdout,
            br"Here are the days:  Mon Tue Wed Thu Fri Sat Sun
Here are the months:  Jan
Feb
Mar
Apr
May
Jun
Jul
Aug

There's something going on here.
With the three double-quotes.
We'll be able to type as much as we like.
Even 4 lines if we want, or 5, or 6.

"
        );
    }
}
