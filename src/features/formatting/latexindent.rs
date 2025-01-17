use std::{
    path::Path,
    process::{Command, Stdio},
};

use lsp_types::TextEdit;
use rowan::{TextLen, TextRange};
use tempfile::tempdir;

use crate::{
    db::{Document, Language, Workspace},
    util::line_index_ext::LineIndexExt,
    Db, LatexindentOptions,
};

pub fn format_with_latexindent(db: &dyn Db, document: Document) -> Option<Vec<TextEdit>> {
    let workspace = Workspace::get(db);
    let options = workspace.options(db);
    let target_dir = tempdir().ok()?;
    let source_dir = workspace
        .working_dir(db, document.directory(db))
        .path(db)
        .as_deref()?;

    let target_file = target_dir
        .path()
        .join(if document.language(db) == Language::Bib {
            "file.bib"
        } else {
            "file.tex"
        });
    std::fs::write(&target_file, document.contents(db).text(db)).ok()?;

    let args = build_arguments(&options.latexindent, &target_file);

    log::debug!(
        "Running latexindent in folder \"{}\" with args: {:?}",
        source_dir.display(),
        args,
    );

    let output = Command::new("latexindent")
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .current_dir(source_dir)
        .output()
        .ok()?;

    let old_text = document.contents(db).text(db);
    let new_text = String::from_utf8_lossy(&output.stdout).into_owned();
    if new_text.is_empty() {
        None
    } else {
        let line_index = document.contents(db).line_index(db);
        Some(vec![TextEdit {
            range: line_index.line_col_lsp_range(TextRange::new(0.into(), old_text.text_len())),
            new_text,
        }])
    }
}

fn build_arguments(options: &LatexindentOptions, target_file: &Path) -> Vec<String> {
    let mut args = Vec::new();

    args.push(match &options.local {
        Some(yaml_file) => format!("--local={yaml_file}"),
        None => "--local".to_string(),
    });

    if options.modify_line_breaks {
        args.push("--modifylinebreaks".to_string());
    }

    args.push(target_file.display().to_string());
    args
}
