pub fn find_prev_word(line: &String, pos: usize, big: bool) -> usize {
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

pub fn find_next_word(line: &String, pos: usize, big: bool, e: bool) -> usize {
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
    return line.len() - 1;
}

pub fn find_char(line: &String, item: &char, pos: usize, t: bool) -> usize {
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
pub fn find_char_rev(line: &String, item: &char, pos: usize, t: bool) -> usize {
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

pub fn find_first_non_whitespace(line: &String) -> usize {
    for (i, ch) in line.chars().enumerate() {
        if ch != ' ' {
            return i;
        }
    }
    return 0;
}

pub fn find_last_non_whitespace(line: &String) -> usize {
    for (i, ch) in line.chars().rev().enumerate() {
        if ch != ' ' {
            return line.len() - i - 1;
        }
    }
    return line.len() - 1;
}
