use crate::Page;
use telegram_ui::{Avatar, Progress};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct Service {
    pub element: HtmlElement,
}

impl Service {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page service-page");

        let mut html = String::new();
        html.push_str("<h1>Service Components</h1>\n");

        // Avatar
        html.push_str("<h2>Avatar</h2>\n");
        html.push_str("<div class='avatar-group'>\n");
        html.push_str(
            &Avatar::new()
                .initials("JD")
                .size("48px")
                .bg_color("#3498db")
                .text_color("white")
                .render(),
        );
        html.push_str(
            &Avatar::new()
                .initials("AS")
                .size("48px")
                .bg_color("#2ecc71")
                .text_color("white")
                .render(),
        );
        html.push_str(
            &Avatar::new()
                .initials("MK")
                .size("48px")
                .bg_color("#e74c3c")
                .text_color("white")
                .render(),
        );
        html.push_str("</div>\n");

        // Progress
        html.push_str("<h2>Progress</h2>\n");
        html.push_str("<div class='telegram-ui-progress'>\n");
        html.push_str("<div class='telegram-ui-progress-bar' style='width: 60%;'></div>\n");
        html.push_str("</div>\n");

        // Divider
        html.push_str("<h2>Divider</h2>\n");
        html.push_str("<div class='telegram-ui-divider'></div>\n");
        html.push_str("<div class='telegram-ui-divider telegram-ui-divider--inset'></div>\n");

        section.set_inner_html(&html);

        Ok(Self { element: section })
    }
}

impl Page for Service {
    fn element(&self) -> &HtmlElement {
        &self.element
    }
}
