use crate::Page;
use telegram_ui::{Chip, ColorInput, FileInput, FormInput, Rating, Slider};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct FormsNew {
    pub element: HtmlElement,
}

impl FormsNew {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page forms-new-page");

        let mut html = String::new();
        html.push_str("<h1>New Form Components</h1>\n");

        // Chip
        html.push_str("<h2>Chip</h2>\n");
        html.push_str("<div class='chip-group'>\n");
        html.push_str(&Chip::new("Tag 1").render());
        html.push_str(&Chip::new("Tag 2").mode("mono").render());
        html.push_str(&Chip::new("Tag 3").mode("outline").render());
        html.push_str(&Chip::new("Selected").selected(true).render());
        html.push_str("</div>\n");
        html.push_str("<br>\n");

        // ColorInput
        html.push_str("<h2>ColorInput</h2>\n");
        html.push_str(
            &ColorInput::new()
                .value("#ff0000")
                .text("Red Color")
                .render(),
        );
        html.push_str("<br><br>\n");

        // FileInput
        html.push_str("<h2>FileInput</h2>\n");
        html.push_str(&FileInput::new().label("Upload File").render());
        html.push_str("<br><br>\n");

        // FormInput
        html.push_str("<h2>FormInput</h2>\n");
        html.push_str(
            &FormInput::new()
                .header("Username")
                .children("<input type='text' placeholder='Enter username'>")
                .render(),
        );
        html.push_str("<br>\n");
        html.push_str(
            &FormInput::new()
                .status("error")
                .children("<input type='text' value='Invalid input'>")
                .render(),
        );
        html.push_str("<br><br>\n");

        // Slider
        html.push_str("<h2>Slider</h2>\n");
        html.push_str(&Slider::new().value(50.0).render());
        html.push_str("<br><br>\n");

        // Rating
        html.push_str("<h2>Rating</h2>\n");
        html.push_str(&Rating::new().value(3.5).render());
        html.push_str("<br><br>\n");

        section.set_inner_html(&html);

        Ok(Self { element: section })
    }
}

impl Page for FormsNew {
    fn element(&self) -> &HtmlElement {
        &self.element
    }
}
