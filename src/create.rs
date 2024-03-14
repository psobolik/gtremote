/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-13
 */

use std::io::{BufRead, Write};
use crate::{app_error::AppError, create::create_parameters::CreateParameters};
use git_lib::GitLib;
use gitea_api::{CreateRepoOptions, GiteaApi, Repository, TrustModel};
use std::path::PathBuf;
use url::Url;

mod create_parameters;
pub mod parameters_error;

#[allow(clippy::too_many_arguments)]
pub(crate) async fn create(
    path: &Option<PathBuf>,
    gitea_url: &Option<Url>,
    gitea_name: &Option<String>,
    description: &Option<String>,
    default_branch: &Option<String>,
    remote_name: &Option<String>,
    private: &Option<bool>,
    template: &Option<bool>,
) -> Result<(), AppError> {
    let create_parameters = CreateParameters::prompt_for_missing(
        path,
        gitea_url,
        remote_name,
        gitea_name,
        description,
        default_branch,
        private,
        template,
    )?;
    {
        if !confirm()? {
            return Err(AppError::from("Cancelled"));
        }
        // Create the remote repository
        let repository = create_repository(&create_parameters).await?;
        println!("✔️ Created remote repository: {}", repository.clone_url);

        // Add the remote to the local repository
        if let Err(error) = GitLib::remote_add(
            create_parameters.remote_name(),
            repository.clone_url.as_str(),
            Some(create_parameters.path()),
        ) {
            Err(AppError::from(error.to_string()))
        } else {
            println!(
                "✔️ Tracking remote repository locally as: {}",
                create_parameters.remote_name()
            );
            println!(
                "ℹ️ Push: git push -u {} {}",
                create_parameters.remote_name(),
                repository.default_branch
            );
            Ok(())
        }
    }
}

fn confirm() -> Result<bool, std::io::Error> {
    let mut bucket = String::with_capacity(2048);
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    
    // We do this because stdout may be buffered
    write!(stdout, "☑️ Continue? [y]|n: ")?;
    stdout.flush()?;

    stdin.read_line(&mut bucket)?;
    let result = bucket.trim_end_matches('\n').trim_end_matches('\r');
    Ok(if result.is_empty() { true } else { result.to_lowercase() == *"true" })

}
async fn create_repository(parameters: &CreateParameters) -> Result<Repository, AppError> {
    let url = parameters.gitea_url().as_str();
    // Need credentials for the Gitea server
    match GitLib::credentials_fill(url) {
        Ok(credentials) => {
            let gitea_api = GiteaApi::new(
                url,
                credentials.username().as_deref(),
                credentials.password().as_deref(),
            );
            let create_repo_options = create_repo_options(parameters);
            Ok(gitea_api.create_repo(&create_repo_options).await?)
        }
        Err(error) => Err(AppError::from(error.to_string())),
    }
}

fn create_repo_options(create_parameters: &CreateParameters) -> CreateRepoOptions {
    let description = if create_parameters.description().is_empty() {
        None
    } else {
        Some(create_parameters.description().to_string())
    };

    CreateRepoOptions::new(
        create_parameters.gitea_name().to_string(), // name: String,
        create_parameters.default_branch().to_string(), // default_branch: String,
        TrustModel::Default,                        // trust_model: TrustModel,
        false,                                      // auto_init: bool,
        *create_parameters.private(),               // private: bool,
        *create_parameters.template(),              // template: bool,
        description,                                // description: Option<String>,
        None,                                       // gitignores: Option<String>,
        None,                                       // issue_labels: Option<String>,
        None,                                       // license: Option<String>,
        None,                                       // readme: Option<String>,
    )
}
