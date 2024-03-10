/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-10
 */
use crate::parameters::{prompt_for_path, prompt_for_remote_name, ParametersError};
use std::path::PathBuf;

#[derive(Debug)]
pub struct BrowseParameters {
    path: PathBuf,
    remote_name: String,
}

impl BrowseParameters {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn remote_name(&self) -> &String {
        &self.remote_name
    }

    pub fn prompt_for_missing(
        path: &Option<PathBuf>,
        remote_name: &Option<String>,
    ) -> Result<BrowseParameters, ParametersError> {
        let mut bucket = String::with_capacity(2048);
        let mut stdin = std::io::stdin().lock();
        let mut stdout = std::io::stdout().lock();

        let path = prompt_for_path(path, &mut bucket, &mut stdin, &mut stdout)?;
        bucket.clear();
        let remote_name =
            prompt_for_remote_name(remote_name, &mut bucket, &mut stdin, &mut stdout)?;
        bucket.clear();

        Ok(BrowseParameters { path, remote_name })
    }
}
