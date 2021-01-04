use super::{ScanningError::*, TokenKind::*, *};

const LEXEME_KINDS: [(&str, TokenKind); 49] = [
    ("(", LeftParen),
    (")", RightParen),
    ("{", LeftBrace),
    ("}", RightBrace),
    ("[", LeftBracket),
    ("]", RightBracket),
    (",", Comma),
    (".", Dot),
    ("?", Question),
    (":", Colon),
    (";", Semicolon),
    ("-", Minus),
    ("-=", MinusEqual),
    ("--", MinusMinus),
    ("+", Plus),
    ("+=", PlusEqual),
    ("++", PlusPlus),
    ("*", Star),
    ("*=", StarEqual),
    ("%", Percent),
    ("%=", PercentEqual),
    ("/", Slash),
    ("/=", SlashEqual),
    ("!", Bang),
    ("!=", BangEqual),
    ("=", Equal),
    ("==", EqualEqual),
    ("<", Less),
    ("<=", LessEqual),
    (">", Greater),
    (">=", GreaterEqual),
    ("and", And),
    ("break", Break),
    ("class", Class),
    ("continue", Continue),
    ("else", Else),
    ("false", False),
    ("for", For),
    ("fun", Fun),
    ("if", If),
    ("nil", Nil),
    ("or", Or),
    ("print", Print),
    ("return", Return),
    ("super", Super),
    ("this", This),
    ("true", True),
    ("var", Var),
    ("while", While),
];

fn get_tokens<'a>(input: &'a str) -> Result<Vec<Token<'a>>, ScanningError> {
    let mut scanner = Scanner::new(input);
    let mut tokens = Vec::new();
    loop {
        let token = scanner.scan_token()?;
        let kind = token.kind;
        tokens.push(token);

        if kind == TokenKind::EOF {
            break Ok(tokens);
        }
    }
}

fn token<'a, L: Into<Loc>>(kind: TokenKind, lexeme: &'a str, loc: L) -> Token<'a> {
    Token {
        kind,
        lexeme,
        loc: loc.into(),
    }
}

fn eof_token<L: Into<Loc>>(loc: L) -> Token<'static> {
    token(EOF, "", loc)
}

fn no_token(line: usize, column: usize) -> Vec<Token<'static>> {
    vec![eof_token((line, column))]
}

fn one_token(token: Token) -> Vec<Token> {
    let mut loc = token.loc;
    loc.column += token.lexeme.len();
    vec![token, eof_token(loc)]
}

#[test]
fn test_empty_input() {
    let tokens = get_tokens("");
    assert_eq!(Ok(no_token(0, 0)), tokens);
}

#[test]
fn test_single_token() {
    for (lexeme, kind) in LEXEME_KINDS.iter() {
        let tokens = get_tokens(lexeme);
        let expected_token = token(*kind, lexeme, (0, 0));
        assert_eq!(Ok(one_token(expected_token)), tokens);
    }
}

#[test]
fn test_line_comment() {
    let tokens = get_tokens("// a line comment");
    assert_eq!(Ok(no_token(0, 17)), tokens);
}

#[test]
fn test_block_comment() {
    let input = r#"/* This is a block
        comment */"#;
    let tokens = get_tokens(input);
    assert_eq!(Ok(no_token(1, 18)), tokens);
}

#[test]
fn test_multiple_tokens() {
    let tokens = get_tokens("(){},.-+;*!=!%:==<=</>>=?=//this should be ignored");
    let expected_tokens = vec![
        token(LeftParen, "(", (0, 0)),
        token(RightParen, ")", (0, 1)),
        token(LeftBrace, "{", (0, 2)),
        token(RightBrace, "}", (0, 3)),
        token(Comma, ",", (0, 4)),
        token(Dot, ".", (0, 5)),
        token(Minus, "-", (0, 6)),
        token(Plus, "+", (0, 7)),
        token(Semicolon, ";", (0, 8)),
        token(Star, "*", (0, 9)),
        token(BangEqual, "!=", (0, 10)),
        token(Bang, "!", (0, 12)),
        token(Percent, "%", (0, 13)),
        token(Colon, ":", (0, 14)),
        token(EqualEqual, "==", (0, 15)),
        token(LessEqual, "<=", (0, 17)),
        token(Less, "<", (0, 19)),
        token(Slash, "/", (0, 20)),
        token(Greater, ">", (0, 21)),
        token(GreaterEqual, ">=", (0, 22)),
        token(Question, "?", (0, 24)),
        token(Equal, "=", (0, 25)),
        eof_token((0, 50)),
    ];
    assert_eq!(Ok(expected_tokens), tokens);
}

#[test]
fn test_string() {
    let input = r#""this is a string""#;
    let tokens = get_tokens(&input);
    let expected_token = token(Str, &input, (0, 0));
    assert_eq!(Ok(one_token(expected_token)), tokens);
}

