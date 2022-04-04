use crate::{formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{AstNode, JsxText, JsxTextFields};
impl ToFormatElement for JsxText {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxTextFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
