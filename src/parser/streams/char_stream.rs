use crate::{common::Location, string::Utf16Str};

/// 入力文字列から文字を読み取る  
/// オリジナルのAiScriptと異なり1ページのみ
pub struct CharStream<'a> {
    page: &'a Utf16Str,
    address: usize,
    char: Option<u16>,

    /// zero-based number
    line: usize,

    /// zero-based number
    column: usize,
}

impl CharStream<'_> {
    pub fn new<'a>(source: &'a Utf16Str, opts: CharStreamOpts) -> CharStream<'a> {
        let mut result = CharStream {
            page: source,
            address: 0,
            char: None,
            line: opts.line,
            column: opts.column,
        };
        result.move_next();
        return result;
    }

    /// ストリームの終わりに達しているかどうかを取得します。
    pub fn eof(&self) -> bool {
        return self.end_of_page();
    }

    /// カーソル位置にある文字を取得します。
    pub fn char(&self) -> Option<u16> {
        self.char
    }

    /// カーソル位置に対応するソースコード上の行番号と列番号を取得します。
    pub fn get_pos(&self) -> Location {
        return Location::At {
            line: self.line + 1,
            column: self.column + 1,
        };
    }

    /// カーソル位置を<'次の文字へ進めます。
    pub fn next(&mut self) {
        if !self.eof() && self.char.is_some_and(|char| char == '\n' as u16) {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        self.inc_addr();
        self.move_next();
    }

    pub fn prev(&mut self) {
        self.dec_addr();
        self.move_prev();
    }

    fn end_of_page(&self) -> bool {
        return self.address >= self.page.len();
    }

    fn move_next(&mut self) {
        self.load_char();
        loop {
            if self.char.is_some_and(|char| char == '\r' as u16) {
                self.inc_addr();
                self.load_char();
                continue;
            }
            break;
        }
    }

    fn inc_addr(&mut self) {
        if !self.end_of_page() {
            self.address += 1;
        }
    }

    fn move_prev(&mut self) {
        self.load_char();
        loop {
            if self.char.is_some_and(|char| char == '\r' as u16) {
                self.dec_addr();
                self.load_char();
                continue;
            }
            break;
        }
    }

    fn dec_addr(&mut self) {
        if self.address > 0 {
            self.address -= 1;
        }
    }

    fn load_char(&mut self) {
        if self.eof() {
            self.char = None;
        } else {
            self.char = self
                .page
                .as_u16s()
                .get(self.address)
                .map(|char| char.clone());
        }
    }
}

pub struct CharStreamOpts {
    line: usize,
    column: usize,
}

impl Default for CharStreamOpts {
    fn default() -> Self {
        CharStreamOpts { line: 0, column: 0 }
    }
}

impl<'a> From<&'a Utf16Str> for CharStream<'a> {
    fn from(value: &'a Utf16Str) -> Self {
        CharStream::new(value, CharStreamOpts::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::string::Utf16String;

    use super::*;

    #[test]
    fn char() {
        let source = Utf16String::from("abc");
        let stream = CharStream::new(&source, Default::default());
        assert_eq!(Some('a' as u16), stream.char());
    }

    #[test]
    fn pos() {
        let source = Utf16String::from("ab\nc");
        let mut stream = CharStream::new(&source, Default::default());
        assert_eq!(stream.get_pos(), Location::At { line: 1, column: 1 });
        stream.next();
        assert_eq!(stream.get_pos(), Location::At { line: 1, column: 2 });
        stream.next();
        assert_eq!(stream.get_pos(), Location::At { line: 1, column: 3 });
        stream.next();
        assert_eq!(stream.get_pos(), Location::At { line: 2, column: 1 });
        stream.next();
        assert_eq!(stream.get_pos(), Location::At { line: 2, column: 2 });
    }

    #[test]
    fn next() {
        let source = Utf16String::from("abc");
        let mut stream = CharStream::new(&source, Default::default());
        stream.next();
        assert_eq!(Some('b' as u16), stream.char());
    }

    #[cfg(test)]
    mod prev {
        use super::*;

        #[test]
        fn test_move() {
            let source = Utf16String::from("abc");
            let mut stream = CharStream::new(&source, Default::default());
            stream.next();
            assert_eq!(Some('b' as u16), stream.char());
            stream.prev();
            assert_eq!(Some('a' as u16), stream.char());
        }

        #[test]
        fn no_move_out_of_bound() {
            let source = Utf16String::from("abc");
            let mut stream = CharStream::new(&source, Default::default());
            stream.prev();
            assert_eq!(Some('a' as u16), stream.char());
        }
    }

    #[test]
    fn eof() {
        let source = Utf16String::from("abc");
        let mut stream = CharStream::new(&source, Default::default());
        assert_eq!(false, stream.eof());
        stream.next();
        assert_eq!(false, stream.eof());
        stream.next();
        assert_eq!(false, stream.eof());
        stream.next();
        assert_eq!(true, stream.eof());
    }

    #[test]
    fn none_when_ref_char_at_eof() {
        let source = Utf16String::new();
        let stream = CharStream::new(&source, Default::default());
        assert_eq!(true, stream.eof());
        assert!(stream.char().is_none());
    }

    #[test]
    fn cr_skipped() {
        let source = Utf16String::from("a\r\nb");
        let mut stream = CharStream::new(&source, Default::default());
        assert_eq!(Some('a' as u16), stream.char());
        stream.next();
        assert_eq!(Some('\n' as u16), stream.char());
        stream.next();
        assert_eq!(Some('b' as u16), stream.char());
        stream.next();
        assert_eq!(true, stream.eof());
    }
}
