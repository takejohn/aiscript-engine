use core::slice;
use std::{
    char::decode_utf16,
    fmt::{Debug, Display},
    iter,
    num::ParseFloatError,
    ops::{self, AddAssign},
    vec,
};

/// 参照として使用できるUTF-16文字列。
/// サロゲートペアが完全である必要はない。
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Utf16Str {
    data: [u16],
}

impl Utf16Str {
    #[inline]
    pub const fn new(data: &[u16]) -> &Self {
        unsafe { &*(data as *const [u16] as *const Utf16Str) }
    }

    #[inline]
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

    pub fn parse<F: FromUtf16Str>(&self) -> Result<F, F::Err> {
        F::from(self)
    }

    /// selfを区切り文字として引数に与えらえれた文字列を結合する。
    pub fn join<'a>(&self, strings: impl Iterator<Item = impl Into<&'a Utf16Str>>) -> Utf16String {
        let mut iter = strings;
        let mut result = Utf16String::new();
        if let Some(first) = iter.next() {
            result += first.into();
        } else {
            return result;
        }
        while let Some(item) = iter.next() {
            result += self;
            result += item.into();
        }
        return result;
    }
}

impl<'a> From<&'a Utf16String> for &'a Utf16Str {
    fn from(value: &'a Utf16String) -> Self {
        value.as_utf16_str()
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

impl<'a> FromIterator<&'a u16> for Utf16String {
    fn from_iter<T: IntoIterator<Item = &'a u16>>(iter: T) -> Self {
        Utf16String {
            data: iter.into_iter().map(|&value| value).collect(),
        }
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

impl ops::AddAssign<u16> for Utf16String {
    fn add_assign(&mut self, rhs: u16) {
        self.push(rhs);
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

impl ops::Add<u16> for Utf16String {
    type Output = Self;

    /// 末尾に文字を結合する破壊的メソッド
    fn add(mut self, rhs: u16) -> Self::Output {
        self.add_assign(rhs);
        return self;
    }
}

pub trait FromUtf16Str: Sized {
    type Err;

    fn from(s: &Utf16Str) -> Result<Self, Self::Err>;
}

impl FromUtf16Str for f64 {
    type Err = ParseFloatError;

    fn from(s: &Utf16Str) -> Result<Self, Self::Err> {
        // TODO: ECMAScriptと同じアルゴリズムを用いる
        s.to_string().parse()
    }
}

#[cfg(test)]
mod tests {
    use utf16_literal::utf16;

    use super::*;

    #[test]
    fn utf16_macro() {
        let s = Utf16Str::new(&utf16!("abc"));
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
            buf += Utf16Str::new(&utf16!("123"));
            assert_eq!(buf.to_string(), "abc123")
        }
    }
}
