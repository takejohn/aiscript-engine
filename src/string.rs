use core::slice;
use std::{
    char::decode_utf16,
    fmt::{Debug, Display},
    iter,
    ops::{self, AddAssign},
    vec,
};

#[macro_export(local_inner_macros)]
macro_rules! macro_args_len {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + macro_args_len!($($xs)*));
}

/// UTF-16の文字コードから静的領域に文字列を生成するマクロ
#[macro_export(local_inner_macros)]
macro_rules! utf16_str {
    ( $($c:expr),* ) => {{
        const DATA: [u16; macro_args_len!($(($c))*)]  = [$($c as u16),*];
        const STR: &$crate::string::Utf16Str = $crate::string::Utf16Str::new(&DATA);
        STR
    }};
}

/// 参照として使用できるUTF-16文字列。
/// サロゲートペアが完全である必要はない。
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Utf16Str {
    data: [u16],
}

impl Utf16Str {
    pub const fn new(data: &[u16]) -> &Self {
        unsafe { &*(data as *const [u16] as *const Utf16Str) }
    }

    pub fn new_mut(data: &mut [u16]) -> &mut Self {
        unsafe { &mut *(data as *mut [u16] as *mut Utf16Str) }
    }

    pub fn as_u16s(&self) -> &[u16] {
        &self.data
    }

    pub fn as_mut_u16s(&mut self) -> &mut [u16] {
        &mut self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Display for Utf16Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for char in decode_utf16(self.data.iter().map(|r| r.clone())) {
            match char {
                Ok(char) => write!(f, "{}", char)?,
                Err(_) => write!(f, "\u{FFFD}")?, // U+FFFD: REPLACEMENT CHARACTER
            }
        }
        Ok(())
    }
}

impl Debug for Utf16Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl<'a> IntoIterator for &'a Utf16Str {
    type Item = u16;

    type IntoIter = iter::Cloned<slice::Iter<'a, u16>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter().cloned()
    }
}

/// 可変なUTF-16文字列。
/// サロゲートペアが完全である必要はない。
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Utf16String {
    data: Vec<u16>,
}

impl Utf16String {
    pub fn new() -> Self {
        Utf16String { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn as_utf16_str(&self) -> &Utf16Str {
        &Utf16Str::new(&self.data)
    }

    pub fn as_mut_utf16_str(&mut self) -> &mut Utf16Str {
        Utf16Str::new_mut(&mut self.data)
    }

    pub fn is_empty(&mut self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, ch: u16) {
        self.data.push(ch);
    }
}

impl FromIterator<char> for Utf16String {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut data = Vec::new();
        for c in iter {
            let cp: u32 = c as u32;
            if cp <= 0xFFFF {
                data.push(cp as u16);
            } else {
                data.push(((cp - 0x10000) / 0x400 + 0xD800) as u16);
                data.push(((cp - 0x10000) % 0x400 + 0xDC00) as u16);
            }
        }
        return Utf16String { data };
    }
}

impl ops::Deref for Utf16String {
    type Target = Utf16Str;

    fn deref(&self) -> &Self::Target {
        self.as_utf16_str()
    }
}

impl ops::DerefMut for Utf16String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_utf16_str()
    }
}

impl From<&Utf16Str> for Utf16String {
    fn from(value: &Utf16Str) -> Self {
        Utf16String {
            data: Vec::from(value.as_u16s()),
        }
    }
}

impl From<&str> for Utf16String {
    fn from(value: &str) -> Self {
        Utf16String {
            data: value.encode_utf16().collect(),
        }
    }
}

impl From<u16> for Utf16String {
    fn from(value: u16) -> Self {
        Utf16String { data: vec![value] }
    }
}

impl From<char> for Utf16String {
    fn from(value: char) -> Self {
        Utf16String::from_iter(iter::once(value))
    }
}

impl Display for Utf16String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.as_utf16_str(), f)
    }
}

impl Debug for Utf16String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.as_utf16_str(), f)
    }
}

impl IntoIterator for Utf16String {
    type Item = u16;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Extend<u16> for Utf16String {
    fn extend<T: IntoIterator<Item = u16>>(&mut self, iter: T) {
        self.data.extend(iter);
    }
}

impl ops::AddAssign<&Utf16Str> for Utf16String {
    fn add_assign(&mut self, rhs: &Utf16Str) {
        self.extend(rhs);
    }
}

impl ops::Add<&Utf16Str> for Utf16String {
    type Output = Self;

    /// 末尾に別の文字列を結合する破壊的メソッド
    fn add(mut self, rhs: &Utf16Str) -> Self::Output {
        self.add_assign(rhs);
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_macro() {
        let s = utf16_str!('a', 'b', 'c');
        assert_eq!(Utf16String::from("abc").as_utf16_str(), s);
    }

    mod utf16_string {
        use super::*;

        #[test]
        fn to_string() {
            let original = "abc";
            assert_eq!(Utf16String::from("abc").to_string(), original);
        }

        #[test]
        fn add_assign() {
            let mut buf = Utf16String::from("abc");
            buf += &Utf16String::from("123");
            assert_eq!(buf.to_string(), "abc123")
        }
    }
}
