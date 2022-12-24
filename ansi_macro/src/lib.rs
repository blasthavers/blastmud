use proc_macro::TokenStream;
use syn::{parse_macro_input, Lit};
use quote::ToTokens;
use nom::{
    combinator::eof,
    branch::alt, multi::fold_many0,
    bytes::complete::{take_till, take_till1, tag},
    sequence::{tuple, pair},
    error::Error,
    Err,
    Parser
};

enum AnsiFrag<'l> {
    Lit(&'l str),
    Special(&'l str)
}
use AnsiFrag::Special;

#[proc_macro]
pub fn ansi(input: TokenStream) -> TokenStream {
    let raw = match parse_macro_input!(input as Lit) {
        Lit::Str(lit_str) => lit_str.value(),
        _ => panic!("Expected a string literal")
    };
    fn parser(i: &str) -> Result<String, Err<Error<&str>>> {
        pair(fold_many0(
            alt((
                take_till1(|c| c == '<').map(AnsiFrag::Lit),
                tuple((tag("<"), take_till(|c| c == '>'), tag(">"))).map(|t| AnsiFrag::Special(t.1))
            )),
            || "".to_owned(),
            |a, r| a + match r {
                AnsiFrag::Lit(s) => &s,
                Special(s) if s == "reset" => "\x1b[0m",
                Special(s) if s == "bold" => "\x1b[1m",
                Special(s) if s == "under" => "\x1b[4m",
                Special(s) if s == "strike" => "\x1b[9m",
                Special(s) if s == "nounder" => "\x1b[24m",
                Special(s) if s == "black" => "\x1b[30m",
                Special(s) if s == "red" => "\x1b[31m",
                Special(s) if s == "green" => "\x1b[32m",
                Special(s) if s == "yellow" => "\x1b[33m",
                Special(s) if s == "blue" => "\x1b[34m",
                Special(s) if s == "magenta" => "\x1b[35m",
                Special(s) if s == "cyan" => "\x1b[36m",
                Special(s) if s == "white" => "\x1b[37m", 
                Special(s) if s == "bgblack" => "\x1b[40m",
                Special(s) if s == "bgred" => "\x1b[41m",
                Special(s) if s == "bggreen" => "\x1b[42m",
                Special(s) if s == "bgyellow" => "\x1b[43m",
                Special(s) if s == "bgblue" => "\x1b[44m",
                Special(s) if s == "bgmagenta" => "\x1b[45m",
                Special(s) if s == "bgcyan" => "\x1b[46m",
                Special(s) if s == "bgwhite" => "\x1b[47m",
                Special(s) if s == "lt" => "<",
                Special(r) => panic!("Unknown ansi type {}", r)
            }
        ), eof)(i).map(|(_, (r, _))| r)
    }
    TokenStream::from(parser(&raw)
                      .unwrap_or_else(|e| { panic!("Bad ansi literal: {}", e) })
                      .into_token_stream())
}
