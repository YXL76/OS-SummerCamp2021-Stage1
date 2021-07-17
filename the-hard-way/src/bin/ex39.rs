use std::io;

pub fn ex39<W: io::Write>(stdout: &mut W) {
    let ten_things = "Apples Oranges Crows Telephone Light Sugar";

    writeln!(
        stdout,
        "Wait there's not 10 things in that list, let's fix that."
    )
    .unwrap();

    let mut stuff = ten_things.split(' ').collect::<Vec<&str>>();
    let mut more_stuff = vec![
        "Day", "Night", "Song", "Frisbee", "Corn", "Banana", "Girl", "Boy",
    ];

    while stuff.len() != 10 {
        let next_one = more_stuff.pop().unwrap();
        writeln!(stdout, "Adding:  {}", next_one).unwrap();
        stuff.push(next_one);
        writeln!(stdout, "There's {} items now.", stuff.len()).unwrap();
    }

    writeln!(stdout, "There we go:  {:?}", stuff).unwrap();

    writeln!(stdout, "Let's do some things with stuff.").unwrap();

    writeln!(stdout, "{}", stuff[1]).unwrap();
    writeln!(stdout, "{}", stuff[stuff.len() - 1]).unwrap();
    writeln!(stdout, "{}", stuff.pop().unwrap()).unwrap();
    writeln!(stdout, "{}", stuff.join(" ")).unwrap();
    writeln!(stdout, "{}", stuff[3..5].join("#")).unwrap();
}

fn main() {
    ex39(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex39_test() {
        let mut stdout = Vec::new();
        ex39(&mut stdout);

        assert_eq!(
            stdout,
            br#"Wait there's not 10 things in that list, let's fix that.
Adding:  Boy
There's 7 items now.
Adding:  Girl
There's 8 items now.
Adding:  Banana
There's 9 items now.
Adding:  Corn
There's 10 items now.
There we go:  ["Apples", "Oranges", "Crows", "Telephone", "Light", "Sugar", "Boy", "Girl", "Banana", "Corn"]
Let's do some things with stuff.
Oranges
Corn
Corn
Apples Oranges Crows Telephone Light Sugar Boy Girl Banana
Telephone#Light
"#
        );
    }
}
