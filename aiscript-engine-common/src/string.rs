use core::slice;
use std::{
    borrow::{Borrow, BorrowMut},
    char::decode_utf16,
    fmt::{Debug, Display},
    iter,
    num::ParseFloatError,
    ops::{self, AddAssign, Index, IndexMut},
    vec,
};

use serde::{de::Visitor, Deserialize, Serialize};

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

    #[inline]
    pub fn as_u16s(&self) -> &[u16] {
        &self.data
    }

    #[inline]
    pub fn as_mut_u16s(&mut self) -> &mut [u16] {
        &mut self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn parse<F: FromUtf16Str>(&self) -> Result<F, F::Err> {
        F::from(self)
    }
}

impl ToOwned for Utf16Str {
    type Owned = Utf16String;

    fn to_owned(&self) -> Self::Owned {
        Utf16String::from_iter(self.as_u16s())
    }
}

impl Index<usize> for Utf16Str {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Utf16Str {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
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

impl Serialize for Utf16Str {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
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

    pub fn with_capacity(capacity: usize) -> Self {
        Utf16String {
            data: Vec::with_capacity(capacity),
        }
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

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, ch: u16) {
        self.data.push(ch);
    }

    /// sepを区切り文字として引数に与えらえれた文字列を結合する。
    pub fn join<S>(strings: &[S], sep: &Utf16Str) -> Utf16String
    where
        S: Borrow<Utf16Str>,
    {
        let mut iter = strings.iter();
        let mut result = Utf16String::new();
        if let Some(first) = iter.next() {
            result += first.borrow();
        } else {
            return result;
        }
        while let Some(item) = iter.next() {
            result += sep;
            result += item.borrow();
        }
        return result;
    }
}

impl Borrow<Utf16Str> for Utf16String {
    fn borrow(&self) -> &Utf16Str {
        self.as_utf16_str()
    }
}

impl Borrow<Utf16Str> for &Utf16String {
    fn borrow(&self) -> &Utf16Str {
        self.as_utf16_str()
    }
}

impl BorrowMut<Utf16Str> for Utf16String {
    fn borrow_mut(&mut self) -> &mut Utf16Str {
        self.as_mut_utf16_str()
    }
}

impl Index<usize> for Utf16String {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Utf16String {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
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

impl Serialize for Utf16String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct Utf16StringVisitor;

impl<'de> Visitor<'de> for Utf16StringVisitor {
    type Value = Utf16String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Utf16String")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Utf16String::from(v))
    }
}

impl<'de> Deserialize<'de> for Utf16String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Utf16StringVisitor)
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

    mod utf16 {
        use super::*;

        #[test]
        fn convert() {
            let data = utf16!("abc");
            let s = Utf16Str::new(&data);
            assert_eq!(s.as_u16s(), &data);

            let mut mut_data = data.to_owned();
            let s_mut = Utf16Str::new_mut(&mut mut_data);
            assert_eq!(s_mut.as_mut_u16s(), &data);
        }

        #[test]
        fn len() {
            let s = Utf16Str::new(&utf16!("abc"));
            assert_eq!(s.len(), 3);

            let s = Utf16Str::new(&utf16!("🥰"));
            assert_eq!(s.len(), 2);
        }

        #[test]
        fn parse() {
            let s = Utf16Str::new(&utf16!("9.75"));
            assert_eq!(s.parse::<f64>().unwrap(), 9.75);
        }

        #[test]
        fn join() {
            let sep = Utf16Str::new(&utf16!(" "));

            assert_eq!(
                Utf16String::join(&[] as &[&Utf16Str], sep),
                Utf16String::new()
            );

            let strings = [Utf16Str::new(&utf16!("abc")), Utf16Str::new(&utf16!("123"))];
            assert_eq!(
                Utf16String::join(&strings, sep),
                Utf16String::from_iter(&utf16!("abc 123"))
            );
        }

        #[test]
        fn to_owned() {
            let s = Utf16Str::new(&utf16!("abc"));
            assert_eq!(s.to_owned(), Utf16String::from_iter(&utf16!("abc")));
        }

        #[test]
        fn index() {
            let s = Utf16Str::new(&utf16!("abc"));
            assert_eq!(s[0], utf16!('a'));

            let mut data = utf16!("abc");
            let s = Utf16Str::new_mut(&mut data);
            s[0] = utf16!('1');
            assert_eq!(s[0], utf16!('1'));
        }

