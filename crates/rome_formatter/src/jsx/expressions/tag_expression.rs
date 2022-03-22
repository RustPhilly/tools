use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{AstNode, JsxTagExpression, JsxTagExpressionFields};
impl ToFormatElement for JsxTagExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxTagExpressionFields { tag } = self.as_fields();
        Ok(tag?.to_format_element(formatter)?)
    }
}
