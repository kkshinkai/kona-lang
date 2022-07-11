// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

pub fn is_inline_space(c: char) -> bool {
    matches!(c, ' '/* U+0020 space */
              | '\t'/* U+0009 horizontal tab */
              | '\0'/* U+0000 NULL */
              | '\u{000B}'/* U+000B vertical tab */
              | '\u{000C}'/* U+000C form feed */)
}

pub fn is_linebreak(c: char) -> bool {
    matches!(c, '\n'/* U+000A line feed */
              | '\r'/* U+000D carriage return */)
}

#[deprecated]
pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t'
}

#[deprecated]
pub fn is_eol(c: char) -> bool {
    c == '\n' || c == '\r'
}

pub fn is_alpha_ident_head(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

pub fn is_alpha_ident_body(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

pub fn is_sym_ident(c: char) -> bool {
    matches!(c, '!' | '%' | '&' | '$' | '+' | '-' | ':' | '<'
              | '=' | '>' | '?' | '/' | '~' | '^' | '|' | '*')
}

pub fn is_digit(c: char) -> bool {
    matches!(c, '0'..='9')
}
