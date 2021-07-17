use std::io;

pub fn ex33<W: io::Write>(stdout: &mut W) {
    let mut i = 0;
    let mut numbers = Vec::new();

    while i < 6 {
        writeln!(stdout, "At the top i is {}", i).unwrap();
        numbers.push(i);

        i += 1;
        writeln!(stdout, "Numbers now:  {:?}", numbers).unwrap();
        writeln!(stdout, "At the bottom i is {}", i).unwrap();
    }

    writeln!(stdout, "The numbers: ").unwrap();

    for num in numbers {
        writeln!(stdout, "{}", num).unwrap();
    }
}

fn main() {
    ex33(&mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex33_test() {
        let mut stdout = Vec::new();
        ex33(&mut stdout);

        assert_eq!(
            stdout,
            br"At the top i is 0
Numbers now:  [0]
At the bottom i is 1
At the top i is 1
Numbers now:  [0, 1]
At the bottom i is 2
At the top i is 2
Numbers now:  [0, 1, 2]
At the bottom i is 3
At the top i is 3
Numbers now:  [0, 1, 2, 3]
At the bottom i is 4
At the top i is 4
Numbers now:  [0, 1, 2, 3, 4]
At the bottom i is 5
At the top i is 5
Numbers now:  [0, 1, 2, 3, 4, 5]
At the bottom i is 6
The numbers: 
0
1
2
3
4
5
"
        );
    }
}
