use std::fmt::{Debug, Display};

use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Position {
    At { line: usize, column: usize },
    EOF,
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut result = serializer.serialize_map(Some(2))?;
        match self {
            Position::At { line, column } => {
                result.serialize_entry("line", line)?;
                result.serialize_entry("column", column)?;
            }
            Position::EOF => {
                result.serialize_entry("line", &-1)?;
                result.serialize_entry("column", &-1)?;
            }
        }
        return result.end();
    }
}

enum PositionKey {
    Line,
    Column,
}

struct PositionKeyVisitor;

impl<'de> Visitor<'de> for PositionKeyVisitor {
    type Value = PositionKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("\"line\" or \"column\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "line" => Ok(PositionKey::Line),
            "column" => Ok(PositionKey::Column),
            _ => Err(serde::de::Error::unknown_field(v, &["line", "column"])),
        }
    }
}

impl<'de> Deserialize<'de> for PositionKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(PositionKeyVisitor)
    }
}

enum PositionValue {
    NotNegative(usize),
    Negative,
}

struct PositionValueVisitor;

impl<'de> Visitor<'de> for PositionValueVisitor {
    type Value = PositionValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("integer")
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(if v < 0 {
            PositionValue::Negative
        } else {
            PositionValue::NotNegative(v as usize)
        })
    }
}

impl<'de> Deserialize<'de> for PositionValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i128(PositionValueVisitor)
    }
}

struct PositionVisitor;

impl<'de> Visitor<'de> for PositionVisitor {
    type Value = Position;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Position")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut line: Option<PositionValue> = None;
        let mut column: Option<PositionValue> = None;

        while let Some((key, value)) = map.next_entry::<PositionKey, PositionValue>()? {
            match key {
                PositionKey::Line => {
                    if line.is_some() {
                        return Err(serde::de::Error::duplicate_field("line"));
                    }
                    line = Some(value);
                }
                PositionKey::Column => {
                    if column.is_some() {
                        return Err(serde::de::Error::duplicate_field("column"));
                    }
                    column = Some(value);
                }
            }
        }

        let Some(line) = line else {
            return Err(serde::de::Error::missing_field("line"));
        };
        let Some(column) = column else {
            return Err(serde::de::Error::missing_field("column"));
        };

        if let PositionValue::NotNegative(line) = line {
            if let PositionValue::NotNegative(column) = column {
                return Ok(Position::At { line, column });
            }
        }
        return Ok(Position::EOF);
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(PositionVisitor)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::At { line, column } => write!(f, "{line}:{column}"),
            Position::EOF => write!(f, "EOF"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, to_string};

    fn serialize(pos: &Position) -> String {
        to_string(&pos).unwrap()
    }

    fn deserialize(json: &str) -> Position {
        from_str::<Position>(json).unwrap()
    }

    #[test]
    fn test_serialize() {
        assert_eq!(
            serialize(&Position::At { line: 2, column: 1 }),
            r#"{"line":2,"column":1}"#
        );
        assert_eq!(serialize(&Position::EOF), r#"{"line":-1,"column":-1}"#);
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            deserialize(r#"{"line":2,"column":1}"#),
            Position::At { line: 2, column: 1 }
        );
        assert_eq!(deserialize(r#"{"line":-1,"column":-1}"#), Position::EOF);
        assert!(from_str::<Position>(r#"[]"#).is_err());
        assert!(from_str::<Position>(r#"{"a":1}"#).is_err());
        assert!(from_str::<Position>(r#"{"line":"abc","column":"def"}"#).is_err());
        assert!(from_str::<Position>(r#"{"line":1,"line":1,"column":1}"#).is_err());
        assert!(from_str::<Position>(r#"{"line":1,"column":1,"column":1}"#).is_err());
        assert!(from_str::<Position>(r#"{"line":1}"#).is_err());
        assert!(from_str::<Position>(r#"{"column":1}"#).is_err());
    }

    #[test]
    fn display() {
        assert_eq!(Position::At { line: 2, column: 1 }.to_string(), "2:1");
        assert_eq!(Position::EOF.to_string(), "EOF");
    }
}
