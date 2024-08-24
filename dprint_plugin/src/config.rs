use dprint_core::configuration::{
    get_nullable_value, get_unknown_property_diagnostics, get_value, ConfigKeyMap,
    ConfigurationDiagnostic, GlobalConfiguration, NewLineKind, ResolveConfigurationResult,
};
use pretty_graphql::config::*;

pub(crate) fn resolve_config(
    mut config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<FormatOptions> {
    let mut diagnostics = Vec::new();
    let pretty_graphql_config = FormatOptions {
        layout: LayoutOptions {
            print_width: get_value(
                &mut config,
                "printWidth",
                global_config.line_width.unwrap_or(80),
                &mut diagnostics,
            ) as usize,
            use_tabs: get_value(
                &mut config,
                "useTabs",
                global_config.use_tabs.unwrap_or_default(),
                &mut diagnostics,
            ),
            indent_width: get_value(
                &mut config,
                "indentWidth",
                global_config.indent_width.unwrap_or(2),
                &mut diagnostics,
            ) as usize,
            line_break: match &*get_value(
                &mut config,
                "lineBreak",
                match global_config.new_line_kind {
                    Some(NewLineKind::LineFeed) => "lf",
                    Some(NewLineKind::CarriageReturnLineFeed) => "crlf",
                    _ => "lf",
                }
                .to_string(),
                &mut diagnostics,
            ) {
                "lf" => LineBreak::Lf,
                "crlf" => LineBreak::Crlf,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "lineBreak".into(),
                        message: "invalid value for config `lineBreak`".into(),
                    });
                    LineBreak::Lf
                }
            },
        },
        language: LanguageOptions {
            comma: match &*get_value(
                &mut config,
                "comma",
                "onlySingleLine".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "comma".into(),
                        message: "invalid value for config `comma`".into(),
                    });
                    Comma::OnlySingleLine
                }
            },
            arguments_comma: match &*get_value(
                &mut config,
                "arguments.comma",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "arguments.comma".into(),
                        message: "invalid value for config `arguments.comma`".into(),
                    });
                    Comma::Inherit
                }
            },
            arguments_definition_comma: match &*get_value(
                &mut config,
                "argumentsDefinition.comma",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "argumentsDefinition.comma".into(),
                        message: "invalid value for config `argumentsDefinition.comma`".into(),
                    });
                    Comma::Inherit
                }
            },
            directives_comma: match &*get_value(
                &mut config,
                "directives.comma",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "directives.comma".into(),
                        message: "invalid value for config `directives.comma`".into(),
                    });
                    Comma::Never
                }
            },
            enum_values_definition_comma: match &*get_value(
                &mut config,
                "enumValuesDefinition.comma",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "enumValuesDefinition.comma".into(),
                        message: "invalid value for config `enumValuesDefinition.comma`".into(),
                    });
                    Comma::Never
                }
            },
            fields_definition_comma: match &*get_value(
                &mut config,
                "fieldsDefinition.comma",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "fieldsDefinition.comma".into(),
                        message: "invalid value for config `fieldsDefinition.comma`".into(),
                    });
                    Comma::Never
                }
            },
            input_fields_definition_comma: match &*get_value(
                &mut config,
                "inputFieldsDefinition.comma",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "inputFieldsDefinition.comma".into(),
                        message: "invalid value for config `inputFieldsDefinition.comma`".into(),
                    });
                    Comma::Never
                }
            },
            list_value_comma: match &*get_value(
                &mut config,
                "listValue.comma",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "listValue.comma".into(),
                        message: "invalid value for config `listValue.comma`".into(),
                    });
                    Comma::Inherit
                }
            },
            object_value_comma: match &*get_value(
                &mut config,
                "objectValue.comma",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "objectValue.comma".into(),
                        message: "invalid value for config `objectValue.comma`".into(),
                    });
                    Comma::Inherit
                }
            },
            schema_definition_comma: match &*get_value(
                &mut config,
                "schemaDefinition.comma",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "schemaDefinition.comma".into(),
                        message: "invalid value for config `schemaDefinition.comma`".into(),
                    });
                    Comma::Never
                }
            },
            schema_extension_comma: match &*get_value(
                &mut config,
                "schemaExtension.comma",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "schemaExtension.comma".into(),
                        message: "invalid value for config `schemaExtension.comma`".into(),
                    });
                    Comma::Never
                }
            },
            selection_set_comma: match &*get_value(
                &mut config,
                "selectionSet.comma",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "selectionSet.comma".into(),
                        message: "invalid value for config `selectionSet.comma`".into(),
                    });
                    Comma::Never
                }
            },
            variable_definitions_comma: match &*get_value(
                &mut config,
                "variableDefinitions.comma",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "always" => Comma::Always,
                "never" => Comma::Never,
                "noTrailing" => Comma::NoTrailing,
                "onlySingleLine" => Comma::OnlySingleLine,
                "inherit" => Comma::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "variableDefinitions.comma".into(),
                        message: "invalid value for config `variableDefinitions.comma`".into(),
                    });
                    Comma::Inherit
                }
            },
            single_line: match &*get_value(
                &mut config,
                "singleLine",
                "smart".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "singleLine".into(),
                        message: "invalid value for config `singleLine`".into(),
                    });
                    SingleLine::Smart
                }
            },
            arguments_single_line: match &*get_value(
                &mut config,
                "arguments.singleLine",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "arguments.singleLine".into(),
                        message: "invalid value for config `arguments.singleLine`".into(),
                    });
                    SingleLine::Inherit
                }
            },
            arguments_definition_single_line: match &*get_value(
                &mut config,
                "argumentsDefinition.singleLine",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "argumentsDefinition.singleLine".into(),
                        message: "invalid value for config `argumentsDefinition.singleLine`".into(),
                    });
                    SingleLine::Inherit
                }
            },
            directive_locations_single_line: match &*get_value(
                &mut config,
                "directiveLocations.singleLine",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "directiveLocations.singleLine".into(),
                        message: "invalid value for config `directiveLocations.singleLine`".into(),
                    });
                    SingleLine::Inherit
                }
            },
            directives_single_line: match &*get_value(
                &mut config,
                "directives.singleLine",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "directives.singleLine".into(),
                        message: "invalid value for config `directives.singleLine`".into(),
                    });
                    SingleLine::Inherit
                }
            },
            enum_values_definition_single_line: match &*get_value(
                &mut config,
                "enumValuesDefinition.singleLine",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "enumValuesDefinition.singleLine".into(),
                        message: "invalid value for config `enumValuesDefinition.singleLine`"
                            .into(),
                    });
                    SingleLine::Never
                }
            },
            fields_definition_single_line: match &*get_value(
                &mut config,
                "fieldsDefinition.singleLine",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "fieldsDefinition.singleLine".into(),
                        message: "invalid value for config `fieldsDefinition.singleLine`".into(),
                    });
                    SingleLine::Never
                }
            },
            implements_interfaces_single_line: match &*get_value(
                &mut config,
                "implementsInterfaces.singleLine",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "implementsInterfaces.singleLine".into(),
                        message: "invalid value for config `implementsInterfaces.singleLine`"
                            .into(),
                    });
                    SingleLine::Inherit
                }
            },
            input_fields_definition_single_line: match &*get_value(
                &mut config,
                "inputFieldsDefinition.singleLine",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "inputFieldsDefinition.singleLine".into(),
                        message: "invalid value for config `inputFieldsDefinition.singleLine`"
                            .into(),
                    });
                    SingleLine::Never
                }
            },
            list_value_single_line: match &*get_value(
                &mut config,
                "listValue.singleLine",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "listValue.singleLine".into(),
                        message: "invalid value for config `listValue.singleLine`".into(),
                    });
                    SingleLine::Inherit
                }
            },
            object_value_single_line: match &*get_value(
                &mut config,
                "objectValue.singleLine",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "objectValue.singleLine".into(),
                        message: "invalid value for config `objectValue.singleLine`".into(),
                    });
                    SingleLine::Inherit
                }
            },
            schema_definition_single_line: match &*get_value(
                &mut config,
                "schemaDefinition.singleLine",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "schemaDefinition.singleLine".into(),
                        message: "invalid value for config `schemaDefinition.singleLine`".into(),
                    });
                    SingleLine::Never
                }
            },
            schema_extension_single_line: match &*get_value(
                &mut config,
                "schemaExtension.singleLine",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "schemaExtension.singleLine".into(),
                        message: "invalid value for config `schemaExtension.singleLine`".into(),
                    });
                    SingleLine::Never
                }
            },
            selection_set_single_line: match &*get_value(
                &mut config,
                "selectionSet.singleLine",
                "never".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "selectionSet.singleLine".into(),
                        message: "invalid value for config `selectionSet.singleLine`".into(),
                    });
                    SingleLine::Never
                }
            },
            union_member_types_single_line: match &*get_value(
                &mut config,
                "unionMemberTypes.singleLine",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "unionMemberTypes.singleLine".into(),
                        message: "invalid value for config `unionMemberTypes.singleLine`".into(),
                    });
                    SingleLine::Inherit
                }
            },
            variable_definitions_single_line: match &*get_value(
                &mut config,
                "variableDefinitions.singleLine",
                "inherit".to_string(),
                &mut diagnostics,
            ) {
                "prefer" => SingleLine::Prefer,
                "smart" => SingleLine::Smart,
                "never" => SingleLine::Never,
                "inherit" => SingleLine::Inherit,
                _ => {
                    diagnostics.push(ConfigurationDiagnostic {
                        property_name: "variableDefinitions.singleLine".into(),
                        message: "invalid value for config `variableDefinitions.singleLine`".into(),
                    });
                    SingleLine::Inherit
                }
            },
            paren_spacing: get_value(&mut config, "parenSpacing", false, &mut diagnostics),
            arguments_paren_spacing: get_nullable_value(
                &mut config,
                "arguments.parenSpacing",
                &mut diagnostics,
            ),
            arguments_definition_paren_spacing: get_nullable_value(
                &mut config,
                "argumentsDefinition.parenSpacing",
                &mut diagnostics,
            ),
            variable_definitions_paren_spacing: get_nullable_value(
                &mut config,
                "variableDefinitions.parenSpacing",
                &mut diagnostics,
            ),
            bracket_spacing: get_value(&mut config, "bracketSpacing", false, &mut diagnostics),
            brace_spacing: get_value(&mut config, "braceSpacing", true, &mut diagnostics),
            enum_values_definition_brace_spacing: get_nullable_value(
                &mut config,
                "enumValuesDefinition.braceSpacing",
                &mut diagnostics,
            ),
            fields_definition_brace_spacing: get_nullable_value(
                &mut config,
                "fieldsDefinition.braceSpacing",
                &mut diagnostics,
            ),
            input_fields_definition_brace_spacing: get_nullable_value(
                &mut config,
                "inputFieldsDefinition.braceSpacing",
                &mut diagnostics,
            ),
            object_value_brace_spacing: get_nullable_value(
                &mut config,
                "objectValue.braceSpacing",
                &mut diagnostics,
            ),
            schema_definition_brace_spacing: get_nullable_value(
                &mut config,
                "schemaDefinition.braceSpacing",
                &mut diagnostics,
            ),
            schema_extension_brace_spacing: get_nullable_value(
                &mut config,
                "schemaExtension.braceSpacing",
                &mut diagnostics,
            ),
            selection_set_brace_spacing: get_nullable_value(
                &mut config,
                "selectionSet.braceSpacing",
                &mut diagnostics,
            ),
            format_comments: get_value(&mut config, "formatComments", false, &mut diagnostics),
            ignore_comment_directive: get_value(
                &mut config,
                "ignoreCommentDirective",
                "dprint-ignore".into(),
                &mut diagnostics,
            ),
        },
    };

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        config: pretty_graphql_config,
        diagnostics,
    }
}
