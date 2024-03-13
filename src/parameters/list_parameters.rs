/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-09
 */

use url::Url;

#[derive(Debug)]
pub struct ListParameters {
    gitea_url: Url,
    filter: Option<String>,
}

impl ListParameters {
    pub fn new(gitea_url: Url, filter: Option<String>) -> ListParameters {
        ListParameters { gitea_url, filter }
    }

    pub fn gitea_url(&self) -> &Url {
        &self.gitea_url
    }
    pub fn filter(&self) -> &Option<String> {
        &self.filter
    }
}
