use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// TODO: ^, g_, c, r, gu, gU, ;(repeat), visual mode, copy, paste, search, number repeat
// TODO: maybe add multiline support

#[derive(Debug, Clone)]
enum Mode {
    Normal,
    Insert,
    // Command,  // need to add support for this
}

#[derive(Debug, Clone)]
enum Go {
    Right,
    Left,
    Start,
    End,
    Word,
    BigWord,
    WordEnd,
    BigWordEnd,
    Back,
    BigBack,
    Find(char),
    FindBack(char),
    Till(char),
    TillBack(char),
}

#[derive(Debug, Clone)]
struct Cursor {
    pos: usize,
    mode: Mode,
}

#[derive(Debug, Clone)]
enum Transform {
    Goto(Go),
    Insert(String),
}

fn parse_transforms(transformation: &String) -> Vec<Transform> {
    let mut transforms: Vec<Transform> = Vec::new();
    let mut state: String = "".to_string();
    let mut mode: Mode = Mode::Normal;
    for item in transformation.chars() {
        state = state + &item.to_string();
        // println!("state: {:?}", state);

        match mode {
            Mode::Insert => {
                if state.chars().next().unwrap() != '<' {
                    transforms.push(Transform::Insert(state.to_string()));
                    state = "".to_owned();
                } else if state == "<esc>".to_string() {
                    state = "".to_string();
                    mode = Mode::Normal;
                } else {
                    if state.len() > 5 {
                        transforms.push(Transform::Insert(state.to_string()));
                        state = "".to_owned();
                    } else {
                        // TODO: optimize this
                        let eqstate: String = "<esc>".chars().take(state.len()).collect();
                        if state != eqstate {
                            transforms.push(Transform::Insert(state.to_string()));
                            state = "".to_owned();
                        }
                    }
                }
            }
            Mode::Normal => {
                let first_char = state.chars().next().unwrap();
                if ['f', 'F', 't', 'T'].contains(&first_char) {
                    if state.len() == 1 {
                        continue;
                    } else {
                        let second_char = state.chars().nth(1).unwrap();
                        match first_char {
                            'f' => transforms.push(Transform::Goto(Go::Find(second_char))),
                            'F' => transforms.push(Transform::Goto(Go::FindBack(second_char))),
                            't' => transforms.push(Transform::Goto(Go::Till(second_char))),
                            'T' => transforms.push(Transform::Goto(Go::TillBack(second_char))),
                            _ => {}
                        }
                        state = "".to_string()
                    }
                } else if state == "$".to_string() {
                    transforms.push(Transform::Goto(Go::End));
                    transforms.push(Transform::Goto(Go::Left));
                    state = "".to_string();
                } else if state == "0".to_string() {
                    transforms.push(Transform::Goto(Go::Start));
                    state = "".to_string();
                } else if state == "A".to_string() {
                    transforms.push(Transform::Goto(Go::End));
                    mode = Mode::Insert;
                    state = "".to_string();
                } else if state == "I".to_string() {
                    transforms.push(Transform::Goto(Go::Start));
                    mode = Mode::Insert;
                    state = "".to_string();
                } else if state == "i".to_string() {
                    mode = Mode::Insert;
                    state = "".to_string();
                } else if state == "a".to_string() {
                    transforms.push(Transform::Goto(Go::Right));
                    mode = Mode::Insert;
                    state = "".to_string();
                } else if state == "l".to_string() {
                    transforms.push(Transform::Goto(Go::Right));
                    state = "".to_string();
                } else if state == "h".to_string() {
                    transforms.push(Transform::Goto(Go::Left));
                    state = "".to_string();
                } else if state == "w".to_string() {
                    transforms.push(Transform::Goto(Go::Word));
                    state = "".to_string();
                } else if state == "W".to_string() {
                    transforms.push(Transform::Goto(Go::BigWord));
                    state = "".to_string();
                } else if state == "e".to_string() {
                    transforms.push(Transform::Goto(Go::WordEnd));
                    state = "".to_string();
                } else if state == "E".to_string() {
                    transforms.push(Transform::Goto(Go::BigWordEnd));
                    state = "".to_string();
                } else if state == "b".to_string() {
                    transforms.push(Transform::Goto(Go::Back));
                    state = "".to_string();
                } else if state == "B".to_string() {
                    transforms.push(Transform::Goto(Go::BigBack));
                    state = "".to_string();
                }
            }
        }
    }
    return transforms;
}

