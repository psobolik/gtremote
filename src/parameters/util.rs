/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-10
 */

use crate::parameters::ParametersError;
use git_lib::GitLib;
use std::io::{BufRead, StdinLock, StdoutLock, Write};
use std::path::PathBuf;
use url::Url;

pub fn prompt_for_path(
    path: &Option<PathBuf>,
    bucket: &mut String,
    stdin: &mut StdinLock,
    stdout: &mut StdoutLock,
) -> Result<PathBuf, ParametersError> {
    const PATH_PROMPT: &str = "Repository path";
    let mut path = if let Some(path) = path {
        // Display the path if it was  specified
        display_value(PATH_PROMPT, path.to_str().unwrap());
        path.clone()
    } else {
        // Prompt for the path if it wasn't specified
        // Suggest the current directory as the default
        let default_path = std::env::current_dir().unwrap();
        let value = prompt_for_value(
            PATH_PROMPT,
            default_path.to_str().unwrap(),
            bucket,
            stdin,
            stdout,
        );
        PathBuf::from(value)
    };

    // Return the top-level Git folder of the specified path
    // (Fails if the path is not in a Git repository)
    path = GitLib::top_level(Some(&path))?;
    Ok(path)
}

pub fn prompt_for_gitea_url(
    gitea_url: &Option<Url>,
    bucket: &mut String,
    stdin: &mut StdinLock,
    stdout: &mut StdoutLock,
) -> Result<Url, ParametersError> {
    const GITEA_URL_PROMPT: &str = "Gitea URL";
    let gitea_url = if let Some(gitea_url) = gitea_url {
        // Display the remote name if it was  specified
        display_value(GITEA_URL_PROMPT, gitea_url.as_str());
        gitea_url.clone()
    } else {
        // Prompt for the Gitea repository URL it wasn't specified
        // Suggest the value of the GITEA_URL environment variable as the default (if it exists)
        let default_gitea_url =
            Box::new(std::env::var("GITEA_URL").unwrap_or_else(|_| String::new())).leak();
        let value = prompt_for_value(GITEA_URL_PROMPT, default_gitea_url, bucket, stdin, stdout);
        Url::parse(value.as_str())?
    };
    Ok(gitea_url)
}

pub fn prompt_for_remote_name(
    remote_name: &Option<String>,
    bucket: &mut String,
    stdin: &mut StdinLock,
    stdout: &mut StdoutLock,
) -> Result<String, ParametersError> {
    const REMOTE_NAME_PROMPT: &str = "Remote name";
    let remote_name = if let Some(remote_name) = remote_name {
        // Display the remote name if it was  specified
        display_value(REMOTE_NAME_PROMPT, remote_name);
        remote_name.clone()
    } else {
        // Prompt for the remote name if it wasn't specified
        // Suggest "origin" as the default
        prompt_for_value(REMOTE_NAME_PROMPT, "origin", bucket, stdin, stdout)
    };
    Ok(remote_name)
}

pub fn prompt_for_value(
    prompt: &str,
    default: &str,
    bucket: &mut String,
    stdin: &mut StdinLock,
    stdout: &mut StdoutLock,
) -> String {
    let mut prompt_string = String::from(prompt);
    if !default.is_empty() {
        prompt_string.push_str(format!(" [default: {default}]").as_str());
    }
    // We do this because stdout may be buffered
    write!(stdout, "{prompt_string}: ").expect("Write failed");
    stdout.flush().expect("Write failed");

    stdin.read_line(bucket).expect("Read Line failed");
    let result = bucket.trim_end_matches('\n').trim_end_matches('\r');
    if result.is_empty() { default } else { result }.to_string()
}

pub fn display_value(prompt: &str, value: &str) {
    println!("{}: {}", prompt, value);
}
