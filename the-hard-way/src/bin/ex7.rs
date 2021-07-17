use std::io;

pub fn ex7<W: io::Write>(stdout: &mut W) {
    writeln!(stdout, "Mary had a little lamb.").unwrap();
    writeln!(stdout, "Its fleece was white as {}.", "snow").unwrap();
    writeln!(stdout, "And everywhere that Mary went.").unwrap();
    writeln!(stdout, "{}", ".".repeat(10)).unwrap();

    let end1 = "C";
    let end2 = "h";
    let end3 = "e";
    let end4 = "e";
    let end5 = "s";
    let end6 = "e";
    let end7 = "B";
    let end8 = "u";
    let end9 = "r";
    let end10 = "g";
    let end11 = "e";
    let end12 = "r";

    write!(
        stdout,
        "{} ",
        end1.to_string() + end2 + end3 + end4 + end5 + end6
    )
    .unwrap();
    writeln!(
        stdout,
        "{}",
        end7.to_string() + end8 + end9 + end10 + end11 + end12
    )
    .unwrap();
}

fn main() {
    ex7(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex7_test() {
        let mut stdout = Vec::new();
        ex7(&mut stdout);

        assert_eq!(
            stdout,
            br"Mary had a little lamb.
Its fleece was white as snow.
And everywhere that Mary went.
..........
Cheese Burger
"
        );
    }
}
