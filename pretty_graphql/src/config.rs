//! Types about configuration.

#[cfg(feature = "config_serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// The whole configuration of Pretty GraphQL.
///
/// For detail, please refer to [Configuration](https://github.com/g-plane/pretty_graphql/blob/main/docs/config.md) on GitHub.
pub struct FormatOptions {
    #[cfg_attr(feature = "config_serde", serde(flatten))]
    pub layout: LayoutOptions,
    #[cfg_attr(feature = "config_serde", serde(flatten))]
    pub language: LanguageOptions,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// Configuration related to layout, such as indentation or print width.
pub struct LayoutOptions {
    #[cfg_attr(feature = "config_serde", serde(alias = "printWidth"))]
    pub print_width: usize,

    #[cfg_attr(feature = "config_serde", serde(alias = "useTabs"))]
    pub use_tabs: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "indentWidth"))]
    pub indent_width: usize,

    #[cfg_attr(
        feature = "config_serde",
        serde(alias = "lineBreak", alias = "linebreak")
    )]
    pub line_break: LineBreak,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            print_width: 80,
            use_tabs: false,
            indent_width: 2,
            line_break: LineBreak::Lf,
        }
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum LineBreak {
    #[default]
    Lf,
    Crlf,
}

impl From<LineBreak> for tiny_pretty::LineBreak {
    fn from(value: LineBreak) -> Self {
        match value {
            LineBreak::Lf => tiny_pretty::LineBreak::Lf,
            LineBreak::Crlf => tiny_pretty::LineBreak::Crlf,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(default))]
/// Configuration related to syntax.
pub struct LanguageOptions {
    pub comma: Comma,
    #[cfg_attr(feature = "config_serde", serde(alias = "arguments.comma"))]
    pub arguments_comma: Option<Comma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "arguments_definition.comma",
            alias = "argumentsDefinition.comma"
        )
    )]
    pub arguments_definition_comma: Option<Comma>,
    #[cfg_attr(feature = "config_serde", serde(alias = "directives.comma"))]
    pub directives_comma: Option<Comma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "enum_values_definition.comma",
            alias = "enumValuesDefinition.comma"
        )
    )]
    pub enum_values_definition_comma: Option<Comma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "fields_definition.comma", alias = "fieldsDefinition.comma")
    )]
    pub fields_definition_comma: Option<Comma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "input_fields_definition.comma",
            alias = "inputFieldsDefinition.comma"
        )
    )]
    pub input_fields_definition_comma: Option<Comma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "list_value.comma", alias = "listValue.comma")
    )]
    pub list_value_comma: Option<Comma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "object_value.comma", alias = "objectValue.comma")
    )]
    pub object_value_comma: Option<Comma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "schema_definition.comma", alias = "schemaDefinition.comma")
    )]
    pub schema_definition_comma: Option<Comma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "schema_extension.comma", alias = "schemaExtension.comma")
    )]
    pub schema_extension_comma: Option<Comma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "selection_set.comma", alias = "selectionSet.comma")
    )]
    pub selection_set_comma: Option<Comma>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "variable_definitions.comma",
            alias = "variableDefinitions.comma"
        )
    )]
    pub variable_definitions_comma: Option<Comma>,

    #[cfg_attr(feature = "config_serde", serde(alias = "singleLine"))]
    pub single_line: SingleLine,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "arguments.single_line", alias = "arguments.singleLine")
    )]
    pub arguments_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "arguments_definition.single_line",
            alias = "argumentsDefinition.singleLine"
        )
    )]
    pub arguments_definition_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "enum_values_definition.single_line",
            alias = "enumValuesDefinition.singleLine"
        )
    )]
    pub enum_values_definition_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "directive_locations.single_line",
            alias = "directiveLocations.singleLine"
        )
    )]
    pub directive_locations_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "directives.single_line", alias = "directives.singleLine")
    )]
    pub directives_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "fields_definition.single_line",
            alias = "fieldsDefinition.singleLine"
        )
    )]
    pub fields_definition_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "implements_interfaces.single_line",
            alias = "implementsInterfaces.singleLine"
        )
    )]
    pub implements_interfaces_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "input_fields_definition.single_line",
            alias = "inputFieldsDefinition.singleLine"
        )
    )]
    pub input_fields_definition_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "list_value.single_line", alias = "listValue.singleLine")
    )]
    pub list_value_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "object_value.single_line", alias = "objectValue.singleLine")
    )]
    pub object_value_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "schema_definition.single_line",
            alias = "schemaDefinition.singleLine"
        )
    )]
    pub schema_definition_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "schema_extension.single_line",
            alias = "schemaExtension.singleLine"
        )
    )]
    pub schema_extension_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "selection_set.single_line",
            alias = "selectionSet.singleLine"
        )
    )]
    pub selection_set_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "union_member_types.single_line",
            alias = "unionMemberTypes.singleLine"
        )
    )]
    pub union_member_types_single_line: Option<SingleLine>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "variable_definitions.single_line",
            alias = "variableDefinitions.singleLine"
        )
    )]
    pub variable_definitions_single_line: Option<SingleLine>,

    #[cfg_attr(feature = "config_serde", serde(alias = "parenSpacing"))]
    pub paren_spacing: bool,
    #[cfg_attr(
        feature = "config_serde",
        serde(rename = "arguments.paren_spacing", alias = "arguments.parenSpacing")
    )]
    pub arguments_paren_spacing: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "arguments_definition.paren_spacing",
            alias = "argumentsDefinition.parenSpacing"
        )
    )]
    pub arguments_definition_paren_spacing: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "variable_definitions.paren_spacing",
            alias = "variableDefinitions.parenSpacing"
        )
    )]
    pub variable_definitions_paren_spacing: Option<bool>,

    #[cfg_attr(feature = "config_serde", serde(alias = "bracketSpacing"))]
    pub bracket_spacing: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "braceSpacing"))]
    pub brace_spacing: bool,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "enum_values_definition.brace_spacing",
            alias = "enumValuesDefinition.braceSpacing"
        )
    )]
    pub enum_values_definition_brace_spacing: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "fields_definition.brace_spacing",
            alias = "fieldsDefinition.braceSpacing"
        )
    )]
    pub fields_definition_brace_spacing: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "input_fields_definition.brace_spacing",
            alias = "inputFieldsDefinition.braceSpacing"
        )
    )]
    pub input_fields_definition_brace_spacing: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "object_value.brace_spacing",
            alias = "objectValue.braceSpacing"
        )
    )]
    pub object_value_brace_spacing: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "schema_definition.brace_spacing",
            alias = "schemaDefinition.braceSpacing"
        )
    )]
    pub schema_definition_brace_spacing: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "schema_extension.brace_spacing",
            alias = "schemaExtension.braceSpacing"
        )
    )]
    pub schema_extension_brace_spacing: Option<bool>,
    #[cfg_attr(
        feature = "config_serde",
        serde(
            rename = "selection_set.brace_spacing",
            alias = "selectionSet.braceSpacing"
        )
    )]
    pub selection_set_brace_spacing: Option<bool>,

    #[cfg_attr(feature = "config_serde", serde(alias = "formatComments"))]
    pub format_comments: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "ignoreCommentDirective"))]
    pub ignore_comment_directive: String,
}

