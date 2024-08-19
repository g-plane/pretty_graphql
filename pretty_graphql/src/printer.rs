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

impl DocGen for Alias {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(3);
        let mut trivias = vec![];
        if let Some(name) = self.name() {
            docs.push(name.doc(ctx));
            trivias.append(&mut format_trivias_after_node(&name, ctx));
        }

        docs.push(Doc::text(":"));
        docs.append(&mut trivias);
        Doc::list(docs)
    }
}

impl DocGen for Argument {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(4);
        let mut trivias = vec![];
        if let Some(name) = self.name() {
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(colon) = self.colon_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text(":"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(colon), ctx);
        }
        if let Some(value) = self.value() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(value.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for Arguments {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("()")
        } else {
            format_delimiters(
                format_optional_comma_separated_list(
                    self,
                    self.arguments(),
                    Doc::line_or_space(),
                    ctx,
                ),
                ("(", ")"),
                Doc::line_or_nil(),
                (
                    self.l_paren_token().map(SyntaxElement::Token),
                    self.r_paren_token().map(SyntaxElement::Token),
                ),
                ctx,
            )
            .group()
        }
    }
}

impl DocGen for ArgumentsDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("()")
        } else {
            format_delimiters(
                format_optional_comma_separated_list(
                    self,
                    self.input_value_definitions(),
                    Doc::line_or_space(),
                    ctx,
                ),
                ("(", ")"),
                Doc::line_or_nil(),
                (
                    self.l_paren_token().map(SyntaxElement::Token),
                    self.r_paren_token().map(SyntaxElement::Token),
                ),
                ctx,
            )
            .group()
        }
    }
}

impl DocGen for BooleanValue {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        Doc::text(self.source_string())
    }
}

impl DocGen for DefaultValue {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(3);
        docs.push(Doc::text("="));
        let mut trivias = if let Some(eq) = self.eq_token() {
            format_trivias_after_token(&SyntaxElement::Token(eq), ctx)
        } else {
            vec![]
        };

        if let Some(default_value) = self.value() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(default_value.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for Definition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            Definition::OperationDefinition(node) => node.doc(ctx),
            Definition::FragmentDefinition(node) => node.doc(ctx),
            Definition::DirectiveDefinition(node) => node.doc(ctx),
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

impl DocGen for Description {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(string) = self.string_value() {
            string.doc(ctx)
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for Directive {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(4);

        docs.push(Doc::text("@"));
        if let Some(at) = self.at_token() {
            docs.append(&mut format_trivias_after_token(
                &SyntaxElement::Token(at),
                ctx,
            ));
        }

        if let Some(name) = self.name() {
            docs.push(name.doc(ctx));
            docs.append(&mut format_trivias_after_node(&name, ctx));
        }

        if let Some(arguments) = self.arguments() {
            docs.push(arguments.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for DirectiveDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(description) = self.description() {
            docs.push(description.doc(ctx));
            trivias = format_trivias_after_node(&description, ctx);
        }
        if let Some(directive) = self.directive_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("directive"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(directive), ctx);
        }
        if self.at_token().is_some() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(Doc::text("@"));
        }
        if let Some(name) = self.name() {
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(arguments_def) = self.arguments_definition() {
            docs.append(&mut trivias);
            docs.push(arguments_def.doc(ctx));
            trivias = format_trivias_after_node(&arguments_def, ctx);
        }
        if let Some(repeatable) = self.repeatable_token() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(Doc::text("repeatable"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(repeatable), ctx);
        }
        if let Some(on) = self.on_token() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(Doc::text("on"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(on), ctx);
        }
        if let Some(directive_locations) = self.directive_locations() {
            if trivias.is_empty() {
                docs.push(
                    Doc::line_or_space()
                        .append(directive_locations.doc(ctx))
                        .group()
                        .nest(ctx.indent_width),
                );
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directive_locations.doc(ctx).nest(ctx.indent_width));
            }
        }

        Doc::list(docs)
    }
}

impl DocGen for DirectiveLocation {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        Doc::text(self.source_string())
    }
}

impl DocGen for DirectiveLocations {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_union(self, self.directive_locations(), ctx).group()
    }
}

impl DocGen for Directives {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_optional_comma_separated_list(self, self.directives(), Doc::line_or_space(), ctx)
    }
}

