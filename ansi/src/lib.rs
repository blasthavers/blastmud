pub use ansi_macro::ansi;
use std::rc::Rc;

/// Removes all non-printable characters except tabs and newlines.
/// Doesn't attempt to remove printable characters as part of an
/// escape - so use this for untrusted input that you don't expect
/// to contain ansi escapes at all.
pub fn ignore_special_characters(input: &str) -> String {
    input.chars().filter(|c| *c == '\t' || *c == '\n' ||
                             (*c >= ' ' && *c <= '~')).collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct AnsiState {
    col: u64,
    background: u64, // 0 means default.
    foreground: u64,
    bold: bool,
    underline: bool,
    strike: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct AnsiEvent<'l> (
    AnsiParseToken<'l>,
    Rc<AnsiState>
);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum AnsiParseToken<'l> {
    Character(char),
    ControlSeq(&'l str),
    Newline,
}

/// Emits events with only LF, spaces, tabs, and a small set of
/// character attributes (colours, bold, underline). Anything else
/// sent will be emitted as printable characters. Tabs are replaced
/// with 4 spaces.
#[derive(Clone, Debug)]
struct AnsiIterator<'l> {
    underlying: std::iter::Enumerate<std::str::Chars<'l>>,
    input: &'l str,
    state: Rc<AnsiState>,
    pending_col: bool,
    inject_spaces: u64,
}


impl AnsiIterator<'_> {
    fn new<'l>(input: &'l str) -> AnsiIterator<'l> {
        AnsiIterator { underlying: input.chars().enumerate(),
                       input: input,
                       state: Rc::new(AnsiState {
                           col: 0,
                           background: 0,
                           foreground: 0,
                           bold: false,
                           underline: false,
                           strike: false
                       }),
                       pending_col: false,
                       inject_spaces: 0
        }
    }
}

impl <'l>Iterator for AnsiIterator<'l> {
    type Item = AnsiEvent<'l>;

    fn next(self: &mut Self) -> Option<AnsiEvent<'l>> {
        if self.pending_col {
            Rc::make_mut(&mut self.state).col += 1;
            self.pending_col = false;
        }
        if self.inject_spaces > 0 {
            self.pending_col = true;
            self.inject_spaces -= 1;
            return Some(AnsiEvent::<'l>(AnsiParseToken::Character(' '), self.state.clone()));
        }
        while let Some((i0, c)) = self.underlying.next() {
            if c == '\n' {
                Rc::make_mut(&mut self.state).col = 0; 
                return Some(AnsiEvent::<'l>(AnsiParseToken::Newline, self.state.clone()));
            } else if c == '\t' {
                for _ in 0..4 {
                    self.pending_col = true;
                    self.inject_spaces = 3;
                    return Some(AnsiEvent::<'l>(AnsiParseToken::Character(' '), self.state.clone()));
                }
            } else if c >= ' ' && c <= '~' {
                self.pending_col = true;
                return Some(AnsiEvent::<'l>(AnsiParseToken::Character(c), self.state.clone()));
            } else if c == '\x1b' {
                if let Some((_, c2)) = self.underlying.next() {
                    if c2 != '[' {
                        continue;
                    }
                }
                if let Some((_, cs1)) = self.underlying.next() {
                    let mut imax = i0;
                    let mut cs_no: i64 = cs1 as i64 - b'0' as i64;
                    if cs_no < 0 || cs_no > 9 {
                        continue;
                    }
                    if let Some((i2, cs2)) = self.underlying.next() {
                        let cs_no2: i64 = cs2 as i64 - b'0' as i64;
                        if cs_no2 >= 0 && cs_no2 <= 9 {
                            if let Some((i3, cs3)) = self.underlying.next() {
                                if cs3 == 'm' {
                                    cs_no *= 10;
                                    cs_no += cs_no2;
                                    imax = i3;
                                } else { continue; }
                            }
                        } else if cs2 != 'm' {
                            continue;
                        } else {
                            imax = i2;
                        }
                        let st = Rc::make_mut(&mut self.state);
                        match cs_no {
                            0 => {
                                st.background = 0;
                                st.foreground = 0;
                                st.bold = false;
                                st.underline = false;
                                st.strike = false;
                            }
                            1 => { st.bold = true; }
                            4 => { st.underline = true; }
                            9 => { st.strike = true; }
                            24 => { st.underline = false; }
                            i if i >= 30 && i <= 37 => {
                                st.foreground = i as u64 - 29;
                            }
                            i if i >= 40 && i <= 47 => {
                                st.foreground = i as u64 - 39;
                            }
                            _ => continue
                        }
                        drop(st);
                        return Some(AnsiEvent::<'l>(
                            AnsiParseToken::ControlSeq(
                                &self.input[i0..(imax + 1)]
                            ), self.state.clone()));
                    }
                }
            }
        }
        None
    }
            
}

/// Strips out basic colours / character formatting codes cleanly. Tabs are
/// changed to spaces, and newlines are preserved. All other ANSI non-printables
/// are stripped but might display incorrectly.
pub fn strip_special_characters(input: &str) -> String {
    let mut buf: String = String::new();
    let it = AnsiIterator::new(input);
    for AnsiEvent(e, _) in it {
        match e {
            AnsiParseToken::Character(c) => buf.push(c),
            AnsiParseToken::Newline => buf.push('\n'),
            _ => {}
        }
    }
    buf
}

/// Allows basic colours / character formatting codes. Tabs are
/// changed to spaces, and newlines are preserved. All other ANSI non-printables
/// are stripped but might display incorrectly.
pub fn limit_special_characters(input: &str) -> String {
    let mut buf: String = String::new();
    let it = AnsiIterator::new(input);
    for AnsiEvent(e, _) in it {
        match e {
            AnsiParseToken::Character(c) => buf.push(c),
            AnsiParseToken::Newline => buf.push('\n'),
            AnsiParseToken::ControlSeq(t) => buf.push_str(t)
        }
    }
    buf
}

/// Flows a second column around a first column, limiting the width of both
/// columns as specified, and adding a gutter.
pub fn flow_around(col1: &str, col1_width: u64, gutter: &str,
                   col2: &str, col2_width: u64) -> String {
    "not yet".to_owned()
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn ignore_special_characters_removes_esc() {
        assert_eq!(ignore_special_characters("hello\x1b[world"), "hello[world");
    }

    #[test]
    fn strip_special_characters_makes_plaintext() {
        assert_eq!(strip_special_characters("a\tb"), "a    b");
        assert_eq!(
            strip_special_characters(ansi!("<red>hello<green>world")),
            "helloworld");
        assert_eq!(
            strip_special_characters("hello\r\x07world\n"),
            "helloworld\n");
        assert_eq!(
            strip_special_characters("hello\r\x07world\n"),
            "helloworld\n");
        assert_eq!(
            strip_special_characters("Test\x1b[5;5fing"),
            "Test5fing");
    }

    #[test]
    fn limit_special_characters_strips_some_things() {
        assert_eq!(limit_special_characters(ansi!("a<bgred><green>b<bggreen><red>c<reset>d")),
                   ansi!("a<bgred><green>b<bggreen><red>c<reset>d"));
        assert_eq!(limit_special_characters("Test\x1b[5;5fing"),
                   "Test5fing");
    }
    
}
