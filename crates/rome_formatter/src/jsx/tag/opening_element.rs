use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::{AstNode, JsxOpeningElement, JsxOpeningElementFields};
impl ToFormatElement for JsxOpeningElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsxOpeningElementFields {
            l_angle_token,
            name,
            attributes,
            r_angle_token,
        } = self.as_fields();

        Ok(format_elements![
            l_angle_token.format(formatter)?,
            name?.to_format_element(formatter)?,
            formatter.format_list(attributes),
            r_angle_token.format(formatter)?
        ])
    }
}
