use std::{
    fs,
    io::{self, prelude::*, BufReader},
};

fn print_all<W: Write>(stdout: &mut W, f: &mut fs::File) {
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    writeln!(stdout, "{}", buffer).unwrap();
}

fn rewind(f: &mut fs::File) {
    f.seek(io::SeekFrom::Start(0)).unwrap();
}

fn print_a_line<W: Write>(
    stdout: &mut W,
    line_count: u32,
    lines: &mut io::Lines<BufReader<fs::File>>,
) {
    writeln!(stdout, "{} {}\n", line_count, lines.next().unwrap().unwrap()).unwrap();
}

pub fn ex20<W: Write>(stdout: &mut W) {
    let input_file = "ex20_sample.txt";

    let mut current_file = fs::File::open(input_file).unwrap();

    writeln!(stdout, "First let's print the whole file:\n").unwrap();

    print_all(stdout, &mut current_file);

    writeln!(stdout, "Now let's rewind, kind of like a tape.").unwrap();

    rewind(&mut current_file);

    writeln!(stdout, "Let's print three lines:").unwrap();

    let reader = BufReader::new(current_file);
    let mut lines = reader.lines();
    let mut current_line = 1;
    print_a_line(stdout, current_line, &mut lines);

    current_line += 1;
    print_a_line(stdout, current_line, &mut lines);

    current_line += 1;
    print_a_line(stdout, current_line, &mut lines);
}

fn main() {
    ex20(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex20_test() {
        let mut stdout = Vec::new();
        ex20(&mut stdout);

        assert_eq!(
            stdout,
            br"First let's print the whole file:

To all the people out there.
I say I don't like my hair.
I need to shave it off.

Now let's rewind, kind of like a tape.
Let's print three lines:
1 To all the people out there.

2 I say I don't like my hair.

3 I need to shave it off.

"
        );
    }
}
