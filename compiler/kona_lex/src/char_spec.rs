// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

/// Returns true if the character is a inline space, space (U+0020) or
/// horizontal tab (U+0009).
pub fn is_inline_space(c: char) -> bool {
    matches!(c, '\x20'/* U+0020 space */
              | '\x09'/* U+0009 horizontal tab */)
}

/// Returns true if the character is a whitespace, space (U+0020), line feed
/// (U+000A), carriage return (U+000D), horizontal tab (U+0009), vertical tab
/// (U+000B), form feed (U+000C) and null (U+0000).
#[allow(dead_code)]
pub fn is_whitespace(c: char) -> bool {
    // FIXME: Reconsider the definition of whitespace.
    matches!(c, '\x20'/* U+0020 space */
              | '\x0A'/* U+000A line feed */
              | '\x0D'/* U+000D carriage return */
              | '\x09'/* U+0009 horizontal tab */
              | '\x0B'/* U+000B vertical tab */
              | '\x0C'/* U+000C form feed */
              | '\x00'/* U+0000 null */)
}

/// Returns true if the character is a line break, line feed (U+000A) or
/// carriage return (U+000D).
pub fn is_line_break(c: char) -> bool {
    // Don use the name `is_eol` here because 'end of line' is CR, LF or CRLF,
    // not a single character.
    matches!(c, '\x0A'/* U+000A line feed */
              | '\x0D'/* U+000D carriage return */)
}

/// Returns true if the character can be the head of an identifier.
pub fn is_ident_head(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

/// Returns true if the character can be a part of an identifier.
pub fn is_ident_part(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

/// Returns true if the character can be a part of an operator, including `!`,
/// `$`, `%`, `&`, `*`, `+`, `-`, `/`, `:`, `<`, `=`, `>`, `?`, `^`, `|`,
/// and `~`.
pub fn is_operator_part(c: char) -> bool {
    matches!(c, '!' | '$' | '%' | '&' | '*' | '+' | '-' | '/'
              | ':' | '<' | '=' | '>' | '?' | '^' | '|' | '~')
}

/// Returns true if the character is a decimal digit.
pub fn is_digit(c: char) -> bool {
    matches!(c, '0'..='9')
}
