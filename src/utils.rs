pub fn is_digit(ch: char) -> bool {
    matches!(ch, '0'..='9')
}

pub fn is_alpha(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '_')
}

pub fn is_alphanumeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}