impl DocGen for Document {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = format_line_break_separated_list::<_, Definition, true>(self, ctx);
        docs.push(Doc::hard_line());
        Doc::list(docs)
    }
}

impl DocGen for EnumValue {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(name) = self.name() {
            name.doc(ctx)
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for Field {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(alias) = self.alias() {
            docs.push(alias.doc(ctx));
            trivias = format_trivias_after_node(&alias, ctx);
        }
        if let Some(name) = self.name() {
            if trivias.is_empty() && !docs.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(arguments) = self.arguments() {
            docs.append(&mut trivias);
            docs.push(arguments.doc(ctx));
            trivias = format_trivias_after_node(&arguments, ctx);
        }
        if let Some(directives) = self.directives() {
            if !trivias.is_empty() {
                docs.append(&mut trivias);
            }
            docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(selection_set) = self.selection_set() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(selection_set.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for FloatValue {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        Doc::text(self.source_string())
    }
}

impl DocGen for FragmentDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(fragment) = self.fragment_token() {
            docs.push(Doc::text("fragment"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(fragment), ctx);
        }
        if let Some(fragment_name) = self.fragment_name() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(fragment_name.doc(ctx));
            trivias = format_trivias_after_node(&fragment_name, ctx);
        }
        if let Some(type_condition) = self.type_condition() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(type_condition.doc(ctx));
            trivias = format_trivias_after_node(&type_condition, ctx);
        }
        if let Some(directives) = self.directives() {
            if !trivias.is_empty() {
                docs.append(&mut trivias);
            }
            docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(selection_set) = self.selection_set() {
            if trivias.is_empty() && !docs.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(selection_set.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for FragmentName {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if let Some(name) = self.name() {
            name.doc(ctx)
        } else {
            Doc::nil()
        }
    }
}

impl DocGen for FragmentSpread {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(dotdotdot) = self.dotdotdot_token() {
            docs.push(Doc::text("..."));
            trivias = format_trivias_after_token(&SyntaxElement::Token(dotdotdot), ctx);
        }
        if let Some(fragment_name) = self.fragment_name() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(fragment_name.doc(ctx));
            trivias = format_trivias_after_node(&fragment_name, ctx);
        }
        if let Some(directives) = self.directives() {
            if !trivias.is_empty() {
                docs.append(&mut trivias);
            }
            docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
        }

        Doc::list(docs)
    }
}

impl DocGen for InlineFragment {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];

        docs.push(Doc::text("..."));
        if let Some(dotdotdot) = self.dotdotdot_token() {
            trivias = format_trivias_after_token(&SyntaxElement::Token(dotdotdot), ctx);
        }
        if let Some(type_condition) = self.type_condition() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(type_condition.doc(ctx));
        }
        if let Some(directives) = self.directives() {
            if !trivias.is_empty() {
                docs.append(&mut trivias);
            }
            docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
        }
        if let Some(selection_set) = self.selection_set() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(selection_set.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for InputValueDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(description) = self.description() {
            docs.push(description.doc(ctx));
            trivias = format_trivias_after_node(&description, ctx);
        }
        if let Some(name) = self.name() {
            if trivias.is_empty() && !docs.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(colon) = self.colon_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text(":"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(colon), ctx);
        }
        if let Some(ty) = self.ty() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(ty.doc(ctx));
            trivias = format_trivias_after_node(&ty, ctx);
        }
        if let Some(default_value) = self.default_value() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(default_value.doc(ctx));
            trivias = format_trivias_after_node(&default_value, ctx);
        }
        if let Some(directives) = self.directives() {
            if !trivias.is_empty() {
                docs.append(&mut trivias);
            }
            docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
        }

        Doc::list(docs)
    }
}

