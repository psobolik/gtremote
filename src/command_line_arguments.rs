/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-08
 */

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use url::Url;

#[derive(Parser)]
#[command(version, about, long_about = None, author, arg_required_else_help = true)]
pub struct CommandLineArguments {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List repositories
    List {
        /// Gitea URL
        #[arg(short, long)]
        gitea_url: Option<Url>,

        /// Only list remotes whose name contains this value
        #[arg()]
        filter: Option<String>,
    },
    /// Open remote repository URL in default browser
    Browse {
        /// Repository path [default: current path]
        #[arg(long)]
        path: Option<PathBuf>,

        /// Remote name
        #[arg(short, long, default_value = "origin")]
        remote_name: String,
    },
    /// Create a remote repository and track it locally
    Create {
        /// Gitea URL
        #[arg(short('u'), long)]
        gitea_url: Option<Url>,

        /// Repository path [default: <current path>]
        #[arg(long)]
        path: Option<PathBuf>,

        /// Name of remote repository [default: origin]
        #[arg(short, long)]
        remote_name: Option<String>,

        /// Repository name [default: name of current path's folder}
        #[arg(short, long)]
        gitea_name: Option<String>,

        /// Repository description
        #[arg(short, long)]
        description: Option<String>,

        /// Default branch [default: main]
        #[arg(short('b'), long)]
        default_branch: Option<String>,

        /// The repository should be private
        #[arg(long, conflicts_with = "not_private")]
        private: bool,

        /// The repository should *not* be private (default)
        #[arg(long, value_name = "not-private", conflicts_with = "private")]
        not_private: bool,

        /// The repository should be a template
        #[arg(long, conflicts_with = "not_template")]
        template: bool,

        /// The repository should *not* be a template (default)
        #[arg(long, value_name = "not-template", conflicts_with = "template")]
        not_template: bool,
    },
}
