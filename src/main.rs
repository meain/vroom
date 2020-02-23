use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::{env, process};

// TODO: g_, c, R, gu, gU, ;(repeat), visual mode, copy, paste, search, number repeat
// TODO: <c-a> and <c-x> to increment numbers, repeat with .
// TODO: maybe add multiline support

mod parser;
mod transform;
mod utils;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <transformation> <filename>", args[0]);
        process::exit(1);
    }

    let transformation = args[1].clone();

    let transforms = parser::parse_transforms(&transformation);

    if args[2] != "-" {
        let file = File::open(args[2].clone()).unwrap();
        let reader = BufReader::new(file);
        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let modified = transform::transform(&transforms, line);
            println!("{}", modified);
        }
    } else {
        let stdin = io::stdin();
        let mut has_next = true;
        let mut line = String::new();
        while has_next {
            match stdin.read_line(&mut line) {
                Ok(bytes) if bytes > 0 => {
                    line.pop();
                    let modified = transform::transform(&transforms, line.clone());
                    println!("{}", modified);
                    line.clear();
                    has_next = true;
                }
                Ok(_) => {
                    has_next = false;
                }
                Err(err) => return eprintln!("Error while reading stream. {}", err),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser;
    use crate::transform;

    #[test]
    fn check_replacements() {
        let checks = [
            ["hello world", "aomer h", "homer hello world"],
            ["hello world", "A again", "hello world again"],
            ["hello world", "A ag<esc>Imo ", "mo hello world ag"],
            ["hello world", "icruel ", "cruel hello world"],
            ["hello world", "Icruel ", "cruel hello world"],
            ["hello world", "llllIcruel ", "cruel hello world"],
            // w
            ["hello world", "wicruel ", "hello cruel world"],
            ["hello-ish world", "wicruel", "hellocruel-ish world"],
            ["hello_ish world", "wicruel ", "hello_ish cruel world"],
            ["hello world", "Wicruel ", "hello cruel world"],
            ["hello-ish world", "Wicruel ", "hello-ish cruel world"],
            ["hello_ish world", "Wicruel ", "hello_ish cruel world"],
            // e
            ["hello world", "eill", "hellllo world"],
            ["hello-ish world", "eill", "hellllo-ish world"],
            ["hello_ish world", "eill", "hello_isllh world"],
            ["hello world", "Eill", "hellllo world"],
            ["hello-ish world", "Eill", "hello-isllh world"],
            ["hello_ish world", "Eill", "hello_isllh world"],
            // b
            ["hello world", "$bill", "hello llworld"],
            ["hello world-ish", "$bill", "hello world-llish"],
            ["hello world_ish", "$bill", "hello llworld_ish"],
            ["hello world", "$Bill", "hello llworld"],
            ["hello world-ish", "$Bill", "hello llworld-ish"],
            ["hello world_ish", "$Bill", "hello llworld_ish"],
            // hjkl
            ["hello world", "$ill", "hello worllld"],
            ["hello world", "ll0ill", "llhello world"],
            ["hello world", "llil", "helllo world"],
            ["hello world", "hia", "ahello world"],
            ["hello world", "hhhhhhhia", "ahello world"],
            ["hello world", "hhllllhia", "helalo world"],
            // f and t
            ["hello world", "fwai", "hello wiorld"],
            ["hello world", "$Fwai", "hello wiorld"],
            ["hello world", "twai", "hello iworld"],
            ["hello world", "$Twai", "hello woirld"],
            // ^
            ["hello world", "^ia", "ahello world"],
            ["    hello world", "^ia", "    ahello world"],
            ["    hello world", "0ia", "a    hello world"],
            // r
            ["hello world", "ra", "aello world"],
            ["hello world", "llra", "healo world"],
            ["hello world", "llrarbhrc", "hcblo world"],
            // buggy
            ["lemon", "wa pie", "lemon pie"],
            ["lemon", "A pie<esc>biand ", "lemon and pie"],
        ];

        for check in checks.iter() {
            let line = check[0].to_string();
            let transforms = parser::parse_transforms(&check[1].to_string());
            let modified = transform::transform(&transforms, line);
            assert_eq!(modified, check[2])
        }
    }
}