impl DocGen for IntValue {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        Doc::text(self.source_string())
    }
}

impl DocGen for ListType {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_delimiters(
            self.ty().map(|ty| ty.doc(ctx)).unwrap_or_else(Doc::nil),
            ("[", "]"),
            Doc::line_or_nil(),
            (
                self.l_brack_token().map(SyntaxElement::Token),
                self.r_brack_token().map(SyntaxElement::Token),
            ),
            ctx,
        )
    }
}

impl DocGen for ListValue {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("[]")
        } else {
            format_delimiters(
                format_optional_comma_separated_list(
                    self,
                    self.values(),
                    Doc::line_or_space(),
                    ctx,
                ),
                ("[", "]"),
                Doc::line_or_nil(),
                (
                    self.l_brack_token().map(SyntaxElement::Token),
                    self.r_brack_token().map(SyntaxElement::Token),
                ),
                ctx,
            )
            .group()
        }
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

impl DocGen for NullValue {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        Doc::text("null")
    }
}

impl DocGen for ObjectField {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(4);
        let mut trivias = vec![];
        if let Some(name) = self.name() {
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(colon) = self.colon_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text(":"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(colon), ctx);
        }
        if let Some(value) = self.value() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(value.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for ObjectValue {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("{}")
        } else {
            format_delimiters(
                format_optional_comma_separated_list(
                    self,
                    self.object_fields(),
                    Doc::line_or_space(),
                    ctx,
                ),
                ("{", "}"),
                Doc::line_or_space(),
                (
                    self.l_curly_token().map(SyntaxElement::Token),
                    self.r_curly_token().map(SyntaxElement::Token),
                ),
                ctx,
            )
            .group()
        }
    }
}

impl DocGen for OperationDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(operation_type) = self.operation_type() {
            docs.push(operation_type.doc(ctx));
            trivias = format_trivias_after_node(&operation_type, ctx);
        }
        if let Some(name) = self.name() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(variable_defs) = self.variable_definitions() {
            if !trivias.is_empty() {
                docs.append(&mut trivias);
            }
            docs.push(variable_defs.doc(ctx));
            trivias = format_trivias_after_node(&variable_defs, ctx);
        }
        if let Some(directives) = self.directives() {
            if !trivias.is_empty() {
                docs.append(&mut trivias);
            }
            docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(selection_set) = self.selection_set() {
            if trivias.is_empty() && !docs.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(selection_set.doc(ctx));
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

impl DocGen for Selection {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            Selection::Field(node) => node.doc(ctx),
            Selection::FragmentSpread(node) => node.doc(ctx),
            Selection::InlineFragment(node) => node.doc(ctx),
        }
    }
}

impl DocGen for SelectionSet {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_delimiters(
            format_optional_comma_separated_list(self, self.selections(), Doc::hard_line(), ctx),
            ("{", "}"),
            Doc::line_or_space(),
            (
                self.l_curly_token().map(SyntaxElement::Token),
                self.r_curly_token().map(SyntaxElement::Token),
            ),
            ctx,
        )
        .group()
    }
}

impl DocGen for StringValue {
    fn doc(&self, _: &Ctx) -> Doc<'static> {
        Doc::text(self.source_string())
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

impl DocGen for TypeCondition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(3);
        let mut trivias = vec![];
        docs.push(Doc::text("on"));
        if let Some(on) = self.on_token() {
            trivias = format_trivias_after_token(&SyntaxElement::Token(on), ctx)
        }
        if let Some(named_type) = self.named_type() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(named_type.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for Value {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        match self {
            Value::Variable(node) => node.doc(ctx),
            Value::StringValue(node) => node.doc(ctx),
            Value::FloatValue(node) => node.doc(ctx),
            Value::IntValue(node) => node.doc(ctx),
            Value::BooleanValue(node) => node.doc(ctx),
            Value::NullValue(node) => node.doc(ctx),
            Value::EnumValue(node) => node.doc(ctx),
            Value::ListValue(node) => node.doc(ctx),
            Value::ObjectValue(node) => node.doc(ctx),
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
        let mut trivias = vec![];
        if let Some(var) = self.variable() {
            docs.push(var.doc(ctx));
            trivias = format_trivias_after_node(&var, ctx);
        }
        if let Some(colon) = self.colon_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text(":"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(colon), ctx);
        }
        if let Some(ty) = self.ty() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(ty.doc(ctx));
            trivias = format_trivias_after_node(&ty, ctx);
        }
        if let Some(default_value) = self.default_value() {
            if trivias.is_empty() {
                docs.push(Doc::space());
            } else {
                docs.append(&mut trivias);
            }
            docs.push(default_value.doc(ctx));
            trivias = format_trivias_after_node(&default_value, ctx);
        }
        if let Some(directives) = self.directives() {
            if !trivias.is_empty() {
                docs.append(&mut trivias);
            }
            docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
        }

        Doc::list(docs)
    }
}

