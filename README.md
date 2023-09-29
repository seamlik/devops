# Various DevOps tasks

Centralized DevOps tasks for my personal projects.

This program must only run in the root directory of a project.

## Background

Previously, all DevOps tasks were written in PowerShell and scattered among my projects.
Over time, maintaining them became more and more costly because:

* PowerShell, like all scripting languages, are not compiled or statically typed.
* Scripts are scattered in all my projects without possibility of reuse.
* Improvments to the scripts in some projects must be manually backported to other projects.

Until one day I overheard that some team in a certain huge company started to write all their DevOps tasks in Rust.
As a result, their Jenkins and TeamCity scripts were greatly simplified.
Migrating to other CI servers would also cost less.
So, why don't I follow this path?
