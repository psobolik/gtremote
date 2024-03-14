# gtrepo
Copyright © 2024 Paul Sobolik

A command line program for working with Gitea repositories.

Note:
* The default Gitea URL may be specified in an environment variable named `GITEA_URL`.
* The program will prompt for options that are not specified in the command line, even if they are optional or have a default.
That means you'd want to specify all of the options if you were using this in a script.
---
`> gtrepo --help`
```
A command line program to work with Gitea repositories

Usage: gtrepo.exe [COMMAND]

Commands:
  list    List repositories
  browse  Open remote repository URL in default browser
  create  Create a remote repository and track it locally
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
---
`> gerepo list --help`
```
List repositories

Usage: gtrepo.exe list [OPTIONS] [FILTER]

Arguments:
  [FILTER]  Only list remotes whose name contains this value

Options:
  -g, --gitea-url <GITEA_URL>  Gitea URL
  -h, --help                   Print help
```
---
`> gtrepo browse --help`
```
Open remote repository URL in default browser

Usage: gtrepo.exe browse [OPTIONS]

Options:
      --path <PATH>                Repository path [default: current path]
  -r, --remote-name <REMOTE_NAME>  Remote name [default: 'origin']
  -h, --help                       Print help
```
---
`> gtrepo create --help`
```
Create a remote repository and track it locally

Usage: gtrepo.exe create [OPTIONS]

Options:
  -u, --gitea-url <GITEA_URL>
          Gitea URL
      --path <PATH>
          Repository path [default: current path]
  -r, --remote-name <REMOTE_NAME>
          Name of remote repository [default: 'origin']
  -g, --gitea-name <GITEA_NAME>
          Repository name [default: name of current path's folder}
  -d, --description <DESCRIPTION>
          Repository description
  -b, --default-branch <DEFAULT_BRANCH>
          Default branch [default: 'main'}
  -p, --private <PRIVATE>
          Should the repository be private? [default: false} [possible values: true, false]
  -t, --template <TEMPLATE>
          Should the repository be a template? [default: false} [possible values: true, false]
  -h, --help
          Print help
```
