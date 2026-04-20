use telegram_ui::{Checkbox, Radio, Switch};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct Forms {
    pub element: HtmlElement,
}

impl Forms {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page forms-page");

        let mut html = String::new();
        html.push_str("<h1>Form Components</h1>\n");

        // Input
        html.push_str("<h2>Input</h2>\n");
        html.push_str("<div class='form-group'>\n");
        html.push_str(
            "<input type='text' class='telegram-ui-input' placeholder='Enter text...'>\n",
        );
        html.push_str("</div>\n");

        // Textarea
        html.push_str("<h2>Textarea</h2>\n");
        html.push_str("<div class='form-group'>\n");
        html.push_str("<textarea class='telegram-ui-input telegram-ui-textarea' rows='4' placeholder='Enter message...'></textarea>\n");
        html.push_str("</div>\n");

        // Checkbox
        html.push_str("<h2>Checkbox</h2>\n");
        let checkbox = Checkbox::new().label("I agree to terms").render();
        html.push_str(&checkbox);
        html.push_str("<br><br>\n");

        // Radio
        html.push_str("<h2>Radio</h2>\n");
        let radio1 = Radio::new().label("Option 1").render();
        let radio2 = Radio::new().label("Option 2").render();
        html.push_str(&radio1);
        html.push_str(" ");
        html.push_str(&radio2);
        html.push_str("<br><br>\n");

        // Switch
        html.push_str("<h2>Switch</h2>\n");
        let switch = Switch::new().render();
        html.push_str(&switch);
        html.push_str("<br><br>\n");

        section.set_inner_html(&html);

        Ok(Self { element: section })
    }
}
