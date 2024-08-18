use crate::config::LanguageOptions;
use apollo_parser::{cst::*, SyntaxElement, SyntaxKind, SyntaxNode};
use rowan::Direction;
use tiny_pretty::Doc;

pub(super) struct Ctx<'a> {
    pub indent_width: usize,
    pub options: &'a LanguageOptions,
}

pub(super) trait DocGen {
    fn doc(&self, ctx: &Ctx) -> Doc<'static>;
}

impl DocGen for Definition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            Definition::OperationDefinition(node) => node.doc(ctx),
            Definition::FragmentDefinition(_) => todo!(),
            Definition::DirectiveDefinition(_) => todo!(),
            Definition::SchemaDefinition(_) => todo!(),
            Definition::ScalarTypeDefinition(_) => todo!(),
            Definition::ObjectTypeDefinition(_) => todo!(),
            Definition::InterfaceTypeDefinition(_) => todo!(),
            Definition::UnionTypeDefinition(_) => todo!(),
            Definition::EnumTypeDefinition(_) => todo!(),
            Definition::InputObjectTypeDefinition(_) => todo!(),
            Definition::SchemaExtension(_) => todo!(),
            Definition::ScalarTypeExtension(_) => todo!(),
            Definition::ObjectTypeExtension(_) => todo!(),
            Definition::InterfaceTypeExtension(_) => todo!(),
            Definition::UnionTypeExtension(_) => todo!(),
            Definition::EnumTypeExtension(_) => todo!(),
            Definition::InputObjectTypeExtension(_) => todo!(),
        }
    }
}

impl DocGen for Document {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = format_line_break_separated_list::<_, Definition, true>(self, ctx);
        docs.push(Doc::hard_line());
        Doc::list(docs)
    }
}

impl DocGen for ListType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(3);
        docs.push(Doc::text("["));
        if let Some(l_brack) = self.l_brack_token() {
            docs.append(&mut format_trivias_after_token(
                &SyntaxElement::Token(l_brack),
                ctx,
            ));
        }

        if let Some(ty) = self.ty() {
            docs.push(ty.doc(ctx));
            docs.append(&mut format_trivias_after_node(&ty, ctx));
        }

        docs.push(Doc::text("]"));

        Doc::list(docs)
    }
}

impl DocGen for Name {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        Doc::text(self.text().to_string())
    }
}

impl DocGen for NamedType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(name) = self.name() {
            name.doc(ctx)
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for NonNullType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(3);
        let mut trivias = vec![];
        if let Some(named_type) = self.named_type() {
            docs.push(named_type.doc(ctx));
            trivias.append(&mut format_trivias_after_node(&named_type, ctx));
        } else if let Some(list_type) = self.list_type() {
            docs.push(list_type.doc(ctx));
        }

        docs.push(Doc::text("!"));
        docs.append(&mut trivias);
        Doc::list(docs)
    }
}

impl DocGen for OperationDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        if let Some(operation_type) = self.operation_type() {
            docs.push(operation_type.doc(ctx));
            let mut trivias = format_trivias_after_node(&operation_type, ctx);
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
        }
        if let Some(name) = self.name() {
            docs.push(name.doc(ctx));
            let mut trivias = format_trivias_after_node(&name, ctx);
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
        }
        if let Some(variable_defs) = self.variable_definitions() {
            docs.push(variable_defs.doc(ctx));
            let mut trivias = format_trivias_after_node(&variable_defs, ctx);
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
        }
        if let Some(directives) = self.directives() {
            todo!()
        }
        if let Some(selection_set) = self.selection_set() {
            //
        }

        Doc::list(docs)
    }
}

impl DocGen for OperationType {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        if self.query_token().is_some() {
            Doc::text("query")
        } else if self.mutation_token().is_some() {
            Doc::text("mutation")
        } else if self.subscription_token().is_some() {
            Doc::text("subscription")
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for Type {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            Type::NamedType(node) => node.doc(ctx),
            Type::ListType(node) => node.doc(ctx),
            Type::NonNullType(node) => node.doc(ctx),
        }
    }
}

impl DocGen for Variable {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        Doc::text("$").append(
            self.name()
                .map(|name| name.doc(ctx))
                .unwrap_or_else(Doc::nil),
        )
    }
}

impl DocGen for VariableDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(6);
        if let Some(var) = self.variable() {
            docs.push(var.doc(ctx));
            docs.append(&mut format_trivias_after_node(&var, ctx));
        }
        if let Some(colon) = self.colon_token() {
            docs.push(Doc::text(":"));
            let mut trivias = format_trivias_after_token(&SyntaxElement::Token(colon), ctx);
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
        }
        if let Some(ty) = self.ty() {
            docs.push(ty.doc(ctx));
            let mut trivias = format_trivias_after_node(&ty, ctx);
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
        }
        if let Some(default_value) = self.default_value() {
            todo!();
        }
        if let Some(directives) = self.directives() {
            todo!();
        }

        Doc::list(docs)
    }
}

