use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context, Result};
use mdbook_preprocessor::book::{Book, Chapter};
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return;
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    if let Err(e) = handle_preprocessing() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn handle_preprocessing() -> Result<()> {
    let pre = HiddenInclude;
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

struct HiddenInclude;

impl Preprocessor for HiddenInclude {
    fn name(&self) -> &str {
        "remove-emphasis"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let book_src = ctx.config.book.src.clone();
        book.for_each_chapter_mut(|ch| match hidden_include(&book_src, ch) {
            Ok(s) => ch.content = s,
            Err(e) => eprintln!("failed to process chapter `{}`: {e:?}", ch.name),
        });
        Ok(book)
    }
}

fn hidden_include(book_src: &Path, chapter: &mut Chapter) -> Result<String> {
    let Some(chapter_path) = &chapter.path else {
        return Ok(chapter.content.clone());
    };

    let chapter_path = book_src.join(chapter_path);

    let expanded = chapter
        .content
        .lines()
        .map(|line| expand_include(&chapter_path, line))
        .collect::<Vec<_>>();
    let expanded: Result<Vec<String>> = expanded.into_iter().collect();
    Ok(expanded?.join("\n"))
}

fn expand_include(chapter_path: &Path, line: &str) -> Result<String> {
    // Find all the line with {{#hidden_include ../src/file.rs:anchor_name}}
    let path_with_anchor = line
        .trim()
        .strip_prefix("{{#hidden_include")
        .and_then(|line| line.strip_suffix("}}"));

    // this file has no source path, return the raw content
    let Some(path_with_anchor) = path_with_anchor else {
        return Ok(line.to_string());
    };

    // remote the anchor name
    let (path, anchor) = match path_with_anchor.rsplit_once(':') {
        Some((p, a)) => (p.trim(), Some(a)),
        _ => (path_with_anchor, None),
    };

    let full_path = chapter_path
        .parent()
        .with_context(|| format!("Cannot file path path for chapter {:?}", chapter_path))?
        .join(path);

    let file = File::open(full_path)?;
    let lines: Vec<String> = BufReader::new(file).lines().collect::<Result<_, _>>()?;
    let lines = match anchor {
        Some(anchor) => {
            // find line with start anchor
            let start = lines
                .iter()
                .position(|l| {
                    l.split_once("ANCHOR:")
                        .is_some_and(|(_, a)| a.trim() == anchor)
                })
                .with_context(|| format!("Cannot find start anchor `{}`", anchor))?;

            let end = lines
                .iter()
                .position(|l| {
                    l.split_once("ANCHOR_END:")
                        .is_some_and(|(_, a)| a.trim() == anchor)
                })
                .with_context(|| format!("Cannot find end anchor `{}`", anchor))?;

            &lines[start..end + 1]
        }
        None => &lines,
    };

    let lines = lines
        .iter()
        .filter(|l| !l.contains("ANCHOR:") && !l.contains("ANCHOR_END:"))
        .map(|l| format!("# {l}"))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(lines)
}
