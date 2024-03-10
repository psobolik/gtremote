/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-10
 */
use std::path::PathBuf;

use url::Url;

use crate::parameters::{
    display_value, prompt_for_gitea_url, prompt_for_path, prompt_for_remote_name, prompt_for_value,
    ParametersError,
};

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

        let path = prompt_for_path(path, &mut bucket, &mut stdin, &mut stdout)?;
        bucket.clear();

        let gitea_url = prompt_for_gitea_url(gitea_url, &mut bucket, &mut stdin, &mut stdout)?;
        bucket.clear();

        let remote_name =
            prompt_for_remote_name(remote_name, &mut bucket, &mut stdin, &mut stdout)?;
        bucket.clear();

        const REPOSITORY_NAME_PROMPT: &str = "Repository name";
        let gitea_name = if let Some(gitea_name) = gitea_name {
            // Display the repository name if it was specified
            display_value(REPOSITORY_NAME_PROMPT, gitea_name);
            gitea_name.clone()
        } else {
            // Prompt for the repository name if it wasn't specified
            let default = path.file_name().unwrap().to_str().unwrap();
            prompt_for_value(
                REPOSITORY_NAME_PROMPT,
                default,
                &mut bucket,
                &mut stdin,
                &mut stdout,
            )
            .clone()
        };
        bucket.clear();

        const REPOSITORY_DESCRIPTION_PROMPT: &str = "Repository description";
        let description = if let Some(description) = description {
            // Display the repository description if it was specified
            display_value(REPOSITORY_DESCRIPTION_PROMPT, description);
            description.clone()
        } else {
            // Prompt for the repository description if it wasn't specified
            prompt_for_value(
                REPOSITORY_DESCRIPTION_PROMPT,
                "",
                &mut bucket,
                &mut stdin,
                &mut stdout,
            )
            .clone()
        };
        bucket.clear();

        const DEFAULT_BRANCH_PROMPT: &str = "Default branch";
        let default_branch = if let Some(default_branch) = default_branch {
            // Display the default branch if it was specified
            display_value(DEFAULT_BRANCH_PROMPT, default_branch);
            default_branch.clone()
        } else {
            // Prompt for the default branch if it wasn't specified
            prompt_for_value(
                DEFAULT_BRANCH_PROMPT,
                "main",
                &mut bucket,
                &mut stdin,
                &mut stdout,
            )
            .clone()
        };
        bucket.clear();

        const PRIVATE_PROMPT: &str = "Private? (true or false)";
        let private = if let Some(private) = private {
            // Display the private flag if it was specified
            display_value(PRIVATE_PROMPT, private.to_string().as_str());
            *private
        } else {
            // Prompt for the private flag if it wasn't specified
            prompt_for_value(
                PRIVATE_PROMPT,
                "false",
                &mut bucket,
                &mut stdin,
                &mut stdout,
            )
            .to_lowercase()
                == "true"
        };
        bucket.clear();

        const TEMPLATE_PROMPT: &str = "Template? (true or false)";
        let template = if let Some(template) = template {
            // Display the private flag if it was specified
            display_value(TEMPLATE_PROMPT, template.to_string().as_str());
            *template
        } else {
            // Prompt for the default branch if it wasn't specified
            prompt_for_value(
                TEMPLATE_PROMPT,
                "false",
                &mut bucket,
                &mut stdin,
                &mut stdout,
            )
            .to_lowercase()
                == "true"
        };
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