impl DocGen for VariableDefinitions {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);

        docs.push(Doc::text("("));
        let paren_space = Doc::line_or_nil();

        if let Some(l_paren) = self.l_paren_token() {
            if let Some(token) = l_paren
                .next_token()
                .filter(|token| token.kind() == SyntaxKind::WHITESPACE)
            {
                if token.text().contains(['\n', '\r']) {
                    docs.push(Doc::hard_line());
                } else {
                    docs.push(paren_space.clone());
                }
                let mut trivia_docs = format_trivias_after_token(&SyntaxElement::Token(token), ctx);
                docs.append(&mut trivia_docs);
            } else {
                docs.push(paren_space.clone());
                let mut trivia_docs =
                    format_trivias_after_token(&SyntaxElement::Token(l_paren), ctx);
                docs.append(&mut trivia_docs);
            }
        }

        docs.push(format_optional_comma_separated_list(
            self,
            self.variable_definitions(),
            ctx,
        ));

        Doc::list(docs)
            .nest(ctx.indent_width)
            .append(paren_space)
            .append(Doc::text(")"))
            .group()
    }
}

fn format_line_break_separated_list<N, Item, const SKIP_SIDE_WS: bool>(
    node: &N,
    ctx: &Ctx,
) -> Vec<Doc<'static>>
where
    N: CstNode,
    Item: CstNode + DocGen,
{
    let mut docs = Vec::with_capacity(2);

    let mut children = node.syntax().children_with_tokens().peekable();
    let mut prev_kind = SyntaxKind::WHITESPACE;
    while let Some(element) = children.next() {
        let kind = element.kind();
        match element {
            SyntaxElement::Node(node) => {
                if should_ignore(&node, ctx) {
                    reflow(&node.to_string(), &mut docs);
                } else if let Some(item) = Item::cast(node) {
                    docs.push(item.doc(ctx));
                }
            }
            SyntaxElement::Token(token) => match token.kind() {
                SyntaxKind::COMMENT => {
                    docs.push(format_comment(token.to_string(), ctx));
                }
                SyntaxKind::WHITESPACE => {
                    if !SKIP_SIDE_WS || token.index() > 0 && children.peek().is_some() {
                        match token.text().chars().filter(|c| *c == '\n').count() {
                            0 => {
                                if prev_kind == SyntaxKind::COMMENT {
                                    docs.push(Doc::hard_line());
                                } else {
                                    docs.push(Doc::space());
                                }
                            }
                            1 => {
                                docs.push(Doc::hard_line());
                            }
                            _ => {
                                docs.push(Doc::empty_line());
                                docs.push(Doc::hard_line());
                            }
                        }
                    }
                }
                _ => {}
            },
        }
        prev_kind = kind;
    }

    docs
}

fn format_optional_comma_separated_list<N, Entry>(
    node: &N,
    entries: CstChildren<Entry>,
    ctx: &Ctx,
) -> Doc<'static>
where
    N: CstNode,
    Entry: CstNode + DocGen,
{
    let node = node.syntax();
    let mut docs = vec![];
    let mut entries = entries.peekable();
    let mut commas = node
        .children_with_tokens()
        .filter_map(|element| match element {
            SyntaxElement::Token(token) if token.kind() == SyntaxKind::COMMA => Some(token),
            _ => None,
        });
    while let Some(entry) = entries.next() {
        docs.push(entry.doc(ctx));
        // TODO: comma

        let comma = commas.next();
        let mut has_comment_before_comma = false;
        let last_ws_index = comma
            .as_ref()
            .and_then(|comma| comma.prev_token())
            .filter(|token| token.kind() == SyntaxKind::WHITESPACE)
            .map(|token| token.index());
        let mut trivia_docs = format_trivias(
            entry
                .syntax()
                .siblings_with_tokens(Direction::Next)
                .filter(|token| {
                    last_ws_index
                        .map(|index| token.index() != index)
                        .unwrap_or(true)
                }),
            &mut has_comment_before_comma,
            ctx,
        );
        docs.append(&mut trivia_docs);

        if let Some(comma) = &comma {
            if entries.peek().is_some() {
                let mut trivia_docs = format_trivias(
                    comma.siblings_with_tokens(Direction::Next),
                    &mut has_comment_before_comma,
                    ctx,
                );
                if trivia_docs.is_empty() {
                    docs.push(Doc::hard_line());
                } else {
                    docs.append(&mut trivia_docs);
                }
            } else {
                let last_ws_index = node
                    .last_token()
                    .into_iter()
                    .flat_map(|token| token.siblings_with_tokens(Direction::Prev))
                    .find_map(|element| match element {
                        SyntaxElement::Token(token) if token.kind() == SyntaxKind::WHITESPACE => {
                            Some(token.index())
                        }
                        _ => None,
                    });
                let mut trivia_docs = format_trivias(
                    entry
                        .syntax()
                        .siblings_with_tokens(Direction::Next)
                        .filter(|token| {
                            last_ws_index
                                .map(|index| token.index() != index)
                                .unwrap_or(true)
                        }),
                    &mut has_comment_before_comma,
                    ctx,
                );
                if !trivia_docs.is_empty() {
                    docs.append(&mut trivia_docs);
                }
            }
        }
    }
    Doc::list(docs)
}

