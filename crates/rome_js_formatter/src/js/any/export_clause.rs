//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyExportClause;
use crate::prelude::*;
use rome_js_syntax::JsAnyExportClause;
impl FormatRule<JsAnyExportClause> for FormatJsAnyExportClause {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyExportClause,
        formatter: &Formatter<Self::Context>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyExportClause::JsExportDefaultDeclarationClause(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyExportClause::JsExportDefaultExpressionClause(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyExportClause::JsExportNamedClause(node) => formatted![formatter, [node.format()]],
            JsAnyExportClause::JsExportFromClause(node) => formatted![formatter, [node.format()]],
            JsAnyExportClause::JsExportNamedFromClause(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyExportClause::JsAnyDeclarationClause(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyExportClause::TsExportAsNamespaceClause(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyExportClause::TsExportAssignmentClause(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyExportClause::TsExportDeclareClause(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
