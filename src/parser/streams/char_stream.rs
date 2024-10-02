use crate::ast::Pos;

/// 入力文字列から文字を読み取る  
/// オリジナルのAiScriptと異なり1ページのみ
pub struct CharStream {
    page: Vec<u16>,
    address: usize,
    char: Option<u16>,

    /// zero-based number
    line: usize,

    /// zero-based number
    column: usize,
}

impl CharStream {
    pub fn new(source: impl Into<Vec<u16>>, opts: CharStreamOpts) -> Self {
        let mut result = CharStream {
            page: source.into(),
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
    pub fn char(&self) -> Result<u16, &'static str> {
        match self.char {
            Some(char) => Ok(char),
            None => Err("end of stream"),
        }
    }

    /// カーソル位置に対応するソースコード上の行番号と列番号を取得します。
    pub fn get_pos(&self) -> Pos {
        return Pos {
            line: self.line + 1,
            column: self.column + 1,
        };
    }

    /// カーソル位置を次の文字へ進めます。
    pub fn next(&mut self) {
        if !self.eof() && self.char.is_some_and(|char| char == '\n' as u16) {
            self.line += 1;
            self.column = 0;
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
            if !self.eof() && self.char.is_some_and(|char| char == '\r' as u16) {
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
            if !self.eof() && self.char.is_some_and(|char| char == '\r' as u16) {
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
            self.char = self.page.get(self.address).map(|char| char.clone());
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

#[cfg(test)]
mod tests {
    use super::*;

    trait AsUtf16<T> {
        fn as_utf16(&self) -> T;
    }

    impl AsUtf16<Vec<u16>> for str {
        fn as_utf16(&self) -> Vec<u16> {
            self.encode_utf16().collect()
        }
    }

    #[test]
    fn char() {
        let source = "abc".as_utf16();
        let stream = CharStream::new(source, Default::default());
        assert_eq!(Ok('a' as u16), stream.char());
    }

    #[test]
    fn next() {
        let source = "abc".as_utf16();
        let mut stream = CharStream::new(source, Default::default());
        stream.next();
        assert_eq!(Ok('b' as u16), stream.char());
    }

    #[cfg(test)]
    mod prev {
        use super::*;

        #[test]
        fn test_move() {
            let source = "abc".as_utf16();
            let mut stream = CharStream::new(source, Default::default());
            stream.next();
            assert_eq!(Ok('b' as u16), stream.char());
            stream.prev();
            assert_eq!(Ok('a' as u16), stream.char());
        }

        #[test]
        fn no_move_out_of_bound() {
            let source = "abc".as_utf16();
            let mut stream = CharStream::new(source, Default::default());
            stream.prev();
            assert_eq!(Ok('a' as u16), stream.char());
        }
    }

    #[test]
    fn eof() {
        let source = "abc".as_utf16();
        let mut stream = CharStream::new(source, Default::default());
        assert_eq!(false, stream.eof());
        stream.next();
        assert_eq!(false, stream.eof());
        stream.next();
        assert_eq!(false, stream.eof());
        stream.next();
        assert_eq!(true, stream.eof());
    }

    #[test]
    fn err_when_ref_char_at_eof() {
        let source = "".as_utf16();
        let stream = CharStream::new(source, Default::default());
        assert_eq!(true, stream.eof());
        assert!(stream.char().is_err());
    }

    #[test]
    fn cr_skipped() {
        let source = "a\r\nb".as_utf16();
        let mut stream = CharStream::new(source, Default::default());
        assert_eq!(Ok('a' as u16), stream.char());
        stream.next();
        assert_eq!(Ok('\n' as u16), stream.char());
        stream.next();
        assert_eq!(Ok('b' as u16), stream.char());
        stream.next();
        assert_eq!(true, stream.eof());
    }
}
