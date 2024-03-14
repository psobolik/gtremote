/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-13
 */

use crate::app_error::AppError;
use gitea_api::{GiteaApi, SearchReposResult};
use url::Url;

// The list command doesn't prompt for missing options, but it will use the
// value in the GITEA_URL environment variable if it exists.
pub(crate) async fn list(gitea_url: &Option<Url>, filter: &Option<String>) -> Result<(), AppError> {
    let gitea_url = match gitea_url {
        Some(gitea_url) => gitea_url.to_owned(),
        None => {
            let gitea_url_str =
                Box::new(std::env::var("GITEA_URL").unwrap_or_else(|_| String::new())).leak();
            if let Ok(gitea_url) = Url::parse(gitea_url_str) {
                gitea_url
            } else {
                // The url ParseError is not very meaningful
                return Err(AppError::from("Missing or invalid Gitea URL"));
            }
        }
    };
    // let list_parameters = ListParameters::new(gitea_url.clone(), filter.clone());
    let gitea_api = GiteaApi::new(gitea_url.as_str(), None, None);
    match gitea_api.search_repos(Option::from(filter)).await {
        Ok(result) => {
            if result.ok() {
                if result.repositories().is_empty() {
                    crate::print_info!("No matches");
                    Ok(())
                } else {
                    let full_name_width = full_name_width(&result);
                    let clone_url_width = clone_url_width(&result);
                    let description_width = description_width(&result);
                    println!(
                        "{:<full_name_width$} {:<clone_url_width$} Description",
                        "Name", "Clone URL"
                    );
                    println!(
                        "{:=<full_name_width$} {:=<clone_url_width$} {:=<description_width$}",
                        "", "", ""
                    );
                    for repository in result.repositories().iter() {
                        println!(
                            "{:<full_name_width$} {:<clone_url_width$} {}",
                            repository.full_name, repository.clone_url, repository.description,
                        );
                    }
                    Ok(())
                }
            } else {
                Err(AppError::from("Failed to get repositories"))
            }
        }
        Err(error) => Err(AppError::from(error)),
    }
}

fn full_name_width(search_repos_result: &SearchReposResult) -> usize {
    search_repos_result
        .repositories()
        .iter()
        .fold(0, |acc, repository| {
            let len = repository.full_name.len();
            if len > acc {
                len
            } else {
                acc
            }
        })
        + 1
}

fn clone_url_width(search_repos_result: &SearchReposResult) -> usize {
    search_repos_result
        .repositories()
        .iter()
        .fold(0, |acc, repository| {
            let len = repository.clone_url.len();
            if len > acc {
                len
            } else {
                acc
            }
        })
        + 1
}

fn description_width(search_repos_result: &SearchReposResult) -> usize {
    search_repos_result
        .repositories()
        .iter()
        .fold(0, |acc, repository| {
            let len = if repository.description.is_empty() {
                "Description"
            } else {
                repository.description.split('\n').fold("", |acc, line| {
                    if line.len() > acc.len() {
                        line
                    } else {
                        acc
                    }
                })
            }
            .len();
            if len > acc {
                len
            } else {
                acc
            }
        })
}
