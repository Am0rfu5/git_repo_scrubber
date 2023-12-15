# Git Repo Scrubber

Remove problematic information from git repos while maintaining the history. And it is written in Rust.

## Overview

There are issues with privacy and git repos. Once code has been committed data it is not possible to simply remove sensitive or private information. There are also issues with the inclusion of large blobs that need to be removed due to inefficiencies.

Solutions are varied.  There are methods for modifyin them using the command line but they can be very tedious and error prone.  The one exception is BFG-repo-cleaner. BFG is a Java program that is good at removing large blobs from git repos. It is also very good at removing files from the history of a repo. This project was started as a method to address some of the existing issues, and an effort to perhaps build an alternative using Rust.

It is still in the very early phases of development, and under an MIT license. 