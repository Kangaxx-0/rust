use crate::pp::{BeginToken, BreakToken, Breaks, IndentStyle, Printer, Token, SIZE_INFINITY};
use std::borrow::Cow;

impl Printer {
    /// "raw box"
    pub fn rbox(&mut self, indent: usize, breaks: Breaks) {
        self.scan_begin(BeginToken {
            indent: IndentStyle::Block { offset: indent as isize },
            breaks,
        })
    }

    /// Inconsistent breaking box
    pub fn ibox(&mut self, indent: usize) {
        self.rbox(indent, Breaks::Inconsistent)
    }

    /// Consistent breaking box
    pub fn cbox(&mut self, indent: usize) {
        self.rbox(indent, Breaks::Consistent)
    }

    pub fn visual_align(&mut self) {
        self.scan_begin(BeginToken { indent: IndentStyle::Visual, breaks: Breaks::Consistent });
    }

    pub fn break_offset(&mut self, n: usize, off: isize) {
        self.scan_break(BreakToken { offset: off, blank_space: n as isize })
    }

    pub fn end(&mut self) {
        self.scan_end()
    }

    pub fn eof(mut self) -> String {
        self.scan_eof();
        self.out
    }

    pub fn word<S: Into<Cow<'static, str>>>(&mut self, wrd: S) {
        let string = wrd.into();
        self.scan_string(string)
    }

    fn spaces(&mut self, n: usize) {
        self.break_offset(n, 0)
    }

    pub fn zerobreak(&mut self) {
        self.spaces(0)
    }

    pub fn space(&mut self) {
        self.spaces(1)
    }

    pub fn hardbreak(&mut self) {
        self.spaces(SIZE_INFINITY as usize)
    }

    pub fn is_beginning_of_line(&self) -> bool {
        match self.last_token() {
            Some(last_token) => last_token.is_hardbreak_tok(),
            None => true,
        }
    }

    pub fn hardbreak_tok_offset(off: isize) -> Token {
        Token::Break(BreakToken { offset: off, blank_space: SIZE_INFINITY })
    }
}

impl Token {
    pub fn is_hardbreak_tok(&self) -> bool {
        matches!(self, Token::Break(BreakToken { offset: 0, blank_space: SIZE_INFINITY }))
    }
}
