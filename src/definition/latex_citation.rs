use crate::feature::FeatureRequest;
use crate::range;
use crate::syntax::bibtex::ast::BibtexDeclaration;
use crate::syntax::latex::{LatexCitationAnalyzer, LatexVisitor};
use crate::syntax::text::SyntaxNode;
use crate::workspace::{Document, SyntaxTree};
use lsp_types::{Location, TextDocumentPositionParams};

pub struct LatexCitationDefinitionProvider;

impl LatexCitationDefinitionProvider {
    pub async fn execute(request: &FeatureRequest<TextDocumentPositionParams>) -> Vec<Location> {
        if let Some(reference) = Self::find_reference(&request) {
            for document in &request.related_documents {
                if let Some(definition) = Self::find_definition(&document, &reference) {
                    return vec![definition];
                }
            }
        }
        Vec::new()
    }

    fn find_definition(document: &Document, reference: &str) -> Option<Location> {
        if let SyntaxTree::Bibtex(tree) = &document.tree {
            for declaration in &tree.root.children {
                if let BibtexDeclaration::Entry(entry) = declaration {
                    if let Some(key) = &entry.key {
                        if key.text() == reference {
                            return Some(Location::new(document.uri.clone(), key.range()));
                        }
                    }
                }
            }
        }
        None
    }

    fn find_reference(request: &FeatureRequest<TextDocumentPositionParams>) -> Option<&str> {
        if let SyntaxTree::Latex(tree) = &request.document.tree {
            let mut analyzer = LatexCitationAnalyzer::new();
            analyzer.visit_root(&tree.root);
            analyzer
                .citations
                .iter()
                .find(|citation| range::contains(citation.key.range(), request.params.position))
                .map(|citation| citation.key.text())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::completion::latex::data::types::LatexComponentDatabase;
    use crate::feature::FeatureSpec;
    use crate::test_feature;
    use lsp_types::Position;

    #[test]
    fn test_has_definition() {
        let locations = test_feature!(
            LatexCitationDefinitionProvider,
            FeatureSpec {
                files: vec![
                    FeatureSpec::file("foo.tex", "\\addbibresource{baz.bib}\n\\cite{foo}"),
                    FeatureSpec::file("bar.bib", "@article{foo, bar = {baz}}"),
                    FeatureSpec::file("baz.bib", "@article{foo, bar = {baz}}"),
                ],
                main_file: "foo.tex",
                position: Position::new(1, 6),
                new_name: "",
                component_database: LatexComponentDatabase::default(),
            }
        );
        assert_eq!(
            locations,
            vec![Location::new(
                FeatureSpec::uri("baz.bib"),
                range::create(0, 9, 0, 12)
            )]
        );
    }

    #[test]
    fn test_no_definition_latex() {
        let locations = test_feature!(
            LatexCitationDefinitionProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.tex", ""),],
                main_file: "foo.tex",
                position: Position::new(0, 0),
                new_name: "",
                component_database: LatexComponentDatabase::default(),
            }
        );
        assert_eq!(locations, Vec::new());
    }

    #[test]
    fn test_no_definition_bibtex() {
        let locations = test_feature!(
            LatexCitationDefinitionProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.bib", ""),],
                main_file: "foo.bib",
                position: Position::new(0, 0),
                new_name: "",
                component_database: LatexComponentDatabase::default(),
            }
        );
        assert_eq!(locations, Vec::new());
    }
}
