use crate::config::{Comma, LanguageOptions, SingleLine};
use apollo_parser::{cst::*, SyntaxElement, SyntaxKind, SyntaxNode, S};
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
            docs.push(Doc::space());
            docs.append(&mut trivias);
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
            DelimitersFormatter::paren(
                self.l_paren_token().map(SyntaxElement::Token),
                self.r_paren_token().map(SyntaxElement::Token),
                ctx.options.arguments_paren_spacing,
                ctx,
            )
            .with_single_line(ctx.options.arguments_single_line.as_ref())
            .format(format_optional_comma_separated_list(
                self,
                self.arguments(),
                Doc::line_or_space(),
                ctx.options.arguments_comma.as_ref(),
                ctx,
            ))
        }
    }
}

impl DocGen for ArgumentsDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("()")
        } else {
            DelimitersFormatter::paren(
                self.l_paren_token().map(SyntaxElement::Token),
                self.r_paren_token().map(SyntaxElement::Token),
                ctx.options.arguments_definition_paren_spacing,
                ctx,
            )
            .with_single_line(ctx.options.arguments_definition_single_line.as_ref())
            .format(format_optional_comma_separated_list(
                self,
                self.input_value_definitions(),
                Doc::line_or_space(),
                ctx.options.arguments_definition_comma.as_ref(),
                ctx,
            ))
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
            docs.push(Doc::space());
            docs.append(&mut trivias);
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
            Definition::SchemaDefinition(node) => node.doc(ctx),
            Definition::ScalarTypeDefinition(node) => node.doc(ctx),
            Definition::ObjectTypeDefinition(node) => node.doc(ctx),
            Definition::InterfaceTypeDefinition(node) => node.doc(ctx),
            Definition::UnionTypeDefinition(node) => node.doc(ctx),
            Definition::EnumTypeDefinition(node) => node.doc(ctx),
            Definition::InputObjectTypeDefinition(node) => node.doc(ctx),
            Definition::SchemaExtension(node) => node.doc(ctx),
            Definition::ScalarTypeExtension(node) => node.doc(ctx),
            Definition::ObjectTypeExtension(node) => node.doc(ctx),
            Definition::InterfaceTypeExtension(node) => node.doc(ctx),
            Definition::UnionTypeExtension(node) => node.doc(ctx),
            Definition::EnumTypeExtension(node) => node.doc(ctx),
            Definition::InputObjectTypeExtension(node) => node.doc(ctx),
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
        format_union_like(self, self.directive_locations(), S![|], "|", ctx).group()
    }
}

impl DocGen for Directives {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        format_optional_comma_separated_list(
            self,
            self.directives(),
            Doc::line_or_space(),
            ctx.options.directives_comma.as_ref(),
            ctx,
        )
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

impl DocGen for EnumTypeDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(description) = self.description() {
            docs.push(description.doc(ctx));
            trivias = format_trivias_after_node(&description, ctx);
        }
        if let Some(r#enum) = self.enum_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("enum"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(r#enum), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(enum_values_def) = self.enum_values_definition() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(enum_values_def.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for EnumTypeExtension {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(extend) = self.extend_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text("extend"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(extend), ctx);
        }
        if let Some(r#enum) = self.enum_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("enum"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(r#enum), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(enum_values_def) = self.enum_values_definition() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(enum_values_def.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for EnumValueDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(description) = self.description() {
            docs.push(description.doc(ctx));
            trivias = format_trivias_after_node(&description, ctx);
        }
        if let Some(enum_value) = self.enum_value() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(enum_value.doc(ctx));
            trivias = format_trivias_after_node(&enum_value, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
        }

        Doc::list(docs)
    }
}

