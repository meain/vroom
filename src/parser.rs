#[derive(Debug, Clone)]
pub enum Mode {
    Normal,
    Insert,
    // Command,  // need to add support for this
}

#[derive(Debug, Clone)]
pub enum Go {
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
    FirstNonSpace,
    ReplaceChar(char),
}

#[derive(Debug, Clone)]
pub enum Transform {
    Goto(Go),
    Insert(String),
}

pub fn parse_transforms(transformation: &String) -> Vec<Transform> {
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
                if ['f', 'F', 't', 'T', 'r'].contains(&first_char) {
                    if state.len() == 1 {
                        continue;
                    } else {
                        let second_char = state.chars().nth(1).unwrap();
                        match first_char {
                            'f' => transforms.push(Transform::Goto(Go::Find(second_char))),
                            'F' => transforms.push(Transform::Goto(Go::FindBack(second_char))),
                            't' => transforms.push(Transform::Goto(Go::Till(second_char))),
                            'T' => transforms.push(Transform::Goto(Go::TillBack(second_char))),
                            'r' => transforms.push(Transform::Goto(Go::ReplaceChar(second_char))),
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
                } else if state == "^".to_string() {
                    transforms.push(Transform::Goto(Go::FirstNonSpace));
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
