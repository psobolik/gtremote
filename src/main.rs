use std::process::ExitCode;

use clap::Parser;

use crate::{
    browse::browse,
    command_line_arguments::{CommandLineArguments, Commands},
    create::create,
    error_level::ErrorLevel,
    list::list,
};

mod app_error;
mod browse;
mod command_line_arguments;
mod create;
mod error_level;
mod list;

#[tokio::main]
async fn main() -> ExitCode {
    let command_line_arguments = CommandLineArguments::parse();

    match &command_line_arguments.command {
        Some(Commands::List {
            gitea_url,
            filter: contains,
        }) => {
            let error_level = if let Err(error) = list(gitea_url, contains).await {
                eprintln!("💥 Error: {}", error);
                ErrorLevel::Failure
            } else {
                ErrorLevel::Success
            };
            ExitCode::from(error_level)
        }
        Some(Commands::Browse { path, remote_name }) => {
            let error_level = if let Err(error) = browse(path, remote_name) {
                eprintln!("💥 Error: {}", error);
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
                eprintln!("💥 Error: {}", error);
                ErrorLevel::Failure
            } else {
                ErrorLevel::Success
            };
            ExitCode::from(error_level)
        }
        None => ExitCode::from(ErrorLevel::Failure),
    }
}
