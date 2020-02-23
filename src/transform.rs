use super::utils;
use super::parser::{Go, Transform};

pub fn transform(transforms: &Vec<Transform>, line: String) -> String {
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
                Go::Word => pos = utils::find_next_word(&line, pos, false, false),
                Go::BigWord => pos = utils::find_next_word(&line, pos, true, false),
                Go::WordEnd => pos = utils::find_next_word(&line, pos, false, true),
                Go::BigWordEnd => pos = utils::find_next_word(&line, pos, true, true),
                Go::Back => pos = utils::find_prev_word(&line, pos, false),
                Go::BigBack => pos = utils::find_prev_word(&line, pos, true),
                Go::Find(c) => pos = utils::find_char(&line, c, pos, false),
                Go::FindBack(c) => pos = utils::find_char_rev(&line, c, pos, false),
                Go::Till(c) => pos = utils::find_char(&line, c, pos, true),
                Go::TillBack(c) => pos = utils::find_char_rev(&line, c, pos, true),
                Go::FirstNonSpace => pos = utils::find_first_non_whitespace(&line),
                Go::ReplaceChar(c) => {
                    modified.insert_str(pos, &c.to_string());
                    modified.remove(pos + 1);
                }
            },
        }
    }
    modified
}
