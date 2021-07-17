use std::io;

pub fn ex5<W: io::Write>(stdout: &mut W) {
    let my_name = "Zed A. Shaw";
    let my_age = 35;
    let my_height = 74;
    let my_weight = 180;
    let my_eyes = "Blue";
    let my_teeth = "White";
    let my_hair = "Brown";

    writeln!(stdout, "Let's talk about {}.", my_name).unwrap();
    writeln!(stdout, "He's {} inches tall.", my_height).unwrap();
    writeln!(stdout, "He's {} pounds heavy.", my_weight).unwrap();
    writeln!(stdout, "Actually that's not too heavy.").unwrap();
    writeln!(stdout, "He's got {} eyes and {} hair.", my_eyes, my_hair).unwrap();
    writeln!(
        stdout,
        "His teeth are usually {} depending on the coffee.",
        my_teeth
    )
    .unwrap();
    writeln!(
        stdout,
        "If I add {}, {}, and {} I get {}.",
        my_age,
        my_height,
        my_weight,
        my_age + my_height + my_weight
    )
    .unwrap();
}

fn main() {
    ex5(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex5_test() {
        let mut stdout = Vec::new();
        ex5(&mut stdout);

        assert_eq!(
            stdout,
            br"Let's talk about Zed A. Shaw.
He's 74 inches tall.
He's 180 pounds heavy.
Actually that's not too heavy.
He's got Blue eyes and Brown hair.
His teeth are usually White depending on the coffee.
If I add 35, 74, and 180 I get 289.
"
        );
    }
}
