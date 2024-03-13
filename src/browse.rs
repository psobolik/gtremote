/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-13
 */

use crate::app_error::AppError;
use git_lib::GitLib;
use std::ffi::OsStr;
use std::path::PathBuf;

pub(crate) fn browse(path: &Option<PathBuf>, remote_name: &String) -> Result<(), AppError> {
    let path = match path {
        Some(path) => path.to_owned(),
        None => std::env::current_dir().unwrap(),
    };
    match GitLib::remote_url(remote_name.as_str(), Option::from(&path)) {
        Ok(remote_url) => {
            let ru = <String as AsRef<OsStr>>::as_ref(&remote_url);
            match open::that_detached(ru) {
                Ok(()) => {
                    println!("Opened '{}'", remote_url);
                    Ok(())
                }
                Err(error) => Err(AppError::from(format!(
                    "Error opening '{}': {}",
                    remote_name, error
                ))),
            }
        }
        Err(error) => Err(AppError::from(format!(
            "Error getting remote URL for '{}': {}",
            remote_name, error
        ))),
    }
}
