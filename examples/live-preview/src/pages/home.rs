use crate::Page;
use telegram_ui::{Button, ButtonMode, ButtonSize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct Home {
    pub element: HtmlElement,
}

impl Home {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page home-page");

        let mut html = String::new();
        html.push_str("<h1>Telegram UI - Live Preview</h1>\n");
        html.push_str("<p class='lead'>A comprehensive showcase of all Telegram UI components built with Rust</p>\n");
        html.push_str("<p>Built with safety, speed, and comfort of Rust language</p>\n");
        html.push_str("<br>\n");

        // Demo button using telegram-ui
        let btn_demo = Button::new()
            .size(ButtonSize::M)
            .mode(ButtonMode::Filled)
            .children("Example Button")
            .render();
        html.push_str("<h2>Example Component</h2>\n");
        html.push_str("<div style='margin-bottom: 24px;'>");
        html.push_str(&btn_demo);
        html.push_str("</div>\n");

        // Navigation cards
        html.push_str("<h2>Browse Components</h2>\n");
        html.push_str("<div class='link-grid'>\n");
        html.push_str("<a href='#buttons' class='link-card'>🔘 Buttons</a>\n");
        html.push_str("<a href='#forms' class='link-card'>📝 Forms</a>\n");
        html.push_str("<a href='#feedback' class='link-card'>⚡ Feedback</a>\n");
        html.push_str("<a href='#layout' class='link-card'>📦 Layout</a>\n");
        html.push_str("<a href='#typography' class='link-card'>📝 Typography</a>\n");
        html.push_str("<a href='#overlays' class='link-card'>🪟 Overlays</a>\n");
        html.push_str("<a href='#service' class='link-card'>⚙️ Service</a>\n");
        html.push_str("</div>\n");

        section.set_inner_html(&html);

        Ok(Self { element: section })
    }
}

impl Page for Home {
    fn element(&self) -> &HtmlElement {
        &self.element
    }
}
