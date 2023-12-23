# Git Updater Design

Removes problematic information from git repos while maintaining the history. And it is written in Rust.

## Problem

Privacy protection for git repos is difficult. Due to the nature of git as a distributed version control system using a hash tree it is difficult to remove sensitive information. Once code has been committed it is not possible to simply remove parts such as email addresses, private keys, names, etc. There are also issues with the inclusion of large blobs that need to be removed from the history in order to prune the size of repo. 

The problem is that it is not possible to remove all the information from the history and retain the history with maximal accuracy.

## Existing Solutions

## Rebase
It is possible to perform an interactive rebase to update multiple commits.  For example, this git command will update the author and email addresses for each commit to and make every commit a single name and email address.

```bash
git -i rebase -root -x "git commit --amend author='~name~  <~your@email~> --no-edit'"
```
Although this can work, it can be a mess and is error prone. It quickly becomes more problematic if you only want to update some of the commits.

Take our example of modifying the author. If we may have multiple users and only want to alter the author from some users we could interactively select the command to run on each commit but for a repo of any size this would very quickly becomes tedious, require a lot of time and possibly cause merging or other issues.

The --no-edit flag is used to prevent the editor from being opened and thus it will retain the original commit message.  This does update the dates for each commit to the current date. This may not be an issue but if you are trying to minimize the impact on the history this will show serious deviation.

The two location of potential problematic data is another issue that we set out to solve.  The Author and Commit fields both show the same information. Likewise with the AuthorDate and the CommitDate. So while we could add the date to the git commit command we would also need to add the date for the CommitDate. In a git command this would be done with the GIT_COMMITER_DATE environment variable.

If we also want to modify the author we need to add the GIT_COMMMIT_NAME and GIT_COMMITER_EMAIL environment variables.

Our git command starts to look more complicated.

```bash
git -i rebase -root -x "GIT_COMMITER_DATE='~date~' git commit --amend --date='~date~' --no-edit"
```

## git-filter-branch
There is a 'git-filter-branch', but this is slow and difficult to use.  It is much more efficient to use the 'git rebase' command for the above example but more complex changes require a different method.

## BFG-Repo-Cleaner
This is the best existing solution. It is fast, efficient and easy to use. However, it is not perfect. For one it is written in Scala/Java and not in Rust..

The BFG-Repo-Cleaner does not give us the perfect result. The BFG only preserves AuthorDate which is what is seen normally when using 'git-log', 'git-show' and in many Git tools and IDEs. However, some tools and repo sites, including Github, display the CommitDate.  This can be confusing and misleading. This is one of the reasons we set out to create our own solution. 