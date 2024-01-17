# Command-Line Help for `markdown-codeblocks`

This document contains the help content for the `markdown-codeblocks` command-line program.

**Command Overview:**

* [`markdown-codeblocks`↴](#markdown-codeblocks)

## `markdown-codeblocks`

Extract code blocks from a Markdown document and yield them as JSON

**Usage:** `markdown-codeblocks [OPTIONS] [PATH]`

###### **Arguments:**

* `<PATH>` — The path to a Markdown file

###### **Options:**

* `--interpret-info-string` — Interpret the info string as a comma-separated `language` and an array of `parameters` (not strictly GFM-compliant)

  Possible values: `true`, `false`

* `--help-markdown` — Print command help as Markdown

  Possible values: `true`, `false`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

