//! Extra utlities for untyped syntax nodes, syntax tokens, and AST nodes.

use crate::{AstNode, JsSyntaxKind, NodeOrToken, SyntaxElement, SyntaxNode, SyntaxToken};
use rome_rowan::Language;

/// Extensions to rowan's SyntaxNode
pub trait SyntaxNodeExt {
    #[doc(hidden)]
    fn to_node(&self) -> &SyntaxNode;

    /// Get all of the tokens of this node, recursively, including whitespace and comments.
    fn tokens(&self) -> Vec<SyntaxToken> {
        self.to_node()
            .descendants_with_tokens()
            .filter_map(|x| x.into_token())
            .collect()
    }

    /// Check if the node is a certain AST node and that it can be casted to it.
    fn is<T: AstNode>(&self) -> bool {
        T::can_cast(self.to_node().kind())
    }

    /// Cast this node to a certain AST node.
    ///
    /// # Panics
    /// Panics if the underlying node cannot be cast to the AST node
    fn to<T: AstNode>(&self) -> T {
        T::cast(self.to_node().to_owned()).unwrap_or_else(|| {
            panic!(
                "Tried to cast node {:?} as `{:?}` but was unable to cast",
                self.to_node(),
                std::any::type_name::<T>()
            )
        })
    }

    /// Try to cast this node to a certain AST node
    fn try_to<T: AstNode>(&self) -> Option<T> {
        T::cast(self.to_node().to_owned())
    }

    /// Go over the descendants of this node, at every descendant call `func`, and keep traversing
    /// the descendants of that node if the function's return is `true`. If the function returns false
    /// then stop traversing the descendants of that node go on to the next child.
    ///
    /// For example:
    /// ```ignore
    /// ROOT
    ///     CHILD // <-- Call `F` with CHILD, `F` returns `false` so go on to the next child...
    ///         SUBCHILD
    ///     CHILD // <-- Call `F` with CHILD, `F` return `true` so go on to the children of CHILD
    ///         SUBCHILD // <-- Same thing
    ///             SUBSUBCHILD
    ///     CHILD // Go on to the next child and do the same thing
    /// ```
    fn descendants_with<F>(&self, func: &mut F)
    where
        F: FnMut(&SyntaxNode) -> bool,
    {
        for node in self.to_node().children() {
            if func(&node) {
                node.descendants_with(func);
            }
        }
    }

    /// Whether the node contains any comments.
    fn contains_comments(&self) -> bool {
        self.tokens()
            .iter()
            .any(|tok| tok.has_trailing_comments() || tok.has_leading_comments())
    }

    fn has_skipped(&self) -> bool {
        self.to_node()
            .children_with_tokens()
            .filter_map(|child| child.into_token())
            .any(|token| {
                token
                    .leading_trivia()
                    .pieces()
                    .chain(token.trailing_trivia().pieces())
                    .any(|piece| piece.is_skipped())
            })
    }

    /// Whether the node contains trailing comments.
    fn has_trailing_comments(&self) -> bool {
        self.tokens()
            .last()
            .map_or(false, |tok| tok.has_trailing_comments())
    }

    /// Whether the node contains leading comments.
    fn has_leading_comments(&self) -> bool {
        self.tokens()
            .first()
            .map_or(false, |tok| tok.has_leading_comments())
    }

    /// Get the first child with a specific kind.
    fn child_with_kind(&self, kind: JsSyntaxKind) -> Option<SyntaxNode> {
        self.to_node().children().find(|child| child.kind() == kind)
    }

    /// Get the parent of this node, recursing through any grouping expressions
    fn expr_parent(&self) -> Option<SyntaxNode> {
        let parent = self.to_node().parent()?;
        if parent.kind() == JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
            parent.parent()
        } else {
            Some(parent)
        }
    }

    /// Get the first direct-child in this node that can be casted to an AST node
    fn child_with_ast<T: AstNode>(&self) -> Option<T> {
        self.to_node().children().find_map(|child| child.try_to())
    }

    /// Same as [`descendants_with`](Self::descendants_with) but considers tokens too.
    fn descendants_with_tokens_with<F>(&self, func: &mut F)
    where
        F: FnMut(&SyntaxElement) -> bool,
    {
        for elem in self.to_node().children_with_tokens() {
            match &elem {
                NodeOrToken::Node(node) => {
                    if func(&elem) {
                        node.descendants_with_tokens_with(func)
                    }
                }
                NodeOrToken::Token(_) => {
                    let _ = func(&elem);
                }
            }
        }
    }

    /// Get a specific token in the node which matches a syntax kind.
    ///
    /// This does not consider tokens in descendant nodes
    fn token_with_kind(&self, kind: JsSyntaxKind) -> Option<SyntaxToken> {
        self.to_node()
            .children_with_tokens()
            .find_map(|t| t.into_token().filter(|it| it.kind() == kind))
    }
}

impl SyntaxNodeExt for SyntaxNode {
    fn to_node(&self) -> &SyntaxNode {
        self
    }
}

pub trait SyntaxTokenExt {
    fn to_token(&self) -> &SyntaxToken;

    /// Convert a comment to a more detailed representation.
    fn comment(&self) -> Option<Comment> {
        if self.to_token().kind() != JsSyntaxKind::COMMENT {
            return None;
        }

        let token = self.to_token();
        let (kind, content) = match &token.text()[0..2] {
            "//" => (
                CommentKind::Inline,
                token
                    .text()
                    .get(2..)
                    .map(|x| x.to_string())
                    .unwrap_or_default(),
            ),
            "/*" if token.text().chars().nth(2) == Some('*') => {
                let len = token.text().len();
                let end = if token.text().get(len - 2..len) == Some("*/") {
                    len - 2
                } else {
                    len
                };
                (
                    CommentKind::JsDoc,
                    token
                        .text()
                        .get(3..end)
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                )
            }
            "/*" => {
                let len = token.text().len();
                let end = if token.text().get(len - 2..len) == Some("*/") {
                    len - 2
                } else {
                    len
                };
                (
                    CommentKind::JsDoc,
                    token
                        .text()
                        .get(3..end)
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                )
            }
            _ => return None,
        };
        Some(Comment {
            kind,
            content,
            token: self.to_token().clone(),
        })
    }
}

impl SyntaxTokenExt for SyntaxToken {
    fn to_token(&self) -> &SyntaxToken {
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    pub kind: CommentKind,
    pub content: String,
    pub token: SyntaxToken,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommentKind {
    /// A block comment which starts with `/**`
    JsDoc,
    /// A block comment which starts with `/*`
    Multiline,
    /// An inline comment which starts with `//`
    Inline,
}

/// Concatenate tokens into a string
pub fn concat_tokens(tokens: &[SyntaxToken]) -> String {
    tokens
        .iter()
        .map(|token| token.text().to_string())
        .collect()
}

/// Check whether a string contains a valid js linebreak consisting of any of these characters:
/// `\n`, `\r`, `\u{2028}`, or `\u{2029}`
pub fn contains_js_linebreak(string: impl AsRef<str>) -> bool {
    let text = string.as_ref();
    text.contains('\n')
        || text.contains('\r')
        || text.contains('\u{2028}')
        || text.contains('\u{2029}')
}

/// Check whether a string contains a valid js whitespace character
// FIXME: this should account for stuff in the Zs unicode category
pub fn contains_js_whitespace(string: impl AsRef<str>) -> bool {
    let text = string.as_ref();
    text.contains(' ')
        || text.contains('\u{000B}')
        || text.contains('\u{000C}')
        || text.contains('\u{0020}')
        || text.contains('\u{00A0}')
        || text.contains('\u{FEFF}')
}
