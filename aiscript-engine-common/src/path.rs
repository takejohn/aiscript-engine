use std::{borrow::Borrow, fmt::Display};

use serde::{Deserialize, Serialize};
use utf16_literal::utf16;

use crate::{Utf16Str, Utf16String};

const SEPARATOR_CHAR: u16 = utf16!(':');
const SEPARATOR: &Utf16Str = Utf16Str::new(&utf16!(":"));

/// 変数名のパス
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NamePath {
    inner: Utf16String,
}

impl NamePath {
    pub fn new() -> Self {
        NamePath {
            inner: Utf16String::new(),
        }
    }

    pub fn from_segments<S>(segments: &[S]) -> Self
    where
        S: Borrow<Utf16Str>,
    {
        NamePath {
            inner: Utf16String::join(segments, SEPARATOR),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn segment_count(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.as_utf16_str()
                .into_iter()
                .filter(|&ch| ch == SEPARATOR_CHAR)
                .count()
                + 1
        }
    }

    pub fn append(&mut self, name: impl Borrow<Utf16Str>) {
        if !self.is_empty() {
            self.inner += SEPARATOR;
        }
        self.inner += name.borrow();
    }

    pub fn append_path(&mut self, path: &NamePath) {
        if !self.is_empty() {
            self.inner += SEPARATOR;
        }
        self.inner += path.as_utf16_str();
    }

    pub fn as_utf16_str(&self) -> &Utf16Str {
        return &self.inner;
    }
}

impl<S> From<S> for NamePath
where
    S: Borrow<Utf16Str>,
{
    fn from(value: S) -> Self {
        NamePath {
            inner: value.borrow().to_owned(),
        }
    }
}

impl Display for NamePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_utf16_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let path = NamePath::from(Utf16Str::new(&utf16!("Ns:id")));
        let serialized = serde_json::to_string(&path).unwrap();
        assert_eq!(serialized, r#""Ns:id""#);
    }

    #[test]
    fn deserialize() {
        let deserialized = serde_json::from_str::<NamePath>(r#""Ns:id""#).unwrap();
        let expected = NamePath::from(Utf16Str::new(&utf16!("Ns:id")));
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn from_segments() {
        let path =
            NamePath::from_segments(&[Utf16Str::new(&utf16!("Ns")), Utf16Str::new(&utf16!("id"))]);
        let expected = NamePath::from(Utf16Str::new(&utf16!("Ns:id")));
        assert_eq!(path, expected);
    }

    #[test]
    fn edit() {
        let mut path = NamePath::from(Utf16Str::new(&utf16!("Ns")));
        path.append(Utf16Str::new(&utf16!("id")));
        let expected = NamePath::from(Utf16Str::new(&utf16!("Ns:id")));
        assert_eq!(path, expected);
    }

    #[test]
    fn display() {
        let path = NamePath::from(Utf16Str::new(&utf16!("Ns:id")));
        assert_eq!(path.to_string(), "Ns:id");
    }
}
