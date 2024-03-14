/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-10
 */
use std::io::{BufRead, StdinLock, StdoutLock, Write};
use std::path::PathBuf;

use url::Url;

use super::parameters_error::ParametersError;

#[derive(Debug)]
pub struct CreateParameters {
    gitea_url: Url,
    path: PathBuf,
    remote_name: String,
    gitea_name: String,
    description: String,
    default_branch: String,
    private: bool,
    template: bool,
    // trust_model: TrustModel
}

impl CreateParameters {
    pub fn gitea_url(&self) -> &Url {
        &self.gitea_url
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn remote_name(&self) -> &String {
        &self.remote_name
    }
    pub fn gitea_name(&self) -> &String {
        &self.gitea_name
    }
    pub fn description(&self) -> &String {
        &self.description
    }
    pub fn default_branch(&self) -> &String {
        &self.default_branch
    }
    pub fn private(&self) -> &bool {
        &self.private
    }
    pub fn template(&self) -> &bool {
        &self.template
    }

    #[allow(clippy::too_many_arguments)]
    pub fn prompt_for_missing(
        path: &Option<PathBuf>,
        gitea_url: &Option<Url>,
        remote_name: &Option<String>,
        gitea_name: &Option<String>,
        description: &Option<String>,
        default_branch: &Option<String>,
        private: &Option<bool>,
        template: &Option<bool>,
    ) -> Result<CreateParameters, ParametersError> {
        let mut bucket = String::with_capacity(2048);
        let mut stdin = std::io::stdin().lock();
        let mut stdout = std::io::stdout().lock();

        let path = maybe_prompt_for_path(path, &mut bucket, &mut stdin, &mut stdout)?;
        bucket.clear();

        let gitea_url =
            maybe_prompt_for_gitea_url(gitea_url, &mut bucket, &mut stdin, &mut stdout)?;
        bucket.clear();

        let remote_name = maybe_prompt_for_string(
            remote_name,
            "origin",
            "Remote name",
            &mut bucket,
            &mut stdin,
            &mut stdout,
        )?;
        bucket.clear();

        let gitea_name = maybe_prompt_for_string(
            gitea_name,
            path.file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
            "Repository name",
            &mut bucket,
            &mut stdin,
            &mut stdout,
        )?;
        bucket.clear();

        let description = maybe_prompt_for_string(
            description,
            "",
            "Repository description",
            &mut bucket,
            &mut stdin,
            &mut stdout,
        )?;
        bucket.clear();

        let default_branch = maybe_prompt_for_string(
            default_branch,
            "main",
            "Default branch",
            &mut bucket,
            &mut stdin,
            &mut stdout,
        )?;
        bucket.clear();

        let private = maybe_prompt_for_bool(
            private,
            false,
            "Private",
            &mut bucket,
            &mut stdin,
            &mut stdout,
        )?;
        bucket.clear();

        let template = maybe_prompt_for_bool(
            template,
            false,
            "Template",
            &mut bucket,
            &mut stdin,
            &mut stdout,
        )?;
        bucket.clear();

        Ok(CreateParameters {
            path,
            gitea_url,
            remote_name,
            gitea_name,
            description,
            default_branch,
            private,
            template,
        })
    }
}

fn maybe_prompt_for_path(
    path: &Option<PathBuf>,
    bucket: &mut String,
    stdin: &mut StdinLock,
    stdout: &mut StdoutLock,
) -> Result<PathBuf, ParametersError> {
    const PATH_PROMPT: &str = "Repository path";
    if let Some(path) = path {
        // Display the path if it was  specified
        display_value(PATH_PROMPT, path.to_str().unwrap_or_default());
        Ok(path.clone())
    } else {
        // Prompt for the path if it wasn't specified
        // Suggest the current directory as the default
        let default_path = std::env::current_dir().unwrap_or_default();
        let value = prompt_for_string(
            PATH_PROMPT,
            default_path.to_str().unwrap_or_default(),
            bucket,
            stdin,
            stdout,
        )?;
        Ok(PathBuf::from(value))
    }
}

fn maybe_prompt_for_gitea_url(
    gitea_url: &Option<Url>,
    bucket: &mut String,
    stdin: &mut StdinLock,
    stdout: &mut StdoutLock,
) -> Result<Url, ParametersError> {
    const GITEA_URL_PROMPT: &str = "Gitea URL";
    if let Some(gitea_url) = gitea_url {
        // Display the remote name if it was  specified
        display_value(GITEA_URL_PROMPT, gitea_url.as_str());
        Ok(gitea_url.clone())
    } else {
        // Prompt for the Gitea repository URL it wasn't specified
        // Suggest the value of the GITEA_URL environment variable as the default (if it exists)
        let default_gitea_url =
            Box::new(std::env::var("GITEA_URL").unwrap_or_else(|_| String::new())).leak();
        let value = prompt_for_string(GITEA_URL_PROMPT, default_gitea_url, bucket, stdin, stdout)?;
        match Url::parse(value.as_str()) {
            Ok(gitea_url) => Ok(gitea_url),
            Err(_) => Err(ParametersError::from("Missing or invalid Gitea URL")),
        }
    }
}

fn maybe_prompt_for_string(
    value: &Option<String>,
    default: &str,
    prompt: &str,
    bucket: &mut String,
    stdin: &mut StdinLock,
    stdout: &mut StdoutLock,
) -> Result<String, ParametersError> {
    if let Some(value) = value {
        // Display the value if it was specified
        display_value(prompt, value);
        Ok(value.clone())
    } else {
        // Prompt for the value if it wasn't specified
        Ok(prompt_for_string(prompt, default, bucket, stdin, stdout)?)
    }
}

fn maybe_prompt_for_bool(
    value: &Option<bool>,
    default: bool,
    prompt: &str,
    bucket: &mut String,
    stdin: &mut StdinLock,
    stdout: &mut StdoutLock,
) -> Result<bool, ParametersError> {
    let true_str = true.to_string().to_lowercase();
    let false_str = false.to_string().to_lowercase();

    if let Some(value) = value {
        // Display the value if it was specified
        display_value(prompt, if *value { &true_str } else { &false_str });
        Ok(*value)
    } else {
        // Prompt for the value if it wasn't specified
        Ok(prompt_for_string(
            format!("{prompt} ({true_str} or {false_str})").as_str(),
            if default { &true_str } else { &false_str }.as_str(),
            bucket,
            stdin,
            stdout,
        )?
        .to_lowercase()
            == true_str)
    }
}

fn prompt_for_string(
    prompt: &str,
    default: &str,
    bucket: &mut String,
    stdin: &mut StdinLock,
    stdout: &mut StdoutLock,
) -> Result<String, ParametersError> {
    let mut prompt_string = format!("➕ {prompt}");
    // let mut prompt_string = String::from(prompt);
    if !default.is_empty() {
        prompt_string.push_str(format!(" [default: {default}]").as_str());
    }
    // We do this because stdout may be buffered
    write!(stdout, "{prompt_string}: ")?;
    stdout.flush().expect("Write failed");

    stdin.read_line(bucket)?;
    let result = bucket.trim_end_matches('\n').trim_end_matches('\r');
    Ok(if result.is_empty() { default } else { result }.to_string())
}

fn display_value(prompt: &str, value: &str) {
    println!("➖️{}: {}", prompt, value);
}
