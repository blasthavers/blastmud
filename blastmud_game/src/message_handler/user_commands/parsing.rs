use nom::{
    bytes::complete::{take_till1},
    character::complete::space0,
    IResult,
};

pub fn parse_command_name(input: &str) -> (&str, &str) {
    fn parse(input: &str) -> IResult<&str, &str> {
        let (input, _) = space0(input)?;
        let (input, cmd) = take_till1(|c| c == ' ' || c == '\n')(input)?;
        let (input, _) = space0(input)?;
        Ok((input, cmd))
    }
    match parse(input) {
        /* This parser only fails on empty  / whitespace only strings. */
        Err(_) => ("", ""),
        Ok((rest, command)) => (command, rest)
    }
}
