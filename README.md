# Git Updater

Removes problematic information from git repos while maintaining the history. And it is written in Rust.

Privacy protection for git repos is difficult. Once code has been committed data it is not possible to simply remove sensitive or private information. There are also issues with the inclusion of large blob that need to be removed. 

There are various methods for modifying git repos but they are very tedious and error prone with one exception, BFG-repo-cleaner.