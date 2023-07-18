use anyhow::Result;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone)]
pub struct Args {
    args: Vec<String>,
    i: usize,
}

#[derive(Error, Debug)]
pub enum ArgsError {
    #[error("Argument out of bounds")]
    OutOfBounds,
    #[error("Unable to parse argument")]
    Parse,
    #[error("Argument '{0} not found")]
    Custom(String),
}

impl Args {
    pub fn new(args: impl IntoIterator<Item = impl ToString>) -> Self {
        Self {
            args: args.into_iter().map(|s| s.to_string()).collect(),
            i: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn arg<T: FromStr>(&mut self, name: &str) -> Result<T> {
        let arg = self
            .args
            .get(self.i)
            .ok_or_else(|| ArgsError::Custom(name.to_string()))?;

        let value: T = arg.parse().map_err(|_| ArgsError::Parse)?;

        self.i += 1;

        Ok(value)
    }
}
