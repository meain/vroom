use super::parser::{Go, Transform};
use super::utils;
use std::env;

#[allow(dead_code)]
fn debug(line: &str, pos: usize, transform: &Transform) {
    let max_line_length = 20; // change 20 to max line length
    let mut ft = format!("{:?}", transform);
    ft = format!("{: >1$}", ft, max_line_length - line.len() + ft.len());

    println!("{}¬ {} [{}]", line, ft, pos);
    println!("{:-<1$}^", "", pos);
}

pub fn transform(transforms: &Vec<Transform>, line: String) -> String {
    let mut pos: usize = 0;
    let mut modified = line.clone();

    let enable_debug = !env::var("VROOM_DEBUG").is_err();

    for transform in transforms {
        if enable_debug {
            debug(&modified, pos, &transform);
        }

        match transform {
            Transform::Insert(text) => {
                modified.insert_str(pos, text);
                pos += text.len();
            }
            Transform::InsertRight(text) => {
                pos += 1;
                modified.insert_str(pos, text);
                pos += text.len();
                pos -= 1;
            }
            Transform::Goto(p) => match p {
                Go::Start => pos = 0,
                Go::End => pos = modified.len() - 1,
                Go::Right => {
                    if modified.len() - 1 > pos {
                        pos += 1;
                    }
                }
                Go::Left => {
                    if pos > 0 {
                        pos -= 1;
                    }
                }
                Go::Word => pos = utils::find_next_word(&modified, pos, false, false),
                Go::BigWord => pos = utils::find_next_word(&modified, pos, true, false),
                Go::WordEnd => pos = utils::find_next_word(&modified, pos, false, true),
                Go::BigWordEnd => pos = utils::find_next_word(&modified, pos, true, true),
                Go::Back => pos = utils::find_prev_word(&modified, pos, false),
                Go::BigBack => pos = utils::find_prev_word(&modified, pos, true),
                Go::Find(c) => pos = utils::find_char(&modified, c, pos, false),
                Go::FindBack(c) => pos = utils::find_char_rev(&modified, c, pos, false),
                Go::Till(c) => pos = utils::find_char(&modified, c, pos, true),
                Go::TillBack(c) => pos = utils::find_char_rev(&modified, c, pos, true),
                Go::ReplaceChar(c) => {
                    modified.insert_str(pos, &c.to_string());
                    modified.remove(pos + 1);
                }
                Go::FirstNonSpace => pos = utils::find_first_non_whitespace(&modified),
                Go::LastNonSpace => pos = utils::find_last_non_whitespace(&modified),
            },
            Transform::None => {}
        }
    }
    modified
}
