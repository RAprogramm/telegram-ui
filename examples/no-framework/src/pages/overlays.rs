use telegram_ui::{Button, ButtonMode, ButtonSize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct Overlays {
    pub element: HtmlElement,
}

impl Overlays {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page overlays-page");

        let mut html = String::new();
        html.push_str("<h1>Overlays</h1>\n");

        // Toast
        html.push_str("<h2>Toast</h2>\n");
        html.push_str("<div class='telegram-ui-toast'>\n");
        html.push_str("This is a toast notification\n");
        html.push_str("</div>\n");

        // Modal
        html.push_str("<h2>Modal</h2>\n");
        html.push_str("<div class='telegram-ui-modal-demo'>\n");
        html.push_str("<div class='telegram-ui-modal'>\n");
        html.push_str("<h3>Modal Title</h3>\n");
        html.push_str("<p>This is the modal content</p>\n");
        html.push_str("<div class='modal-actions'>\n");
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .mode(ButtonMode::Filled)
                .children("OK")
                .render(),
        );
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .mode(ButtonMode::Plain)
                .children("Cancel")
                .render(),
        );
        html.push_str("</div>\n");
        html.push_str("</div>\n");
        html.push_str("</div>\n");

        // Backdrop
        html.push_str("<h2>Backdrop</h2>\n");
        html.push_str("<p>Semi-transparent overlay used behind modals</p>\n");

        section.set_inner_html(&html);

        Ok(Self { element: section })
    }
}
