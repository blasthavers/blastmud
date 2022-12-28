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

impl AnsiState {
    fn restore_ansi(self: &Self) -> String {
        let mut buf = String::new();
        if !(self.bold && self.underline && self.strike &&
              self.background != 0 && self.foreground != 0) {
            buf.push_str(ansi!("<reset>"));
        }
        if self.bold { buf.push_str(ansi!("<bold>")); }
        if self.underline { buf.push_str(ansi!("<under>")); }
        if self.strike { buf.push_str(ansi!("<strike>")); }
        if self.background != 0 {
            buf.push_str(&format!("\x1b[{}m", 39 + self.background)); }
        if self.foreground != 0 {
            buf.push_str(&format!("\x1b[{}m", 29 + self.foreground)); }
        buf
    }
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
pub fn flow_around(col1: &str, col1_width: usize, gutter: &str,
                   col2: &str, col2_width: usize) -> String {
    let mut it1 = AnsiIterator::new(col1).peekable();
    let mut it2 = AnsiIterator::new(col2).peekable();

    let mut buf = String::new();

    // Phase 1: col1 still has data, so flow col2 around col1.
    'around_rows: loop {
        match it1.peek() {
            None => break 'around_rows,
            Some(AnsiEvent(_, st)) => buf.push_str(&st.restore_ansi())
        }
        let mut fill_needed: usize = 0;
        let mut skip_nl = true;
        'col_data: for i in 0..col1_width {
            'until_move_forward: loop {
                match it1.next() {
                    None | Some(AnsiEvent(AnsiParseToken::Newline, _)) => {
                        fill_needed = col1_width - i;
                        skip_nl = false;
                        break 'col_data;
                    }
                    Some(AnsiEvent(AnsiParseToken::Character(c), _)) => {
                        buf.push(c);
                        break 'until_move_forward;
                    }
                    Some(AnsiEvent(AnsiParseToken::ControlSeq(s), _)) => {
                        buf.push_str(s);
                    }
                }
            }
        }
        // If there is a newline (optionally preceded by 1+ control characters),
        // and we didn't just read one, we should skip it, since we broke to a
        // new line anyway. It is safe to eat any control characters since we will
        // restore_ansi() anyway.
        if skip_nl {
            loop {
                match it1.peek() {
                    None => break,
                    Some(AnsiEvent(AnsiParseToken::Character(_), _)) => break,
                    Some(AnsiEvent(AnsiParseToken::ControlSeq(s), _)) => {
                        if fill_needed > 0 { buf.push_str(s); }
                        it1.next();
                    }
                    Some(AnsiEvent(AnsiParseToken::Newline, _)) => {
                        it1.next();
                        break;
                    }
                }
            }
        }
        for _ in 0..fill_needed { buf.push(' '); }

        buf.push_str(gutter);

        if let Some(AnsiEvent(_, st)) = it2.peek() {
            buf.push_str(&st.restore_ansi())
        }
        skip_nl = true;
        'col_data: for _ in 0..col2_width {
            'until_move_forward: loop {
                match it2.next() {
                    None | Some(AnsiEvent(AnsiParseToken::Newline, _)) => {
                        skip_nl = false;
                        break 'col_data;
                    }
                    Some(AnsiEvent(AnsiParseToken::Character(c), _)) => {
                        buf.push(c);
                        break 'until_move_forward;
                    }
                    Some(AnsiEvent(AnsiParseToken::ControlSeq(s), _)) => {
                        buf.push_str(s);
                    }
                }
            }
        }
        if skip_nl {
            loop {
                match it2.peek() {
                    None => break,
                    Some(AnsiEvent(AnsiParseToken::Character(_), _)) => break,
                    Some(AnsiEvent(AnsiParseToken::ControlSeq(s), _)) => {
                        if fill_needed > 0 { buf.push_str(s); }
                        it2.next();
                    }
                    Some(AnsiEvent(AnsiParseToken::Newline, _)) => {
                        it2.next();
                        break;
                    }
                }
            }
        }
        buf.push('\n');
    }

    // Now just copy anything left in it2 over.
    for AnsiEvent(e, _) in it2 {
        match e {
            AnsiParseToken::Character(c) => buf.push(c),
            AnsiParseToken::Newline => buf.push('\n'),
            AnsiParseToken::ControlSeq(t) => buf.push_str(t)
        }
    }
    
    buf
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

    #[test]
    fn flow_around_works_for_plain_text() {
        let str1 = "  /\\  /\\\n\
                    /--------\\\n\
                    | ()  () |\n\
                    |        |\n\
                    |   /\\   |\n\
                    | \\    / |\n\
                    | -(--)- |\n\
                    | /    \\ |\n\
                    \\--------/\n\
                    A very poor rendition of a cat! Meow.";
        let str2 = "Hello world, this is the second column for this test. It starts with a rather long line that will wrap.\n\
                    And here is a shorter line.\n\
                    All of this should by nicely wrapped, even if it is exactly the len\n\
                    gth of column 2!\n\
                    \n\
                    But double newlines should come up as blank lines.\n\
                    Blah\n\
                    Blah\n\
                    Blah\n\
                    Blah\n\
                    Blah\n\
                    Blah\n\
                    Blah\n\
                    And once we get to the bottom of column 1, column 2 should just get written\n\
                    out normally, not in the previous column.";
        // This has a lot of unnecessary resets, but that is expected with the algorithm right now.
        let expected = "\u{1b}[0m  /\\  /\\   | \u{1b}[0mHello world, this is the second column for this test. It starts wit\n\u{1b}[0m/--------\\ | \u{1b}[0mh a rather long line that will wrap.\n\u{1b}[0m| ()  () | | \u{1b}[0mAnd here is a shorter line.\n\u{1b}[0m|        | | \u{1b}[0mAll of this should by nicely wrapped, even if it is exactly the len\n\u{1b}[0m|   /\\   | | \u{1b}[0mgth of column 2!\n\u{1b}[0m| \\    / | | \u{1b}[0m\n\u{1b}[0m| -(--)- | | \u{1b}[0mBut double newlines should come up as blank lines.\n\u{1b}[0m| /    \\ | | \u{1b}[0mBlah\n\u{1b}[0m\\--------/ | \u{1b}[0mBlah\n\u{1b}[0mA very poo | \u{1b}[0mBlah\n\u{1b}[0mr renditio | \u{1b}[0mBlah\n\u{1b}[0mn of a cat | \u{1b}[0mBlah\n\u{1b}[0m! Meow.    | \u{1b}[0mBlah\nBlah\nAnd once we get to the bottom of column 1, column 2 should just get written\nout normally, not in the previous column.";
        assert_eq!(flow_around(str1, 10, " | ", str2, 67), expected);
    }
    
}
