//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnySwitchClause;
use crate::prelude::*;
use rome_js_syntax::JsAnySwitchClause;
impl FormatRule<JsAnySwitchClause> for FormatJsAnySwitchClause {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnySwitchClause,
        formatter: &Formatter<Self::Context>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnySwitchClause::JsCaseClause(node) => formatted![formatter, [node.format()]],
            JsAnySwitchClause::JsDefaultClause(node) => formatted![formatter, [node.format()]],
        }
    }
}
