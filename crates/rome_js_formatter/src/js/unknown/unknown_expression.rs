use crate::prelude::*;

use crate::formatter::unknown_node;
use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownExpression;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownExpression> for FormatNodeRule<JsUnknownExpression> {
    fn format_fields(
        node: &JsUnknownExpression,
        formatter: &Formatter<JsFormatContext>,
    ) -> FormatResult<FormatElement> {
        unknown_node(node.syntax()).format(formatter)
    }
}