fn find_prev_word(line: &String, pos: usize, big: bool) -> usize {
    // TODO: could avoid duplication
    let nonbreak = ['_']; // TODO: could be incomplete list
    let mut flag = false;

    for (i, ch) in line.chars().rev().skip(line.len() - pos).enumerate() {
        if flag {
            if ch.is_alphanumeric() || nonbreak.contains(&ch) {
                return pos - i + 1;
            }
        }
        if big {
            if ch == ' ' {
                flag = true;
                continue;
            }
        } else {
            // TODO: need to also check for thing like '_'
            if !(ch.is_alphanumeric() || nonbreak.contains(&ch)) {
                flag = true;
                continue;
            }
        }
    }
    return 0;
}

fn find_next_word(line: &String, pos: usize, big: bool, e: bool) -> usize {
    let nonbreak = ['_']; // TODO: could be incomplete list
    let mut flag = false;

    for (i, ch) in line.chars().skip(pos).enumerate() {
        if flag {
            if ch.is_alphanumeric() || nonbreak.contains(&ch) {
                return i + pos;
            }
        }
        if big {
            if ch == ' ' {
                if e {
                    return i + pos - 1;
                }
                flag = true;
                continue;
            }
        } else {
            // TODO: need to also check for thing like '_'
            if !ch.is_alphanumeric() && !nonbreak.contains(&ch) {
                if ch != ' ' {
                    if e {
                        return i + pos - 1;
                    }
                    return i + pos;
                }
                if e {
                    return i + pos - 1;
                }
                flag = true;
                continue;
            }
        }
    }
    return line.len();
}

fn find_char(line: &String, item: &char, pos: usize, t: bool) -> usize {
    for (i, ch) in line.chars().skip(pos).enumerate() {
        if &ch == item {
            if t {
                return i + pos - 1;
            } else {
                return i + pos;
            }
        }
    }
    return pos;
}
fn find_char_rev(line: &String, item: &char, pos: usize, t: bool) -> usize {
    for (i, ch) in line.chars().rev().skip(line.len() - pos).enumerate() {
        if &ch == item {
            if t {
                return pos - i;
            } else {
                return pos - i - 1;
            }
        }
    }
    return pos;
}

fn transform(transforms: &Vec<Transform>, line: String) -> String {
    let mut pos: usize = 0;
    let mut modified = line.clone();

    for transform in transforms {
        match transform {
            Transform::Insert(text) => {
                modified.insert_str(pos, text);
                pos += text.len();
            }
            Transform::Goto(p) => match p {
                Go::Start => pos = 0,
                Go::End => pos = modified.len(),
                Go::Right => pos += 1,
                Go::Left => {
                    if pos > 0 {
                        pos -= 1;
                    }
                }
                Go::Word => pos = find_next_word(&line, pos, false, false),
                Go::BigWord => pos = find_next_word(&line, pos, true, false),
                Go::WordEnd => pos = find_next_word(&line, pos, false, true),
                Go::BigWordEnd => pos = find_next_word(&line, pos, true, true),
                Go::Back => pos = find_prev_word(&line, pos, false),
                Go::BigBack => pos = find_prev_word(&line, pos, true),
                Go::Find(c) => pos = find_char(&line, c, pos, false),
                Go::FindBack(c) => pos = find_char_rev(&line, c, pos, false),
                Go::Till(c) => pos = find_char(&line, c, pos, true),
                Go::TillBack(c) => pos = find_char_rev(&line, c, pos, true),
            },
        }
    }
    modified
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Pass a transformation to do");
    }

    let transformation = args[1].clone();

    let file = File::open("test").unwrap();
    let reader = BufReader::new(file);

    let transforms = parse_transforms(&transformation);
    println!("transforms: {:?}", transforms);
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let modified = transform(&transforms, line);
        println!("{}", modified);
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_transforms, transform};

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
        ];

        for check in checks.iter() {
            let line = check[0].to_string();
            let transforms = parse_transforms(&check[1].to_string());
            let modified = transform(&transforms, line);
            assert_eq!(modified, check[2])
        }
    }
}