fn format_comment(text: String, ctx: &Ctx) -> Doc<'static> {
    if ctx.options.format_comments {
        let content = text.strip_prefix('#').expect("comment must start with '#'");
        if content.is_empty() || content.starts_with([' ', '\t']) {
            Doc::text(text.to_string())
        } else {
            Doc::text(format!("# {content}"))
        }
    } else {
        Doc::text(text)
    }
}

fn format_trivias_after_node<N: CstNode>(node: &N, ctx: &Ctx) -> Vec<Doc<'static>> {
    let mut _has_comment = false;
    format_trivias(
        node.syntax().siblings_with_tokens(Direction::Next),
        &mut _has_comment,
        ctx,
    )
}

fn format_trivias_after_token(element: &SyntaxElement, ctx: &Ctx) -> Vec<Doc<'static>> {
    let token = element.as_token().expect("expect rowan token");
    let mut _has_comment = false;
    format_trivias(
        token.siblings_with_tokens(Direction::Next),
        &mut _has_comment,
        ctx,
    )
}

fn format_trivias(
    it: impl Iterator<Item = SyntaxElement>,
    has_comment: &mut bool,
    ctx: &Ctx,
) -> Vec<Doc<'static>> {
    let mut docs = vec![];
    let mut trivias = it
        .skip(1)
        .skip_while(|element| element.kind() == SyntaxKind::WHITESPACE)
        .map_while(|element| match element {
            SyntaxElement::Token(token)
                if token.kind() == SyntaxKind::WHITESPACE
                    || token.kind() == SyntaxKind::COMMENT =>
            {
                Some(token)
            }
            _ => None,
        })
        .peekable();
    if trivias
        .peek()
        .is_some_and(|token| token.kind() == SyntaxKind::COMMENT)
    {
        docs.push(Doc::space());
    }

    while let Some(token) = trivias.next() {
        match token.kind() {
            SyntaxKind::WHITESPACE => match token.text().chars().filter(|c| *c == '\n').count() {
                0 => {
                    if *has_comment {
                        docs.push(Doc::hard_line());
                    } else if trivias
                        .peek()
                        .is_some_and(|token| token.kind() == SyntaxKind::COMMENT)
                    {
                        docs.push(Doc::space());
                    } else {
                        docs.push(Doc::line_or_space());
                    }
                }
                1 => {
                    if *has_comment {
                        docs.push(Doc::hard_line());
                    } else {
                        docs.push(Doc::line_or_space());
                    }
                }
                _ => {
                    docs.push(Doc::empty_line());
                    docs.push(Doc::hard_line());
                }
            },
            SyntaxKind::COMMENT => {
                docs.push(format_comment(token.to_string(), ctx));
                *has_comment = true;
            }
            _ => {}
        }
    }
    docs
}

fn reflow(text: &str, docs: &mut Vec<Doc<'static>>) {
    let mut lines = text.lines();
    if let Some(line) = lines.next() {
        docs.push(Doc::text(line.to_owned()));
    }
    for line in lines {
        docs.push(Doc::empty_line());
        docs.push(Doc::text(line.to_owned()));
    }
}

fn should_ignore(node: &SyntaxNode, ctx: &Ctx) -> bool {
    // for the case that comment comes in the middle of a list of nodes
    node.prev_sibling_or_token()
        .and_then(|element| element.prev_sibling_or_token())
        .or_else(|| {
            // for the case that comment comes at the start or the end of a list of nodes
            node.parent()
                .and_then(|parent| parent.prev_sibling_or_token())
                .and_then(|parent| parent.prev_sibling_or_token())
        })
        .as_ref()
        .and_then(|element| match element {
            SyntaxElement::Token(token) if token.kind() == SyntaxKind::COMMENT => {
                token.text().strip_prefix('#').and_then(|s| {
                    s.trim_start()
                        .strip_prefix(&ctx.options.ignore_comment_directive)
                })
            }
            _ => None,
        })
        .is_some_and(|rest| rest.is_empty() || rest.starts_with(|c: char| c.is_ascii_whitespace()))
}