impl DocGen for EnumValuesDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("{}")
        } else {
            DelimitersFormatter::brace(
                self.l_curly_token().map(SyntaxElement::Token),
                self.r_curly_token().map(SyntaxElement::Token),
                ctx.options.enum_values_definition_brace_spacing,
                ctx,
            )
            .with_single_line(ctx.options.enum_values_definition_single_line.as_ref())
            .format(format_optional_comma_separated_list(
                self,
                self.enum_value_definitions(),
                Doc::hard_line(),
                ctx.options.enum_values_definition_comma.as_ref(),
                ctx,
            ))
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
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(arguments) = self.arguments() {
            docs.append(&mut trivias);
            docs.push(arguments.doc(ctx));
            trivias = format_trivias_after_node(&arguments, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(selection_set) = self.selection_set() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(selection_set.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for FieldDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(description) = self.description() {
            docs.push(description.doc(ctx));
            trivias = format_trivias_after_node(&description, ctx);
        }
        if let Some(name) = self.name() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(arguments_def) = self.arguments_definition() {
            docs.append(&mut trivias);
            docs.push(arguments_def.doc(ctx));
            trivias = format_trivias_after_node(&arguments_def, ctx);
        }
        if let Some(colon) = self.colon_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text(":"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(colon), ctx);
        }
        if let Some(ty) = self.ty() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(ty.doc(ctx));
            trivias = format_trivias_after_node(&ty, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
        }

        Doc::list(docs)
    }
}

impl DocGen for FieldsDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("{}")
        } else {
            DelimitersFormatter::brace(
                self.l_curly_token().map(SyntaxElement::Token),
                self.r_curly_token().map(SyntaxElement::Token),
                ctx.options.fields_definition_brace_spacing,
                ctx,
            )
            .with_single_line(ctx.options.fields_definition_single_line.as_ref())
            .format(format_optional_comma_separated_list(
                self,
                self.field_definitions(),
                Doc::hard_line(),
                ctx.options.fields_definition_comma.as_ref(),
                ctx,
            ))
        }
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
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(fragment_name.doc(ctx));
            trivias = format_trivias_after_node(&fragment_name, ctx);
        }
        if let Some(type_condition) = self.type_condition() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
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

impl DocGen for ImplementsInterfaces {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(3);
        let mut trivias = vec![];
        if let Some(implements) = self.implements_token() {
            docs.push(Doc::text("implements"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(implements), ctx);
        }
        if self.named_types().count() > 0 {
            let types_doc = format_union_like(self, self.named_types(), S![&], "&", ctx);
            if trivias.is_empty() {
                docs.push(
                    Doc::line_or_space()
                        .append(types_doc)
                        .group()
                        .nest(ctx.indent_width),
                );
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(types_doc.group().nest(ctx.indent_width));
            }
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
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(selection_set) = self.selection_set() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(selection_set.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for InputFieldsDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("{}")
        } else {
            DelimitersFormatter::brace(
                self.l_curly_token().map(SyntaxElement::Token),
                self.r_curly_token().map(SyntaxElement::Token),
                ctx.options.input_fields_definition_brace_spacing,
                ctx,
            )
            .with_single_line(ctx.options.input_fields_definition_single_line.as_ref())
            .format(format_optional_comma_separated_list(
                self,
                self.input_value_definitions(),
                Doc::hard_line(),
                ctx.options.input_fields_definition_comma.as_ref(),
                ctx,
            ))
        }
    }
}

impl DocGen for InputObjectTypeDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(description) = self.description() {
            docs.push(description.doc(ctx));
            trivias = format_trivias_after_node(&description, ctx);
        }
        if let Some(input) = self.input_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("input"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(input), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(input_fields_def) = self.input_fields_definition() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(input_fields_def.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for InputObjectTypeExtension {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(extend) = self.extend_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text("extend"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(extend), ctx);
        }
        if let Some(input) = self.input_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("input"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(input), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(input_fields_def) = self.input_fields_definition() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(input_fields_def.doc(ctx));
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
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(colon) = self.colon_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text(":"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(colon), ctx);
        }
        if let Some(ty) = self.ty() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(ty.doc(ctx));
            trivias = format_trivias_after_node(&ty, ctx);
        }
        if let Some(default_value) = self.default_value() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(default_value.doc(ctx));
            trivias = format_trivias_after_node(&default_value, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
        }

        Doc::list(docs)
    }
}

