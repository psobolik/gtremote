/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-08
 */

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use url::Url;

#[derive(Parser)]
#[command(version, about, long_about = None)]
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
        /// Repository path (default: current path)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Remote name (default: 'origin')
        #[arg(short, long)]
        remote_name: Option<String>,
    },
    /// Create a remote repository and track it locally
    Create {
        /// Gitea URL
        #[arg(short('u'), long)]
        gitea_url: Option<Url>,

        /// Repository path (default: current path)
        #[arg(long)]
        path: Option<PathBuf>,

        /// Name of remote repository (default: 'origin')
        #[arg(short, long)]
        remote_name: Option<String>,

        /// Repository name (default: name of current path's folder)
        #[arg(short, long)]
        gitea_name: Option<String>,

        /// Repository description
        #[arg(short, long)]
        description: Option<String>,

        /// Default branch (default: 'main')
        #[arg(short('b'), long)]
        default_branch: Option<String>,

        /// Should the repository be private? (default: false)
        #[arg(short, long)]
        private: Option<bool>,

        /// Should the repository be a template? (default: false)
        #[arg(short, long)]
        template: Option<bool>,
    },
}
