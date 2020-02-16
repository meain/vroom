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
}

#[derive(Debug, Clone)]
struct Cursor {
    pos: usize,
    mode: Mode,
}

#[derive(Debug, Clone)]
enum Transform {
    Goto(Go),
    SwitchTo(Mode), // probably remove this
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
                    transforms.push(Transform::SwitchTo(Mode::Normal));
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
                    transforms.push(Transform::SwitchTo(Mode::Insert));
                    mode = Mode::Insert;
                    state = "".to_string();
                } else if state == "I".to_string() {
                    transforms.push(Transform::Goto(Go::Start));
                    transforms.push(Transform::SwitchTo(Mode::Insert));
                    mode = Mode::Insert;
                    state = "".to_string();
                }
            }
        }
    }
    return transforms;
}

fn transform(transforms: &Vec<Transform>, line: String) -> String {
    let mut pos = 0;
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
            },
            Transform::SwitchTo(_) => {}
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