        #[test]
        fn display() {
            let s = Utf16Str::new(&utf16!("abc"));
            assert_eq!(s.to_string(), "abc");

            let data = utf16!("🥰");
            let s0 = Utf16Str::new(slice::from_ref(&data[0]));
            let s1 = Utf16Str::new(slice::from_ref(&data[1]));
            assert_eq!(s0.to_string(), "\u{FFFD}");
            assert_eq!(s1.to_string(), "\u{FFFD}");
        }

        #[test]
        fn debug() {
            let s = Utf16Str::new(&utf16!("abc"));
            assert_eq!(format!("{:?}", s), "abc");

            let data = utf16!("🥰");
            let s0 = Utf16Str::new(slice::from_ref(&data[0]));
            let s1 = Utf16Str::new(slice::from_ref(&data[1]));
            assert_eq!(format!("{:?}", s0), "\u{FFFD}");
            assert_eq!(format!("{:?}", s1), "\u{FFFD}");
        }

        #[test]
        fn serialize() {
            let s = Utf16Str::new(&utf16!("abc"));
            assert_eq!(serde_json::to_string(s).unwrap(), r#""abc""#);
        }
    }

    mod utf16_string {
        use ops::{Deref, DerefMut};

        use super::*;

        #[test]
        fn len() {
            let s = Utf16String::from_iter(&utf16!("abc"));
            assert_eq!(s.len(), 3);

            let s = Utf16String::from_iter(&utf16!("🥰"));
            assert_eq!(s.len(), 2);
        }

        #[test]
        fn borrow() {
            let mut s = Utf16String::from_iter(&utf16!("abc"));
            let s_mut: &mut Utf16Str = s.borrow_mut();
            s_mut[2] = utf16!('1');
            let s_immutable: &Utf16Str = s.borrow();
            assert_eq!(s_immutable, Utf16Str::new(&utf16!("ab1")));
        }

        #[test]
        fn index() {
            let s = Utf16String::from_iter(&utf16!("abc"));
            assert_eq!(s[0], utf16!('a'));

            let mut s = Utf16String::from_iter(&utf16!("abc"));
            s[0] = utf16!('1');
            assert_eq!(s[0], utf16!('1'));
        }

        #[test]
        fn deref() {
            let s = Utf16String::from_iter(&utf16!("abc"));
            assert_eq!(s.deref(), Utf16Str::new(&utf16!("abc")));

            let mut s = Utf16String::from_iter(&utf16!("abc"));
            s.deref_mut()[0] = utf16!('1');
            assert_eq!(s.deref(), Utf16Str::new(&utf16!("1bc")));
        }

        #[test]
        fn display() {
            let original = "abc";
            assert_eq!(Utf16String::from("abc").to_string(), original);
        }

        #[test]
        fn debug() {
            let original = "abc";
            assert_eq!(format!("{:?}", Utf16String::from("abc")), original);
        }

        #[test]
        fn into_iter() {
            let mut iter = Utf16String::from_iter(&utf16!("abc")).into_iter();
            assert_eq!(iter.next(), Some(utf16!('a')));
            assert_eq!(iter.next(), Some(utf16!('b')));
            assert_eq!(iter.next(), Some(utf16!('c')));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn add_assign() {
            let mut buf = Utf16String::from_iter(&utf16!("abc"));
            buf += Utf16Str::new(&utf16!("123"));
            assert_eq!(buf.as_utf16_str(), Utf16Str::new(&utf16!("abc123")));
            buf += utf16!('4');
            assert_eq!(buf.as_utf16_str(), Utf16Str::new(&utf16!("abc1234")));
        }

        #[test]
        fn add() {
            let buf = Utf16String::from_iter(&utf16!("abc"));
            assert_eq!(
                (buf + Utf16Str::new(&utf16!("123"))).as_utf16_str(),
                Utf16Str::new(&utf16!("abc123"))
            );

            let buf = Utf16String::from_iter(&utf16!("abc"));
            assert_eq!(
                (buf + utf16!('d')).as_utf16_str(),
                Utf16Str::new(&utf16!("abcd"))
            );
        }

        #[test]
        fn serialize() {
            let s = Utf16String::from_iter(&utf16!("abc"));
            assert_eq!(serde_json::to_string(&s).unwrap(), r#""abc""#);
        }

        #[test]
        fn deserialize() {
            let s = serde_json::from_str::<Utf16String>(r#""abc""#).unwrap();
            assert_eq!(s, Utf16String::from_iter(&utf16!("abc")));
        }
    }
}
