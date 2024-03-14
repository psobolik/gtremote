use std::process::ExitCode;

use clap::Parser;

use crate::{
    command_line_arguments::{CommandLineArguments, Commands},
    error_level::ErrorLevel,
};

mod app_error;
mod browse;
mod command_line_arguments;
mod create;
mod error_level;
mod list;
mod macros;

#[tokio::main]
async fn main() -> ExitCode {
    match CommandLineArguments::try_parse() {
        Err(error) => {
            print_error!("{error}");
            ExitCode::from(error.exit_code() as u8)
        }
        Ok(command_line_arguments) => {
            match &command_line_arguments.command {
                Some(Commands::List {
                         gitea_url,
                         filter: contains,
                     }) => {
                    let error_level = if let Err(error) = list::list(gitea_url, contains).await {
                        print_error!("Error: {}", error);
                        ErrorLevel::Failure
                    } else {
                        ErrorLevel::Success
                    };
                    ExitCode::from(error_level)
                }
                Some(Commands::Browse { path, remote_name }) => {
                    let error_level = if let Err(error) = browse::browse(path, remote_name) {
                        print_error!("Error: {}", error);
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
                         not_private,
                         private,
                         not_template,
                         template,
                     }) => {
                    let error_level = if let Err(error) = create::create(
                        path,
                        gitea_url,
                        gitea_name,
                        description,
                        default_branch,
                        remote_name,
                        private,
                        not_private,
                        template,
                        not_template,
                    )
                        .await
                    {
                        print_error!("Error: {}", error);
                        ErrorLevel::Failure
                    } else {
                        ErrorLevel::Success
                    };
                    ExitCode::from(error_level)
                }
                None => ExitCode::from(ErrorLevel::Failure),
            }
        }
    }
}
