/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-09
 */

use url::Url;

use crate::parameters::{prompt_for_filter, prompt_for_gitea_url, ParametersError};

#[derive(Debug)]
pub struct ListParameters {
    gitea_url: Url,
    filter: Option<String>,
}

impl ListParameters {
    pub fn gitea_url(&self) -> &Url {
        &self.gitea_url
    }
    pub fn filter(&self) -> &Option<String> {
        &self.filter
    }

    pub fn prompt_for_missing(
        gitea_url: &Option<Url>,
        filter: &Option<String>,
    ) -> Result<ListParameters, ParametersError> {
        let mut bucket = String::with_capacity(2048);
        let mut stdin = std::io::stdin().lock();
        let mut stdout = std::io::stdout().lock();

        let filter = prompt_for_filter(filter, &mut bucket, &mut stdin, &mut stdout);
        bucket.clear();

        let gitea_url = prompt_for_gitea_url(gitea_url, &mut bucket, &mut stdin, &mut stdout)?;
        bucket.clear();

        Ok(ListParameters { gitea_url, filter })
    }
}
