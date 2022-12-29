use nom::{
    bytes::complete::{take_till1, take_while},
    character::{complete::{space0, space1, alpha1, one_of}},
    combinator::{recognize, fail, eof},
    sequence::terminated,
    branch::alt,
    error::{context, VerboseError, VerboseErrorKind},
    IResult,
};

pub fn parse_command_name(input: &str) -> (&str, &str) {
    fn parse(input: &str) -> IResult<&str, &str> {
        let (input, _) = space0(input)?;
        let (input, cmd) = alt((
            recognize(one_of("-\"':.")),
            take_till1(|c| c == ' ' || c == '\t')
        ))(input)?;
        let (input, _) = space0(input)?;
        Ok((input, cmd))
    }
    match parse(input) {
        /* This parser only fails on empty  / whitespace only strings. */
        Err(_) => ("", ""),
        Ok((rest, command)) => (command, rest)
    }
}

pub fn parse_username(input: &str) -> Result<(&str, &str), &'static str> {
    const CATCHALL_ERROR: &'static str = "Must only contain alphanumeric characters or _";
    fn parse_valid(input: &str) -> IResult<&str, (), VerboseError<&str>> {
        let (input, l1) = context("Must start with a letter", alpha1)(input)?;
        let (input, l2) = context(CATCHALL_ERROR,
                                  take_while(|c: char| c.is_alphanumeric() || c == '_'))(input)?;
        if l1.len() + l2.len() > 20 {
            context("Limit of 20 characters", fail::<&str, &str, VerboseError<&str>>)(input)?;
        }
        Ok((input, ()))
    }
    match terminated(recognize(parse_valid), alt((space1, eof)))(input) {
        Ok((input, username)) => Ok((username, input)),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) =>
            Err(e.errors.into_iter().find_map(|k| match k.1 {
                VerboseErrorKind::Context(s) => Some(s),
                _ => None
            }).unwrap_or(CATCHALL_ERROR)),
        Err(_) => Err(CATCHALL_ERROR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_parses_normal_command() {
        assert_eq!(parse_command_name("help"),
                   ("help", ""));
    }
    
    #[test]
    fn it_parses_normal_command_with_arg() {
        assert_eq!(parse_command_name("help \t    testing stuff"),
                   ("help", "testing stuff"));
    }

    #[test]
    fn it_parses_commands_with_leading_whitespace() {
        assert_eq!(parse_command_name("   \t  \thelp \t    testing stuff"),
                   ("help", "testing stuff"));
    }
    
    #[test]
    fn it_parses_empty_command_names() {
        assert_eq!(parse_command_name(""),
                   ("", ""));
        assert_eq!(parse_command_name(" \t "),
                   ("", ""));
    }

    #[test]
    fn it_parses_usernames() {
        assert_eq!(parse_username("Wizard123"), Ok(("Wizard123", "")));
    }

    #[test]
    fn it_parses_usernames_with_further_args() {
        assert_eq!(parse_username("Wizard_123 with cat"), Ok(("Wizard_123", "with cat")));
    }

    #[test]
    fn it_parses_alpha_only_usernames() {
        assert_eq!(parse_username("W"), Ok(("W", "")));
    }

    #[test]
    fn it_fails_on_empty_usernames() {
        assert_eq!(parse_username(""), Err("Must start with a letter"));
    }

    #[test]
    fn it_fails_on_usernames_with_invalid_start() {
        assert_eq!(parse_username("#hack"), Err("Must start with a letter"));
    }

    #[test]
    fn it_fails_on_usernames_with_underscore_start() {
        assert_eq!(parse_username("_hack"), Err("Must start with a letter"));
    }

    #[test]
    fn it_fails_on_usernames_with_number_start() {
        assert_eq!(parse_username("31337 #"), Err("Must start with a letter"));
    }

    #[test]
    fn it_fails_on_usernames_with_bad_characters() {
        assert_eq!(parse_username("Wizard!"), Err("Must only contain alphanumeric characters or _"));
    }

    #[test]
    fn it_fails_on_long_usernames() {
        assert_eq!(parse_username("A23456789012345678901"), Err("Limit of 20 characters"));
    }
}
