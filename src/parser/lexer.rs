pub fn is_decimal_digit(chr: char) -> bool {
    return '0' <= chr && chr <= '9';
}

pub fn is_identifier(s: char) -> bool {
    if s == "" {
        return false;
    }

    let bytes = s.as_bytes();
    if !is_identifier_start(bytes[0] as char) {
        return false;
    }
    for ch in &bytes[1..] {
        if !is_identifier_part(*ch as char) {
            return false;
        }
    }

    return true;
}

#[inline]
fn is_identifier_start(chr: char) -> bool {
    return chr == '$'
        || chr == '_'
        || chr == '\\'
        || 'a' <= chr && chr <= 'z'
        || 'A' <= chr && chr <= 'Z'
        || chr >= 0x80 as char && is_id_start_unicode(chr);
}

#[inline]
fn is_identifier_part(chr: char) -> bool {
    todo!()
}
#[inline]
fn is_id_start_unicode(chr: char) -> bool {
    todo!()
}
