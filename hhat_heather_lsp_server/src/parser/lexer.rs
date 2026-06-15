
use logos::{Logos, Lexer, FilterResult};


#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\r;,]+")]
enum Token {
    // starting with comments

    #[regex(r"\/\/[^\n]*")]
    SingleLineComment,
    #[token("/-", set_block_comment)]
    BlockComment,

    // some symbols

    /// return keyword is `::`
    #[token("::")]
    Return,

    /// variadic keyword is `...` for variadic arguments
    #[token("...")]
    Variadic,

    /// reference keyword is `&`
    #[token("&")]
    Ref,

    /// both pointer and cast keywords are `*`
    #[token("*")]
    PointerOrCast,

    /// colon keyword is `:`, used for type definition on
    /// variable declaration, expression and body on
    /// meta-functions, imports distinction
    #[token(":")]
    Colon,

    /// assignment keyword is `=`
    #[token("=")]
    Assign,

    /// dot keyword is `.`, used for imported objects path,
    /// objects attributes, types members/variants
    #[token(".")]
    Dot,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("{")]
    LCurly,

    #[token("}")]
    RCurly,

    #[token("<")]
    LAngle,

    #[token(">")]
    RAngle,

    // some named keywords

    /// to import something, the keyword is `use`
    #[token("use")]
    Use,
    #[token("type")]
    Type,
    #[token("const")]
    Const,
    #[token("fn")]
    Fn,
    #[token("metafn")]
    MetaFn,
    #[token("modifier")]
    Modifier,
    #[token("main")]
    Main,
    #[token("self")]
    SelfKwd,
    #[token("super-type")]
    SuperType,

    // regex for literals

    #[regex(r"[@]?(true|false)", set_bool_lit)]
    BoolLit(String),
    #[regex(r"\-?[@]?(0|[1-9])[0-9]*", set_int_lit)]
    IntLit(String),
    #[regex(r"\-?[@]?(0|[1-9][0-9]*)\.[0-9]+", set_float_lit)]
    FloatLit(String),
    #[regex(r#""([^"\\]|\\.)*""#, set_str_lit)]
    StrLit(String),

    // regex for identity

    /// identity token
    #[regex(r"[@]?[a-zA-Z][a-zA-Z0-9]*", set_id)]
    Id(String),
}


fn set_block_comment(lexer: &mut Lexer<Token>) -> FilterResult<(), ()> {
    todo!()
}


/// Define boolean literals, for both classical and quantum
///
fn set_bool_lit(lexer: &mut Lexer<Token>) -> Option<String> {
    let slice = lexer.slice();
    Some(String::from(slice))
}


fn set_int_lit(lexer: &mut Lexer<Token>) -> Option<String> {
    todo!()
}

fn set_float_lit(lexer: &mut Lexer<Token>) -> Option<String> {
    todo!()
}

fn set_str_lit(lexer: &mut Lexer<Token>) -> Option<String> {
    todo!()
}

fn set_id(lexer: &mut Lexer<Token>) -> Option<String> {
    todo!()
}