#[test]
fn test_string2() {
    let input = r#"("this is a string")"#;
    let tokens = get_tokens(&input);
    let expected_tokens = vec![
        token(LeftParen, "(", (0, 0)),
        token(Str, r#""this is a string""#, (0, 1)),
        token(RightParen, ")", (0, 19)),
        eof_token((0, 20)),
    ];
    assert_eq!(Ok(expected_tokens), tokens);
}

#[test]
fn test_escaped_string() {
    let input = r#""this\nis\ta \" string\\""#;
    let tokens = get_tokens(&input);
    let expected_token = token(Str, input, (0, 0));
    assert_eq!(Ok(one_token(expected_token)), tokens);
}

#[test]
fn test_integer() {
    let input = "1234";
    let tokens = get_tokens(&input);
    let expected_token = token(Integer, input, (0, 0));
    assert_eq!(Ok(one_token(expected_token)), tokens);
}

#[test]
fn test_float() {
    for input in &[
        "1234.567",
        "1234.567e2",
        "1234.567e+2",
        "1234.567e-2",
        "12e12",
    ] {
        let tokens = get_tokens(input);
        let expected_token = token(Float, input, (0, 0));
        assert_eq!(Ok(one_token(expected_token)), tokens);
    }
}

#[test]
fn test_identifier() {
    for input in &[
        "hello",
        "hello_world",
        "h1",
        "r",
        "anda",
        "CamelCase",
        "_underscore",
    ] {
        let tokens = get_tokens(&input);
        let expected_token = token(Identifier, input, (0, 0));
        assert_eq!(Ok(one_token(expected_token)), tokens);
    }
}

#[test]
fn test_big_input() {
    let input = r#"// This is a comment
        var hello = "world";
        var a = (b + c - d) * e/1.0

        /* this is a
         * block comment */
        fun my_function(something) {
            print something;
            return nil;
        }

        if(i == 0 and j != 3) {
            2.sqrt()
        }
        "#;
    let tokens = get_tokens(&input);
    let expected_tokens = vec![
        // 2nd line
        token(Var, "var", (1, 8)),
        token(Identifier, "hello", (1, 12)),
        token(Equal, "=", (1, 18)),
        token(Str, "\"world\"", (1, 20)),
        token(Semicolon, ";", (1, 27)),
        // 3rd line
        token(Var, "var", (2, 8)),
        token(Identifier, "a", (2, 12)),
        token(Equal, "=", (2, 14)),
        token(LeftParen, "(", (2, 16)),
        token(Identifier, "b", (2, 17)),
        token(Plus, "+", (2, 19)),
        token(Identifier, "c", (2, 21)),
        token(Minus, "-", (2, 23)),
        token(Identifier, "d", (2, 25)),
        token(RightParen, ")", (2, 26)),
        token(Star, "*", (2, 28)),
        token(Identifier, "e", (2, 30)),
        token(Slash, "/", (2, 31)),
        token(Float, "1.0", (2, 32)),
        // 7th line
        token(Fun, "fun", (6, 8)),
        token(Identifier, "my_function", (6, 12)),
        token(LeftParen, "(", (6, 23)),
        token(Identifier, "something", (6, 24)),
        token(RightParen, ")", (6, 33)),
        token(LeftBrace, "{", (6, 35)),
        // 8th line
        token(Print, "print", (7, 12)),
        token(Identifier, "something", (7, 18)),
        token(Semicolon, ";", (7, 27)),
        // 9th line
        token(Return, "return", (8, 12)),
        token(Nil, "nil", (8, 19)),
        token(Semicolon, ";", (8, 22)),
        // 10th line
        token(RightBrace, "}", (9, 8)),
        // 12th line
        token(If, "if", (11, 8)),
        token(LeftParen, "(", (11, 10)),
        token(Identifier, "i", (11, 11)),
        token(EqualEqual, "==", (11, 13)),
        token(Integer, "0", (11, 16)),
        token(And, "and", (11, 18)),
        token(Identifier, "j", (11, 22)),
        token(BangEqual, "!=", (11, 24)),
        token(Integer, "3", (11, 27)),
        token(RightParen, ")", (11, 28)),
        token(LeftBrace, "{", (11, 30)),
        // 13th line
        token(Integer, "2", (12, 12)),
        token(Dot, ".", (12, 13)),
        token(Identifier, "sqrt", (12, 14)),
        token(LeftParen, "(", (12, 18)),
        token(RightParen, ")", (12, 19)),
        // 14th line
        token(RightBrace, "}", (13, 8)),
        // End
        eof_token((14, 8)),
    ];

    for (i, token) in tokens.unwrap().into_iter().enumerate() {
        assert_eq!(token, expected_tokens[i]);
    }
}

#[test]
fn test_error1() {
    let tokens = get_tokens("invalid_character¬");
    assert_eq!(Err(UnrecognizedCharacter('¬', Loc::new(0, 17))), tokens);
}

#[test]
fn test_error2() {
    let tokens = get_tokens("\"unterminated");
    assert_eq!(Err(UnterminatedString(Loc::new(0, 13))), tokens);
}

#[test]
fn test_error4() {
    let tokens = get_tokens("/* /* */");
    assert_eq!(Err(UnterminatedBlockComment(Loc::new(0, 8))), tokens);
}
