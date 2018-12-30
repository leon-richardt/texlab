package texlab.completion

import org.eclipse.lsp4j.CompletionItem
import texlab.BibtexDocument
import texlab.LatexDocument
import texlab.contains
import texlab.syntax.bibtex.*
import texlab.syntax.latex.LatexCommandSyntax
import texlab.syntax.latex.LatexDocumentSyntax
import texlab.syntax.latex.LatexGroupSyntax
import texlab.syntax.latex.LatexTextSyntax

class OrderByQualityProvider(private val provider: CompletionProvider) : CompletionProvider {
    override fun complete(request: CompletionRequest): List<CompletionItem> {
        val name = getName(request)
        return if (name == null) {
            listOf()
        } else {
            provider.complete(request)
                    .sortedByDescending { getQuality(it.label, name) }
        }
    }

    private fun getName(request: CompletionRequest): String? {
        return when (request.document) {
            is LatexDocument -> {
                val node = request.document.tree.root
                        .descendants()
                        .lastOrNull { it.range.contains(request.position) }

                when (node) {
                    is LatexGroupSyntax -> ""
                    is LatexCommandSyntax -> node.name.text.substring(1)
                    is LatexTextSyntax -> node.words[0].text
                    is LatexDocumentSyntax -> null
                    null -> null
                }
            }
            is BibtexDocument -> {
                val node = request.document.tree.root
                        .descendants()
                        .lastOrNull { it.range.contains(request.position) }

                when (node) {
                    is BibtexDocumentSyntax -> null
                    is BibtexDeclarationSyntax -> {
                        if (node.type.range.contains(request.position)) {
                            node.type.text.substring(1)
                        } else {
                            null
                        }
                    }
                    is BibtexCommentSyntax -> null
                    is BibtexFieldSyntax -> {
                        if (node.name.range.contains(request.position)) {
                            node.name.text
                        } else {
                            null
                        }
                    }
                    is BibtexWordSyntax -> {
                        node.token.text
                    }
                    is BibtexCommandSyntax -> {
                        node.token.text.substring(1)
                    }
                    is BibtexQuotedContentSyntax -> ""
                    is BibtexBracedContentSyntax -> ""
                    is BibtexConcatSyntax -> null
                    null -> null
                }
            }
        }
    }

    private fun getQuality(label: String, query: String): Int {
        if (label == query) {
            return 7
        }

        if (label.equals(query, ignoreCase = true)) {
            return 6
        }

        if (label.startsWith(query)) {
            return 5
        }

        if (label.toLowerCase().startsWith(query.toLowerCase())) {
            return 4
        }

        if (label.contains(query)) {
            return 3
        }

        if (label.toLowerCase().contains(query.toLowerCase())) {
            return 2
        }

        return 1
    }
}
