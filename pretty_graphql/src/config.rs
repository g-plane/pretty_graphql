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

    #[cfg_attr(feature = "config_serde", serde(alias = "formatComments"))]
    pub format_comments: bool,

    #[cfg_attr(feature = "config_serde", serde(alias = "ignoreCommentDirective"))]
    pub ignore_comment_directive: String,
}

impl Default for LanguageOptions {
    fn default() -> Self {
        LanguageOptions {
            comma: Comma::Always,
            arguments_comma: None,
            arguments_definition_comma: None,
            directives_comma: Some(Comma::Never),
            enum_values_definition_comma: Some(Comma::Never),
            fields_definition_comma: Some(Comma::Never),
            input_fields_definition_comma: Some(Comma::Never),
            list_value_comma: None,
            object_value_comma: Some(Comma::Never),
            schema_definition_comma: Some(Comma::Never),
            schema_extension_comma: Some(Comma::Never),
            selection_set_comma: Some(Comma::Never),
            variable_definitions_comma: None,
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
    NoTrailing,
    OnlySingleLine,
}
