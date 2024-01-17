use std::io::Read as _;
use clap::Parser as _;

/// Extract code blocks from a Markdown document and yield them as
/// JSON.
#[derive(clap::Parser)]
struct Args {
    /// The path to a Markdown file.
    path: Option<std::path::PathBuf>,

    /// Interpret the info string as a comma-separated `language` and
    /// an array of `parameters` (not strictly GFM-compliant).
    #[arg(long)]
    interpret_info_string: bool,

    /// Print command help as Markdown.
    #[arg(long, hide = true)]
    help_markdown: bool,
}

mod info_string {
    /// This part isn't strictly GFM-compliant (the standard doesn't
    /// mandate any interpretation of the info string).
    #[derive(serde::Serialize)]
    pub struct Interpreted<Language, Parameters> {
        pub language: Language,
        pub parameters: Parameters,
    }

    #[derive(serde::Serialize)]
    pub struct Uninterpreted<InfoString> {
        pub info_string: InfoString,
    }
}

#[derive(serde::Serialize)]
struct Block<Contents, InfoString> {
    #[serde(flatten)]
    info_string: Option<InfoString>,
    contents: Contents,
}

fn main() -> anyhow::Result<()> {
    let Args {
        path,
        interpret_info_string,
        help_markdown,
    } = Args::parse();

    if help_markdown {
        clap_markdown::print_help_markdown::<Args>();
        return Ok(());
    }

    let text = if let Some(path) = path {
        std::fs::read_to_string(path)?
    } else {
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text)?;
        text
    };

    for block in markdown::tokenize(&text) {
        if let markdown::Block::CodeBlock(info_string, ref contents) = block {
            let output = if interpret_info_string {
                serde_json::to_string(&Block {
                    info_string: info_string.as_ref().and_then(|info_string| {
                        let mut parts = info_string.split(',');
                        parts.next().map(|language| {
                            info_string::Interpreted {
                                language,
                                parameters: parts.collect::<Vec<_>>(),
                            }
                        })
                    }),
                    contents,
                })
            } else {
                serde_json::to_string(&Block {
                    info_string: info_string.as_ref()
                        .map(|info_string| info_string::Uninterpreted { info_string }),
                    contents,
                })
            }?;

            println!("{}", output);
        }
    }

    Ok(())
}