impl DocGen for VariableDefinitions {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("()")
        } else {
            format_delimiters(
                format_optional_comma_separated_list(
                    self,
                    self.variable_definitions(),
                    Doc::line_or_space(),
                    ctx,
                ),
                ("(", ")"),
                Doc::line_or_nil(),
                (
                    self.l_paren_token().map(SyntaxElement::Token),
                    self.r_paren_token().map(SyntaxElement::Token),
                ),
                ctx,
            )
            .group()
        }
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
    separator: Doc<'static>,
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
        let mut has_last_line_break = false;
        if let Some(next) = entries.peek() {
            let last_ws_index = comma
                .as_ref()
                .and_then(|comma| comma.prev_token())
                .or_else(|| {
                    next.syntax()
                        .prev_sibling_or_token()
                        .and_then(|element| element.into_token())
                })
                .filter(|token| {
                    if token.kind() == SyntaxKind::WHITESPACE {
                        has_last_line_break =
                            token.text().chars().filter(|c| *c == '\n').count() > 1;
                        !has_last_line_break
                    } else {
                        false
                    }
                })
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
                false,
                ctx,
            );
            docs.append(&mut trivia_docs);
            if has_comment_before_comma && !has_last_line_break {
                docs.push(Doc::hard_line());
            }
        }

        if let Some(comma) = &comma {
            if entries.peek().is_some() {
                let mut trivia_docs = format_trivias(
                    comma.siblings_with_tokens(Direction::Next),
                    &mut has_comment_before_comma,
                    false,
                    ctx,
                );
                if trivia_docs.is_empty() {
                    docs.push(separator.clone());
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
                    false,
                    ctx,
                );
                if !trivia_docs.is_empty() {
                    docs.append(&mut trivia_docs);
                }
            }
        } else if entries.peek().is_some() && !has_comment_before_comma && !has_last_line_break {
            docs.push(separator.clone());
        }
    }
    Doc::list(docs)
}

fn format_union<N, Entry>(node: &N, mut entries: CstChildren<Entry>, ctx: &Ctx) -> Doc<'static>
where
    N: CstNode,
    Entry: CstNode + DocGen,
{
    let node = node.syntax();
    let pipes = node
        .children_with_tokens()
        .filter_map(|element| match element {
            SyntaxElement::Token(token) if token.kind() == SyntaxKind::PIPE => Some(token),
            _ => None,
        });
    let mut docs = Vec::with_capacity(4);

    if node
        .first_token()
        .is_some_and(|token| token.kind() != SyntaxKind::PIPE)
    {
        if let Some(first) = entries.next() {
            docs.push(Doc::flat_or_break(Doc::nil(), Doc::text("| ")));
            docs.push(first.doc(ctx));
            let mut trivias = format_trivias_after_node(&first, ctx);
            if trivias.is_empty() {
                docs.push(Doc::line_or_space());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
            }
        }
    }

    let mut it = entries.zip(pipes).peekable();
    while let Some((entry, pipe)) = it.next() {
        docs.push(Doc::text("| "));
        docs.push(entry.doc(ctx));
        if it.peek().is_some() {
            let mut trivias_after_pipe =
                format_trivias_after_token(&SyntaxElement::Token(pipe), ctx);
            let mut trivias_after_node = format_trivias_after_node(&entry, ctx);
            if trivias_after_pipe.is_empty() && trivias_after_node.is_empty() {
                docs.push(Doc::line_or_space());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias_after_pipe);
                docs.append(&mut trivias_after_node);
            }
        }
    }

    Doc::list(docs)
}

