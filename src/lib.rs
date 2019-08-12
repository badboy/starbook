use std::io::Write;
use std::process::{Command, Stdio};

use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::{Error, ErrorKind, Result};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct Starbook;

impl Preprocessor for Starbook {
    fn name(&self) -> &str {
        "starbook"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(Starbook::process(chapter).map(|md| {
                    chapter.content = md;
                }));
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }
}

impl Starbook {
    fn process(chapter: &mut Chapter) -> Result<String> {
        let args = ["--backend=html5", "--no-header-footer", "-"];
        let mut asciidoc = Command::new("asciidoc")
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        {
            let stdin = asciidoc.stdin.as_mut().expect("stdin of asciidoc broken");
            stdin.write_all(chapter.content.as_bytes())?;
        }
        let output  = asciidoc.wait_with_output()?;

        if output.status.success() {
            let out = output.stdout;
            let out = String::from_utf8(out)?;
            Ok(out)
        } else {
            Err(Error::from_kind(ErrorKind::Msg("asciidoc failed".into())))
        }
    }
}
