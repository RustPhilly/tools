use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{AstNode, JsxElement, JsxElementFields};
impl ToFormatElement for JsxElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxElementFields {
            opening_element,
            children,
            closing_element,
        } = self.as_fields();

        Ok(format_elements![
            opening_element?.to_format_element(formatter)?,
            formatter.format_list(children),
            closing_element?.to_format_element(formatter)?
        ])
    }
}
