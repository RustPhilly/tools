use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rome_formatter::{group_elements, soft_block_indent};
use rome_js_syntax::{JsxElement, JsxElementFields};
impl ToFormatElement for JsxElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxElementFields {
            opening_element,
            children,
            closing_element,
        } = self.as_fields();

        Ok(group_elements(format_elements![
            opening_element.format(formatter)?,
            soft_block_indent(format_elements![formatter.format_list(children)]),
            closing_element.format(formatter)?
        ]))
    }
}
