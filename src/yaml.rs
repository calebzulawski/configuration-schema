use crate::data::{Position, Spanned, SpannedValue, Value};
use crate::error::{ContextualError, Error};
use crate::validator::Validator;
use linked_hash_map::LinkedHashMap;
use std::path::Path;
use yaml_rust::parser::{Event, Parser};
use yaml_rust::scanner::{ScanError, TScalarStyle, TokenType};
use yaml_rust::Yaml;

pub fn load_from_file<P: AsRef<Path>, V: Validator>(
    path: P,
    validator: &V,
) -> Result<SpannedValue, Error> {
    let s = std::fs::read_to_string(path)?;
    let v = load_stream(s.chars())?;
    validator.validate(&v)?;
    Ok(v)
}

impl From<ScanError> for ContextualError {
    fn from(err: ScanError) -> Self {
        ContextualError::new(&format!("{}", err))
    }
}

fn load_stream<T: Iterator<Item = char>>(src: T) -> Result<SpannedValue, ContextualError> {
    let mut parser = Parser::new(src);
    assert_eq!(parser.next()?.0, Event::StreamStart);
    assert_eq!(parser.next()?.0, Event::DocumentStart);
    let value = load_node(&mut parser)?;
    assert_eq!(parser.next()?.0, Event::DocumentEnd);
    let (event, marker) = parser.next()?;
    if event != Event::StreamEnd {
        Err(ContextualError::new("expected end of file"))
    } else {
        Ok(value)
    }
}

fn load_node<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
) -> Result<SpannedValue, ContextualError> {
    let (event, marker) = parser.next()?;
    let start = Position {
        line: marker.line(),
        column: marker.col(),
    };
    let parsed = match event {
        Event::Scalar(value, style, anchor, tag) => {
            if style != TScalarStyle::Plain {
                Ok(Value::String(value))
            } else if let Some(TokenType::Tag(ref handle, ref suffix)) = tag {
                if handle == "!!" {
                    match suffix.as_ref() {
                        "bool" => value
                            .parse::<bool>()
                            .map(|x| Value::Boolean(x))
                            .map_err(|_| ContextualError::new("invalid boolean")),
                        "int" => value
                            .parse::<i64>()
                            .map(|x| Value::Integer(x))
                            .map_err(|_| ContextualError::new("invalid integer")),
                        "float" => value
                            .parse::<f64>()
                            .map(|x| Value::Float(x))
                            .map_err(|_| ContextualError::new("invalid float")),
                        "null" => Err(ContextualError::new("null not allowed")),
                        _ => Ok(Value::String(value)),
                    }
                } else {
                    Ok(Value::String(value))
                }
            } else {
                Ok(match Yaml::from_str(&value) {
                    Yaml::Real(v) => Value::Float(v.parse::<f64>().unwrap()),
                    Yaml::Integer(v) => Value::Integer(v),
                    Yaml::String(v) => Value::String(v),
                    Yaml::Boolean(v) => Value::Boolean(v),
                    _ => panic!("incorrect type of parsed scalar"),
                })
            }
        }
        Event::SequenceStart(_) => {
            let mut array = Vec::new();
            while parser.peek()?.0 != Event::SequenceEnd {
                array.push(load_node(parser)?);
            }
            assert_eq!(parser.next()?.0, Event::SequenceEnd);
            Ok(Value::Array(array))
        }
        Event::MappingStart(_) => {
            let mut map = LinkedHashMap::new();
            while parser.peek()?.0 != Event::MappingEnd {
                let key = load_node(parser)?;
                let value = load_node(parser)?;
                let keyspan = key.span();
                let s = if let Value::String(s) = key.as_ref() {
                    Ok(s.to_owned())
                } else {
                    Err(ContextualError::new("unexpected non-string key"))
                }?;
                map.insert(Spanned::new(s, keyspan.0, keyspan.1), value);
            }
            assert_eq!(parser.next()?.0, Event::MappingEnd);
            Ok(Value::Map(map))
        }
        _ => Err(ContextualError::new("unexpected type")),
    };
    parsed.map(|x| Spanned::new(x, start, start))
}