impl DocGen for InterfaceTypeDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(description) = self.description() {
            docs.push(description.doc(ctx));
            trivias = format_trivias_after_node(&description, ctx);
        }
        if let Some(interface) = self.interface_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("interface"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(interface), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(interfaces) = self.implements_interfaces() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(interfaces.doc(ctx));
            trivias = format_trivias_after_node(&interfaces, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(fields_def) = self.fields_definition() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(fields_def.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for InterfaceTypeExtension {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(extend) = self.extend_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text("extend"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(extend), ctx);
        }
        if let Some(interface) = self.interface_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("interface"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(interface), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(interfaces) = self.implements_interfaces() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(interfaces.doc(ctx));
            trivias = format_trivias_after_node(&interfaces, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(fields_def) = self.fields_definition() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(fields_def.doc(ctx));
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
        DelimitersFormatter::bracket(
            self.l_brack_token().map(SyntaxElement::Token),
            self.r_brack_token().map(SyntaxElement::Token),
            Some(true),
            ctx,
        )
        .with_single_line(Some(&SingleLine::Prefer))
        .with_space(Doc::nil())
        .format(self.ty().map(|ty| ty.doc(ctx)).unwrap_or_else(Doc::nil))
    }
}

impl DocGen for ListValue {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("[]")
        } else {
            DelimitersFormatter::bracket(
                self.l_brack_token().map(SyntaxElement::Token),
                self.r_brack_token().map(SyntaxElement::Token),
                Some(ctx.options.bracket_spacing),
                ctx,
            )
            .with_single_line(ctx.options.list_value_single_line.as_ref())
            .format(format_optional_comma_separated_list(
                self,
                self.values(),
                Doc::line_or_space(),
                ctx.options.list_value_comma.as_ref(),
                ctx,
            ))
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
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(value.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for ObjectTypeDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(description) = self.description() {
            docs.push(description.doc(ctx));
            trivias = format_trivias_after_node(&description, ctx);
        }
        if let Some(r#type) = self.type_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("type"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(r#type), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(interfaces) = self.implements_interfaces() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(interfaces.doc(ctx));
            trivias = format_trivias_after_node(&interfaces, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(fields_def) = self.fields_definition() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(fields_def.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for ObjectTypeExtension {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(extend) = self.extend_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text("extend"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(extend), ctx);
        }
        if let Some(r#type) = self.type_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("type"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(r#type), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(interfaces) = self.implements_interfaces() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(interfaces.doc(ctx));
            trivias = format_trivias_after_node(&interfaces, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(fields_def) = self.fields_definition() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(fields_def.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for ObjectValue {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("{}")
        } else {
            DelimitersFormatter::brace(
                self.l_curly_token().map(SyntaxElement::Token),
                self.r_curly_token().map(SyntaxElement::Token),
                ctx.options.object_value_brace_spacing,
                ctx,
            )
            .with_single_line(ctx.options.object_value_single_line.as_ref())
            .format(format_optional_comma_separated_list(
                self,
                self.object_fields(),
                Doc::line_or_space(),
                ctx.options.object_value_comma.as_ref(),
                ctx,
            ))
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
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(variable_defs) = self.variable_definitions() {
            docs.append(&mut trivias);
            docs.push(variable_defs.doc(ctx));
            trivias = format_trivias_after_node(&variable_defs, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(selection_set) = self.selection_set() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
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

impl DocGen for RootOperationTypeDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(6);
        let mut trivias = vec![];
        if let Some(operation_type) = self.operation_type() {
            docs.push(operation_type.doc(ctx));
            trivias = format_trivias_after_node(&operation_type, ctx);
        }
        if let Some(colon) = self.colon_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text(":"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(colon), ctx);
        }
        if let Some(named_type) = self.named_type() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(named_type.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for ScalarTypeDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(description) = self.description() {
            docs.push(description.doc(ctx));
            trivias = format_trivias_after_node(&description, ctx);
        }
        if let Some(scalar) = self.scalar_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("scalar"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(scalar), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
        }

        Doc::list(docs)
    }
}

impl DocGen for ScalarTypeExtension {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(extend) = self.extend_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text("extend"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(extend), ctx);
        }
        if let Some(scalar) = self.scalar_token() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(Doc::text("scalar"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(scalar), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
        }

        Doc::list(docs)
    }
}

impl DocGen for SchemaDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(description) = self.description() {
            docs.push(description.doc(ctx));
            trivias = format_trivias_after_node(&description, ctx);
        }
        if let Some(schema) = self.schema_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("schema"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(schema), ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(l_curly) = self.l_curly_token() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            let is_empty = l_curly
                .siblings_with_tokens(Direction::Next)
                .all(|element| {
                    element.kind() != SyntaxKind::WHITESPACE
                        && matches!(element, SyntaxElement::Token(..))
                });
            docs.push(if is_empty {
                Doc::text("{}")
            } else {
                DelimitersFormatter::brace(
                    self.l_curly_token().map(SyntaxElement::Token),
                    self.r_curly_token().map(SyntaxElement::Token),
                    ctx.options.schema_definition_brace_spacing,
                    ctx,
                )
                .with_single_line(ctx.options.schema_definition_single_line.as_ref())
                .format(format_optional_comma_separated_list(
                    self,
                    self.root_operation_type_definitions(),
                    Doc::hard_line(),
                    ctx.options.schema_definition_comma.as_ref(),
                    ctx,
                ))
            });
        }

        Doc::list(docs)
    }
}

impl DocGen for SchemaExtension {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(extend) = self.extend_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text("extend"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(extend), ctx);
        }
        if let Some(schema) = self.schema_token() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(Doc::text("schema"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(schema), ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(l_curly) = self.l_curly_token() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            let is_empty = l_curly
                .siblings_with_tokens(Direction::Next)
                .all(|element| {
                    element.kind() != SyntaxKind::WHITESPACE
                        && matches!(element, SyntaxElement::Token(..))
                });
            docs.push(if is_empty {
                Doc::text("{}")
            } else {
                DelimitersFormatter::brace(
                    self.l_curly_token().map(SyntaxElement::Token),
                    self.r_curly_token().map(SyntaxElement::Token),
                    ctx.options.schema_extension_brace_spacing,
                    ctx,
                )
                .with_single_line(ctx.options.schema_extension_single_line.as_ref())
                .format(format_optional_comma_separated_list(
                    self,
                    self.root_operation_type_definitions(),
                    Doc::hard_line(),
                    ctx.options.schema_extension_comma.as_ref(),
                    ctx,
                ))
            });
        }

        Doc::list(docs)
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
        DelimitersFormatter::brace(
            self.l_curly_token().map(SyntaxElement::Token),
            self.r_curly_token().map(SyntaxElement::Token),
            ctx.options.selection_set_brace_spacing,
            ctx,
        )
        .with_single_line(ctx.options.selection_set_single_line.as_ref())
        .format(format_optional_comma_separated_list(
            self,
            self.selections(),
            Doc::hard_line(),
            ctx.options.selection_set_comma.as_ref(),
            ctx,
        ))
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
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(named_type.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for UnionMemberTypes {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(3);
        let mut trivias = vec![];
        if let Some(eq) = self.eq_token() {
            docs.push(Doc::text("="));
            trivias = format_trivias_after_token(&SyntaxElement::Token(eq), ctx);
        }
        if self.named_types().count() > 0 {
            let types_doc = format_union_like(self, self.named_types(), S![|], "|", ctx);
            if trivias.is_empty() {
                docs.push(
                    Doc::line_or_space()
                        .append(types_doc)
                        .group()
                        .nest(ctx.indent_width),
                );
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(types_doc.group().nest(ctx.indent_width));
            }
        }

        Doc::list(docs)
    }
}

impl DocGen for UnionTypeDefinition {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(description) = self.description() {
            docs.push(description.doc(ctx));
            trivias = format_trivias_after_node(&description, ctx);
        }
        if let Some(union) = self.union_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("union"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(union), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(union_member_types) = self.union_member_types() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(union_member_types.doc(ctx));
        }

        Doc::list(docs)
    }
}

impl DocGen for UnionTypeExtension {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        let mut docs = Vec::with_capacity(5);
        let mut trivias = vec![];
        if let Some(extend) = self.extend_token() {
            docs.append(&mut trivias);
            docs.push(Doc::text("extend"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(extend), ctx);
        }
        if let Some(union) = self.union_token() {
            if !docs.is_empty() {
                docs.push(Doc::space());
            }
            docs.append(&mut trivias);
            docs.push(Doc::text("union"));
            trivias = format_trivias_after_token(&SyntaxElement::Token(union), ctx);
        }
        if let Some(name) = self.name() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(name.doc(ctx));
            trivias = format_trivias_after_node(&name, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
            trivias = format_trivias_after_node(&directives, ctx);
        }
        if let Some(union_member_types) = self.union_member_types() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(union_member_types.doc(ctx));
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
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(ty.doc(ctx));
            trivias = format_trivias_after_node(&ty, ctx);
        }
        if let Some(default_value) = self.default_value() {
            docs.push(Doc::space());
            docs.append(&mut trivias);
            docs.push(default_value.doc(ctx));
            trivias = format_trivias_after_node(&default_value, ctx);
        }
        if let Some(directives) = self.directives() {
            if trivias.is_empty() {
                docs.push(Doc::line_or_space().append(directives.doc(ctx)).group());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias);
                docs.push(directives.doc(ctx).group());
            }
        }

        Doc::list(docs)
    }
}

impl DocGen for VariableDefinitions {
    fn doc(&self, ctx: &Ctx) -> Doc<'static> {
        if is_empty_delimiter(self) {
            Doc::text("()")
        } else {
            DelimitersFormatter::paren(
                self.l_paren_token().map(SyntaxElement::Token),
                self.r_paren_token().map(SyntaxElement::Token),
                ctx.options.variable_definitions_paren_spacing,
                ctx,
            )
            .with_single_line(ctx.options.variable_definitions_single_line.as_ref())
            .format(format_optional_comma_separated_list(
                self,
                self.variable_definitions(),
                Doc::line_or_space(),
                ctx.options.variable_definitions_comma.as_ref(),
                ctx,
            ))
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
    comma: Option<&Comma>,
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
            SyntaxElement::Token(token) if token.kind() == S![,] => Some(token),
            _ => None,
        });
    let comma = comma.unwrap_or(&ctx.options.comma);
    while let Some(entry) = entries.next() {
        docs.push(entry.doc(ctx));
        match comma {
            Comma::Always => {
                if entries.peek().is_some() {
                    docs.push(Doc::text(","));
                } else {
                    docs.push(Doc::flat_or_break(Doc::nil(), Doc::text(",")));
                }
            }
            Comma::Never => {}
            Comma::NoTrailing => {
                if entries.peek().is_some() {
                    docs.push(Doc::text(","));
                }
            }
            Comma::OnlySingleLine => docs.push(Doc::flat_or_break(Doc::text(","), Doc::nil())),
        }

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

fn format_union_like<N, Entry>(
    node: &N,
    mut entries: CstChildren<Entry>,
    sep_token_kind: SyntaxKind,
    sep_text: &'static str,
    ctx: &Ctx,
) -> Doc<'static>
where
    N: CstNode,
    Entry: CstNode + DocGen,
{
    let node = node.syntax();
    let sep_tokens = node
        .children_with_tokens()
        .filter_map(|element| match element {
            SyntaxElement::Token(token) if token.kind() == sep_token_kind => Some(token),
            _ => None,
        });
    let mut docs = Vec::with_capacity(4);

    if node
        .first_token()
        .is_some_and(|token| token.kind() != sep_token_kind)
    {
        if let Some(first) = entries.next() {
            docs.push(Doc::flat_or_break(
                Doc::nil(),
                Doc::text(sep_text).append(Doc::space()),
            ));
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

    let mut it = entries.zip(sep_tokens).peekable();
    while let Some((entry, sep_token)) = it.next() {
        docs.push(Doc::text(sep_text).append(Doc::space()));
        docs.push(entry.doc(ctx));
        if it.peek().is_some() {
            let mut trivias_after_sep_token =
                format_trivias_after_token(&SyntaxElement::Token(sep_token), ctx);
            let mut trivias_after_node = format_trivias_after_node(&entry, ctx);
            if trivias_after_sep_token.is_empty() && trivias_after_node.is_empty() {
                docs.push(Doc::line_or_space());
            } else {
                docs.push(Doc::space());
                docs.append(&mut trivias_after_sep_token);
                docs.append(&mut trivias_after_node);
            }
        }
    }

    Doc::list(docs)
}

struct DelimitersFormatter<'a> {
    open_text: &'static str,
    close_text: &'static str,
    space: Doc<'static>,
    open_token: Option<SyntaxElement>,
    close_token: Option<SyntaxElement>,
    single_line: Option<&'a SingleLine>,
    ctx: &'a Ctx<'a>,
}
impl<'a> DelimitersFormatter<'a> {
    fn paren(
        open: Option<SyntaxElement>,
        close: Option<SyntaxElement>,
        spacing: Option<bool>,
        ctx: &'a Ctx,
    ) -> Self {
        Self {
            open_text: "(",
            close_text: ")",
            space: if spacing.unwrap_or(ctx.options.paren_spacing) {
                Doc::line_or_space()
            } else {
                Doc::line_or_nil()
            },
            open_token: open,
            close_token: close,
            single_line: None,
            ctx,
        }
    }
    fn bracket(
        open: Option<SyntaxElement>,
        close: Option<SyntaxElement>,
        spacing: Option<bool>,
        ctx: &'a Ctx,
    ) -> Self {
        Self {
            open_text: "[",
            close_text: "]",
            space: if spacing.unwrap_or(ctx.options.bracket_spacing) {
                Doc::line_or_space()
            } else {
                Doc::line_or_nil()
            },
            open_token: open,
            close_token: close,
            single_line: None,
            ctx,
        }
    }
    fn brace(
        open: Option<SyntaxElement>,
        close: Option<SyntaxElement>,
        spacing: Option<bool>,
        ctx: &'a Ctx,
    ) -> Self {
        Self {
            open_text: "{",
            close_text: "}",
            space: if spacing.unwrap_or(ctx.options.brace_spacing) {
                Doc::line_or_space()
            } else {
                Doc::line_or_nil()
            },
            open_token: open,
            close_token: close,
            single_line: None,
            ctx,
        }
    }
    fn with_space(mut self, space: Doc<'static>) -> Self {
        self.space = space;
        self
    }
    fn with_single_line(mut self, single_line: Option<&'a SingleLine>) -> Self {
        self.single_line = single_line;
        self
    }
    fn format(self, body: Doc<'static>) -> Doc<'static> {
        let ctx = self.ctx;
        let mut docs = Vec::with_capacity(5);

        docs.push(Doc::text(self.open_text));

        if let Some(open) = self.open_token.and_then(|open| open.into_token()) {
            if let Some(token) = open
                .next_token()
                .filter(|token| token.kind() == SyntaxKind::WHITESPACE)
            {
                match self.single_line.unwrap_or(&ctx.options.single_line) {
                    SingleLine::Prefer => docs.push(self.space.clone()),
                    SingleLine::Smart => {
                        if token.text().contains(['\n', '\r']) {
                            docs.push(Doc::hard_line());
                        } else {
                            docs.push(self.space.clone());
                        }
                    }
                    SingleLine::Never => docs.push(Doc::hard_line()),
                }
                let mut trivia_docs = format_trivias_after_token(&SyntaxElement::Token(token), ctx);
                docs.append(&mut trivia_docs);
            } else {
                docs.push(self.space.clone());
                let mut trivia_docs = format_trivias_after_token(&SyntaxElement::Token(open), ctx);
                docs.append(&mut trivia_docs);
            }
        }

        docs.push(body);

        let mut has_comment = false;
        if let Some(close) = self.close_token.and_then(|close| close.into_token()) {
            let last_ws_index = close
                .prev_token()
                .filter(|token| token.kind() == SyntaxKind::WHITESPACE)
                .map(|token| token.index());
            let last_non_trivia =
                close
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
            .append(if has_comment {
                Doc::hard_line()
            } else {
                self.space
            })
            .append(Doc::text(self.close_text))
            .group()
    }
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
