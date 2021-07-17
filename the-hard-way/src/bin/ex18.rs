use std::io;

fn print_two<W: io::Write>(stdout: &mut W, arg1: &str, arg2: &str) {
    writeln!(stdout, "arg1: {:?}, arg2: {:?}", arg1, arg2).unwrap();
}

fn print_two_again<W: io::Write>(stdout: &mut W, arg1: &str, arg2: &str) {
    writeln!(stdout, "arg1: {:?}, arg2: {:?}", arg1, arg2).unwrap();
}

fn print_one<W: io::Write>(stdout: &mut W, arg1: &str) {
    writeln!(stdout, "arg1: {:?}", arg1).unwrap();
}

fn print_none<W: io::Write>(stdout: &mut W) {
    writeln!(stdout, "I got nothin'.").unwrap();
}

pub fn ex18<W: io::Write>(stdout: &mut W) {
    print_two(stdout, "Zed", "Shaw");
    print_two_again(stdout, "Zed", "Shaw");
    print_one(stdout, "First!");
    print_none(stdout);
}

fn main() {
    ex18(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex18_test() {
        let mut stdout = Vec::new();
        ex18(&mut stdout);

        assert_eq!(
            stdout,
            br#"arg1: "Zed", arg2: "Shaw"
arg1: "Zed", arg2: "Shaw"
arg1: "First!"
I got nothin'.
"#
        );
    }
}
