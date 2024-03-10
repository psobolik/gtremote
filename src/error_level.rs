/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-09
 */

use std::process::ExitCode;

pub enum ErrorLevel {
    Success = 0,
    Failure = 1,
}

impl From<ErrorLevel> for ExitCode {
    fn from(error_level: ErrorLevel) -> Self {
        ExitCode::from(error_level as u8)
    }
}
