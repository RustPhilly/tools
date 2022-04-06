use crate::{
    formatter_traits::FormatTokenAndNode, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::{JsxTagExpression, JsxTagExpressionFields};
impl ToFormatElement for JsxTagExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxTagExpressionFields { tag } = self.as_fields();
        tag.format(formatter)
    }
}
