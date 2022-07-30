use kona_lex::*;

#[test]
fn space_and_tab_test() {
    assert_eq!(tokenize("     ").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::Whitespace),
    ]);

    assert_eq!(tokenize("\t\t\t\t\t").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::Whitespace),
    ]);

    assert_eq!(tokenize(" \t \t \t ").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::Whitespace),
    ]);
}

#[test]
fn line_break_test() {
    assert_eq!(tokenize("\n").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::Eol),
    ]);

    assert_eq!(tokenize("\r\n").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::Eol),
    ]);

    assert_eq!(tokenize("\r").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::Eol),
    ]);

    assert_eq!(tokenize("\n\n\n\n").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::Eol),
        TokenKind::Trivia(TriviaKind::Eol),
        TokenKind::Trivia(TriviaKind::Eol),
        TokenKind::Trivia(TriviaKind::Eol),
    ]);

    assert_eq!(tokenize("\r\r\r\r").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::Eol),
        TokenKind::Trivia(TriviaKind::Eol),
        TokenKind::Trivia(TriviaKind::Eol),
        TokenKind::Trivia(TriviaKind::Eol),
    ]);

    assert_eq!(tokenize("\n\r\r\n").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::Eol),
        TokenKind::Trivia(TriviaKind::Eol),
        TokenKind::Trivia(TriviaKind::Eol),
    ]);

    assert_eq!(tokenize(" \n ").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::Whitespace),
        TokenKind::Trivia(TriviaKind::Eol),
        TokenKind::Trivia(TriviaKind::Whitespace),
    ]);
}

#[test]
fn single_line_comment_test() {
    assert_eq!(tokenize("42 // some commit").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Lit(LitKind::Int),
        TokenKind::Trivia(TriviaKind::Whitespace),
        TokenKind::Trivia(TriviaKind::SingleLineComment),
    ]);

    assert_eq!(tokenize("42 //some commit").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Lit(LitKind::Int),
        TokenKind::Trivia(TriviaKind::Whitespace),
        TokenKind::Trivia(TriviaKind::SingleLineComment),
    ]);

    assert_eq!(tokenize("//////////").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::SingleLineComment),
    ]);

    assert_eq!(tokenize("// commit\n").map(|t| t.kind).collect::<Vec<_>>(), [
        TokenKind::Trivia(TriviaKind::SingleLineComment),
        TokenKind::Trivia(TriviaKind::Eol),
    ]);
}
