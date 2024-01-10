//! Iroha configuration and related utilities.

// FIXME
#![allow(unused, missing_docs, missing_copy_implementations)]

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{Debug, Display, Formatter},
    io::Read,
    ops::Sub,
    str::FromStr,
    time::Duration,
};

use eyre::{eyre, Report, Result, WrapErr};
use serde::{Deserialize, Serialize};

pub mod client_api;
pub mod parameters;

/// User-provided duration
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct UserDuration(Duration);

impl UserDuration {
    pub fn get(self) -> Duration {
        self.0
    }
}

/// Byte size
#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct ByteSize(u64);

pub trait Complete {
    type Output;

    fn complete(self) -> CompleteResult<Self::Output>;
}

// struct StandardEnv;

pub trait ReadEnv {
    fn get(&self, key: impl AsRef<str>) -> Option<&str>;
}

pub trait FromEnv {
    fn from_env(env: &impl ReadEnv) -> FromEnvResult<Self>
    where
        Self: Sized;
}

pub type FromEnvResult<T> = Result<T, ErrorsCollection<Report>>;

pub trait FromEnvDefaultFallback {}

impl<T> FromEnv for T
where
    T: FromEnvDefaultFallback + Default,
{
    fn from_env(env: &impl ReadEnv) -> FromEnvResult<Self>
    where
        Self: Sized,
    {
        Ok(Self::default())
    }
}

pub struct Emitter<T: Debug> {
    errors: Vec<T>,
    bomb: drop_bomb::DropBomb,
}

impl<T: Debug> Emitter<T> {
    fn new() -> Self {
        Self {
            errors: Vec::new(),
            bomb: drop_bomb::DropBomb::new(
                "Errors emitter is dropped without consuming collected errors",
            ),
        }
    }

    fn emit(&mut self, error: T) {
        self.errors.push(error);
    }

    fn emit_collection(&mut self, mut errors: ErrorsCollection<T>) {
        self.errors.append(&mut errors.0);
    }

    fn finish(mut self) -> Result<(), ErrorsCollection<T>> {
        self.bomb.defuse();

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(ErrorsCollection(self.errors))
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CompleteError {
    #[error("Missing field: {path}")]
    MissingField { path: String },
    #[error(transparent)]
    Custom(#[from] Report),
}

pub type CompleteResult<T> = Result<T, ErrorsCollection<CompleteError>>;

impl CompleteError {
    pub fn missing_field(field_name: impl AsRef<str>) -> Self {
        Self::MissingField {
            path: field_name.as_ref().to_string(),
        }
    }
}

impl Emitter<CompleteError> {
    fn emit_missing_field(&mut self, field_name: impl AsRef<str>) {
        self.emit(CompleteError::MissingField {
            path: field_name.as_ref().to_string(),
        })
    }
}

pub struct ErrorsCollection<T>(Vec<T>);

impl<T: Display + Debug> Error for ErrorsCollection<T> {}

impl<T> Display for ErrorsCollection<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{item}")?;
        }
        Ok(())
    }
}

impl<T> Debug for ErrorsCollection<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{item:?}")?;
        }
        Ok(())
    }
}

impl<T> From<T> for ErrorsCollection<T> {
    fn from(value: T) -> Self {
        Self(vec![value])
    }
}

impl<T> IntoIterator for ErrorsCollection<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Default)]
pub struct TestEnv {
    map: HashMap<String, String>,
    visited: RefCell<HashSet<String>>,
}

impl TestEnv {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_map(map: HashMap<String, String>) -> Self {
        Self { map, ..Self::new() }
    }

    #[must_use]
    pub fn set(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.map
            .insert(key.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    pub fn unvisited(&self) -> HashSet<String> {
        let all_keys: HashSet<_> = self.map.keys().map(ToOwned::to_owned).collect();
        let visited: HashSet<_> = self.visited.borrow().clone();
        all_keys.sub(&visited)
    }
}

impl ReadEnv for TestEnv {
    fn get(&self, key: impl AsRef<str>) -> Option<&str> {
        self.visited.borrow_mut().insert(key.as_ref().to_string());
        self.map.get(key.as_ref()).map(std::string::String::as_str)
    }
}

enum ParseEnvResult<T> {
    Value(T),
    ParseError,
    None,
}

impl<T> ParseEnvResult<T>
where
    T: FromStr,
    <T as FromStr>::Err: Error + Send + Sync + 'static,
{
    fn parse_simple(
        emitter: &mut Emitter<Report>,
        env: &impl ReadEnv,
        env_key: impl AsRef<str>,
        field_name: impl AsRef<str>,
    ) -> Self {
        match env
            .get(env_key.as_ref())
            .map(FromStr::from_str)
            .transpose()
            .wrap_err_with(|| {
                eyre!(
                    "failed to parse `{}` field from `{}` env variable",
                    field_name.as_ref(),
                    env_key.as_ref()
                )
            }) {
            Ok(Some(x)) => Self::Value(x),
            Ok(None) => Self::None,
            Err(report) => {
                emitter.emit(report);
                Self::ParseError
            }
        }
    }
}

impl<T> From<ParseEnvResult<T>> for Option<T> {
    fn from(value: ParseEnvResult<T>) -> Self {
        match value {
            ParseEnvResult::None | ParseEnvResult::ParseError => None,
            ParseEnvResult::Value(x) => Some(x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_missing_field() {
        let mut emitter = Emitter::new();

        emitter.emit(CompleteError::missing_field("foo"));

        let err = emitter.finish().unwrap_err();

        assert_eq!(format!("{err}"), "Missing field: foo")
    }

    #[test]
    fn multiple_missing_fields() {
        let mut emitter = Emitter::new();

        emitter.emit(CompleteError::missing_field("foo"));
        emitter.emit(CompleteError::missing_field("bar"));

        let err = emitter.finish().unwrap_err();

        assert_eq!(format!("{err}"), "Missing field: foo\nMissing field: bar")
    }
}
