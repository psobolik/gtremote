use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use git_lib::GitLib;
use gitea_api::{CreateRepoOptions, GiteaApi, Repository, TrustModel};
use url::{ParseError, Url};

use crate::browse::browse;
use crate::list::list;
use crate::{
    app_error::AppError,
    command_line_arguments::{CommandLineArguments, Commands},
    error_level::ErrorLevel,
    parameters::CreateParameters,
};

mod app_error;
mod browse;
mod command_line_arguments;
mod error_level;
mod list;
mod parameters;

#[tokio::main]
async fn main() -> ExitCode {
    let command_line_arguments = CommandLineArguments::parse();

    match &command_line_arguments.command {
        Some(Commands::List {
            gitea_url,
            filter: contains,
        }) => {
            let error_level = if let Err(error) = list(gitea_url, contains).await {
                eprintln!("Error: {}", error);
                ErrorLevel::Failure
            } else {
                ErrorLevel::Success
            };
            ExitCode::from(error_level)
        }
        Some(Commands::Browse { path, remote_name }) => {
            let error_level = if let Err(error) = browse(path, remote_name) {
                eprintln!("Error: {}", error);
                ErrorLevel::Failure
            } else {
                ErrorLevel::Success
            };
            ExitCode::from(error_level)
        }
        Some(Commands::Create {
            path,
            gitea_url,
            remote_name,
            gitea_name,
            description,
            default_branch,
            private,
            template,
        }) => {
            let error_level = if let Err(error) = create(
                path,
                gitea_url,
                gitea_name,
                description,
                default_branch,
                remote_name,
                private,
                template,
            )
            .await
            {
                eprintln!("{}", error);
                ErrorLevel::Failure
            } else {
                ErrorLevel::Success
            };
            ExitCode::from(error_level)
        }
        None => ExitCode::from(ErrorLevel::Failure),
    }
}

fn gitea_url_ev() -> Result<Url, ParseError> {
    let gitea_url_str =
        Box::new(std::env::var("GITEA_URL").unwrap_or_else(|_| String::new())).leak();
    Url::parse(gitea_url_str)
}
#[allow(clippy::too_many_arguments)]
async fn create(
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
        // Create the remote repository
        let repository = create_repository(&create_parameters).await?;
        println!("Created remote repository: {}", repository.clone_url);

        // Add the remote to the local repository
        if let Err(error) = GitLib::remote_add(
            create_parameters.remote_name(),
            repository.clone_url.as_str(),
            Some(create_parameters.path()),
        ) {
            Err(AppError::from(error.to_string()))
        } else {
            println!(
                "Tracking remote repository locally as: {}",
                create_parameters.remote_name()
            );
            println!(
                "Push: git push -u {} {}",
                create_parameters.remote_name(),
                repository.default_branch
            );
            Ok(())
        }
    }
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
