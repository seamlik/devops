# Various DevOps tasks

Centralized DevOps tasks for my personal projects.

This program must only run in the root directory of a project.

## Available Tasks

All tasks are designed as subcommands.

### format

Formats all files in known formats.
Because there are countless file formats in the world, one must specify what kind of files to format.
However, some file formats (e.g., the source code of some programming languages) are automatically detected because their formatters are integrated into their build system.
In those cases, we simply detect if a build script or project definition file exist and run the standard command.

Here are all the file formats we support:

| Formatting | Formatter                                       | Auto-detected? |
| ---------- | ----------------------------------------------- | -------------- |
| css        | [Prettier](https://prettier.io)                 | ❌            |
| html       | [Prettier](https://prettier.io)                 | ❌            |
| json       | [Prettier](https://prettier.io)                 | ❌            |
| markdown   | [Prettier](https://prettier.io)                 | ❌            |
| rust       | [rustfmt](https://github.com/rust-lang/rustfmt) | ✔️            |
| spotless   | [Spotless](https://github.com/diffplug/spotles) | ✔️            |
| xml        | [LemMinX](https://github.com/eclipse/lemminx)   | ❌            |
| yaml       | [Prettier](https://prettier.io)                 | ❌            |

These files, when existing, will be considered when deciding if a file will be ignored:

* .gitignore
* .prettierignore

#### Usage

```powershell
devops format xml yaml ...
```

### rust-code-coverage

Generates a code coverage report using [grcov](https://github.com/mozilla/grcov).

#### Usage

```powershell
devops rust-code-coverage
```

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
