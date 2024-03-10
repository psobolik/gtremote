/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-08
 */

use std::fmt::{Debug, Display, Formatter};

pub enum ParametersError {
    GitCommand(git_lib::git_command::error::Error),
    Url(url::ParseError),
    Other(String),
}

impl Display for ParametersError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ParametersError::GitCommand(error) => error.to_string(),
            ParametersError::Url(error) => error.to_string(),
            ParametersError::Other(error) => error.to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Debug for ParametersError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<git_lib::git_command::error::Error> for ParametersError {
    fn from(err: git_lib::git_command::error::Error) -> ParametersError {
        ParametersError::GitCommand(err)
    }
}

impl From<url::ParseError> for ParametersError {
    fn from(err: url::ParseError) -> ParametersError {
        ParametersError::Url(err)
    }
}

impl From<String> for ParametersError {
    fn from(err: String) -> ParametersError {
        ParametersError::Other(err)
    }
}

impl From<&str> for ParametersError {
    fn from(err: &str) -> ParametersError {
        ParametersError::Other(err.to_string())
    }
}
