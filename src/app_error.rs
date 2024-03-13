/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-09
 */
use std::fmt::{Debug, Display, Formatter};

pub enum AppError {
    Url(url::ParseError),
    Api(gitea_api::api_error::ApiError),
    Parameter(crate::parameters::ParametersError),
    Other(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AppError::Url(error) => error.to_string(),
            AppError::Api(error) => error.to_string(),
            AppError::Parameter(error) => error.to_string(),
            AppError::Other(error) => error.to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<gitea_api::api_error::ApiError> for AppError {
    fn from(err: gitea_api::api_error::ApiError) -> AppError {
        AppError::Api(err)
    }
}

impl From<crate::parameters::ParametersError> for AppError {
    fn from(err: crate::parameters::ParametersError) -> AppError {
        AppError::Parameter(err)
    }
}

impl From<url::ParseError> for AppError {
    fn from(err: url::ParseError) -> AppError {
        AppError::Url(err)
    }
}

impl From<String> for AppError {
    fn from(err: String) -> AppError {
        AppError::Other(err)
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> AppError {
        AppError::Other(err.to_string())
    }
}
