mod citation;
mod component;
mod entry_type;
mod field;
mod label;
mod string_ref;

use lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Url};
use rowan::TextRange;

use crate::{
    util::{cursor::CursorContext, line_index_ext::LineIndexExt},
    Db,
};

pub fn find(db: &dyn Db, uri: &Url, position: Position) -> Option<Hover> {
    let context = CursorContext::new(db, uri, position, ())?;
    log::debug!("[Hover] Cursor: {:?}", context.cursor);

    let result = label::find_hover(&context)
        .or_else(|| citation::find_hover(&context))
        .or_else(|| component::find_hover(&context))
        .or_else(|| string_ref::find_hover(&context))
        .or_else(|| field::find_hover(&context))
        .or_else(|| entry_type::find_hover(&context))?;

    let line_index = context.document.contents(db).line_index(db);
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: result.value_kind,
            value: result.value,
        }),
        range: Some(line_index.line_col_lsp_range(result.range)),
    })
}

#[derive(Debug, Clone)]
struct HoverResult {
    range: TextRange,
    value: String,
    value_kind: MarkupKind,
}
