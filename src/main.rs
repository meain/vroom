use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
                if state == "A".to_string() {
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

fn find_next_word(line: &String, pos: usize, big: bool, e: bool) -> usize {
    let nonbreak = ['_']; // TODO: could be incomplete list
    let mut flag = false;

    for (i, ch) in line.chars().skip(pos).enumerate() {
        if flag {
            if ch.is_alphanumeric() || nonbreak.contains(&ch) {
                return i + pos + 1;
            }
        }
        if big {
            if ch == ' ' {
                if e {
                    return i + pos;
                }
                flag = true;
                continue;
            }
        } else {
            // TODO: need to also check for thing like '_'
            if !(ch.is_alphanumeric() || nonbreak.contains(&ch)) {
                if e {
                    return i + pos;
                }
                flag = true;
                continue;
            }
        }
    }
    return line.len();
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
                _ => {}
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
