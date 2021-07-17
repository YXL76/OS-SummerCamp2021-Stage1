use std::io;

pub fn ex10<W: io::Write>(stdout: &mut W) {
    let tabby_cat = "\tI'm tabbed in.";
    let persian_cat = "I'm split\non a line.";
    let backslash_cat = "I'm \\ a \\ cat.";

    let fat_cat = "
I'll do a list:
\t* Cat food
\t* Fishies
\t* Catnip\n\t* Grass
";

    writeln!(stdout, "{}", tabby_cat).unwrap();
    writeln!(stdout, "{}", persian_cat).unwrap();
    writeln!(stdout, "{}", backslash_cat).unwrap();
    writeln!(stdout, "{}", fat_cat).unwrap();
}

fn main() {
    ex10(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex10_test() {
        let mut stdout = Vec::new();
        ex10(&mut stdout);

        assert_eq!(
            stdout,
            br"	I'm tabbed in.
I'm split
on a line.
I'm \ a \ cat.

I'll do a list:
	* Cat food
	* Fishies
	* Catnip
	* Grass

"
        );
    }
}
