// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

/// Returns true if the character is a inline space, `' '`, `'\t'`, `'\0'`,
/// `'\u{000B}'` or `'\u{000C}'`.
pub fn is_inline_space(c: char) -> bool {
    matches!(c, ' '/* U+0020 space */
              | '\t'/* U+0009 horizontal tab */
              | '\0'/* U+0000 NULL */
              | '\u{000B}'/* U+000B vertical tab */
              | '\u{000C}'/* U+000C form feed */)
}

/// Returns true if the character is a line break, `'\n'` or `'\r'`.
pub fn is_linebreak(c: char) -> bool {
    matches!(c, '\n'/* U+000A line feed */
              | '\r'/* U+000D carriage return */)
}

/// Returns true if the character can be the head of an alphanumeric identifier.
pub fn is_alpha_ident_head(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

/// Returns true if the character can be a part of an alphanumeric identifier.
pub fn is_alpha_ident_part(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

/// Returns true if the character can be the head of a symbolic identifier,
/// including `'!'`, `'%'`, `'&'`, `'$'`, `'+'`, `'-'`, `':'`, `'<'`, `'='`,
/// `'>'`, `'?'`, `'/'`, `'~'`, `'^'`, `'|'`, and `'*'`.
pub fn is_sym_ident(c: char) -> bool {
    matches!(c, '!' | '%' | '&' | '$' | '+' | '-' | ':' | '<'
              | '=' | '>' | '?' | '/' | '~' | '^' | '|' | '*')
}

/// Returns true if the character is a decimal digit.
pub fn is_digit(c: char) -> bool {
    matches!(c, '0'..='9')
}