fn format_delimiters(
    body: Doc<'static>,
    delim_text: (&'static str, &'static str),
    space: Doc<'static>,
    delim_token: (Option<SyntaxElement>, Option<SyntaxElement>),
    ctx: &Ctx,
) -> Doc<'static> {
    let mut docs = Vec::with_capacity(5);

    docs.push(Doc::text(delim_text.0));

    if let Some(open) = delim_token.0.and_then(|open| open.into_token()) {
        if let Some(token) = open
            .next_token()
            .filter(|token| token.kind() == SyntaxKind::WHITESPACE)
        {
            if token.text().contains(['\n', '\r']) {
                docs.push(Doc::hard_line());
            } else {
                docs.push(space.clone());
            }
            let mut trivia_docs = format_trivias_after_token(&SyntaxElement::Token(token), ctx);
            docs.append(&mut trivia_docs);
        } else {
            docs.push(space.clone());
            let mut trivia_docs = format_trivias_after_token(&SyntaxElement::Token(open), ctx);
            docs.append(&mut trivia_docs);
        }
    }

    docs.push(body);

    let mut has_comment = false;
    if let Some(close) = delim_token.1.and_then(|close| close.into_token()) {
        let last_ws_index = close
            .prev_token()
            .filter(|token| token.kind() == SyntaxKind::WHITESPACE)
            .map(|token| token.index());
        let last_non_trivia = close
            .siblings_with_tokens(Direction::Prev)
            .skip(1)
            .find(|element| {
                !matches!(element.kind(), SyntaxKind::WHITESPACE | SyntaxKind::COMMENT)
            });
        let mut trivias = match last_non_trivia {
            Some(SyntaxElement::Node(node)) => format_trivias(
                node.siblings_with_tokens(Direction::Next).filter(|token| {
                    last_ws_index
                        .map(|index| token.index() != index)
                        .unwrap_or(true)
                }),
                &mut has_comment,
                false,
                ctx,
            ),
            Some(SyntaxElement::Token(token)) => format_trivias(
                token.siblings_with_tokens(Direction::Next).filter(|token| {
                    last_ws_index
                        .map(|index| token.index() != index)
                        .unwrap_or(true)
                }),
                &mut has_comment,
                false,
                ctx,
            ),
            None => vec![],
        };
        docs.append(&mut trivias);
    }

    Doc::list(docs)
        .nest(ctx.indent_width)
        .append(if has_comment { Doc::hard_line() } else { space })
        .append(Doc::text(delim_text.1))
        .group()
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
        true,
        ctx,
    )
}

fn format_trivias_after_token(element: &SyntaxElement, ctx: &Ctx) -> Vec<Doc<'static>> {
    let token = element.as_token().expect("expect rowan token");
    let mut _has_comment = false;
    format_trivias(
        token.siblings_with_tokens(Direction::Next),
        &mut _has_comment,
        true,
        ctx,
    )
}

fn format_trivias(
    it: impl Iterator<Item = SyntaxElement>,
    has_comment: &mut bool,
    skip_first_ws: bool,
    ctx: &Ctx,
) -> Vec<Doc<'static>> {
    let mut docs = vec![];
    let mut trivias = it
        .skip(1)
        .skip_while(|element| skip_first_ws && element.kind() == SyntaxKind::WHITESPACE)
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
    if !skip_first_ws
        && trivias
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

fn is_empty_delimiter<N: CstNode>(node: &N) -> bool {
    node.syntax()
        .children_with_tokens()
        .all(|element| element.kind() != SyntaxKind::COMMENT && element.as_node().is_none())
}