impl Default for LanguageOptions {
    fn default() -> Self {
        LanguageOptions {
            comma: Comma::OnlySingleLine,
            arguments_comma: None,
            arguments_definition_comma: None,
            directives_comma: Some(Comma::Never),
            enum_values_definition_comma: Some(Comma::Never),
            fields_definition_comma: Some(Comma::Never),
            input_fields_definition_comma: Some(Comma::Never),
            list_value_comma: None,
            object_value_comma: None,
            schema_definition_comma: Some(Comma::Never),
            schema_extension_comma: Some(Comma::Never),
            selection_set_comma: Some(Comma::Never),
            variable_definitions_comma: None,
            single_line: SingleLine::Smart,
            arguments_single_line: None,
            arguments_definition_single_line: None,
            directive_locations_single_line: None,
            directives_single_line: None,
            enum_values_definition_single_line: Some(SingleLine::Never),
            fields_definition_single_line: Some(SingleLine::Never),
            implements_interfaces_single_line: None,
            input_fields_definition_single_line: Some(SingleLine::Never),
            list_value_single_line: None,
            object_value_single_line: None,
            schema_definition_single_line: Some(SingleLine::Never),
            schema_extension_single_line: Some(SingleLine::Never),
            selection_set_single_line: Some(SingleLine::Never),
            union_member_types_single_line: None,
            variable_definitions_single_line: None,
            paren_spacing: false,
            arguments_paren_spacing: None,
            arguments_definition_paren_spacing: None,
            variable_definitions_paren_spacing: None,
            bracket_spacing: false,
            brace_spacing: true,
            enum_values_definition_brace_spacing: None,
            fields_definition_brace_spacing: None,
            input_fields_definition_brace_spacing: None,
            object_value_brace_spacing: None,
            schema_definition_brace_spacing: None,
            schema_extension_brace_spacing: None,
            selection_set_brace_spacing: None,
            format_comments: false,
            ignore_comment_directive: "pretty-graphql-ignore".into(),
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum Comma {
    Always,
    Never,
    #[serde(alias = "noTrailing")]
    NoTrailing,
    #[serde(alias = "onlySingleLine")]
    OnlySingleLine,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "config_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "config_serde", serde(rename_all = "kebab-case"))]
pub enum SingleLine {
    Prefer,
    Smart,
    Never,
}
