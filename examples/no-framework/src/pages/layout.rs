use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct Layout {
    pub element: HtmlElement,
}

impl Layout {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page layout-page");

        let mut html = String::new();
        html.push_str("<h1>Layout Components</h1>\n");

        // Card
        html.push_str("<h2>Card</h2>\n");
        html.push_str("<div class='telegram-ui-card'>\n");
        html.push_str("<h3>Card Title</h3>\n");
        html.push_str("<p>This is a regular card with shadow</p>\n");
        html.push_str("</div>\n");

        html.push_str("<div class='telegram-ui-card telegram-ui-card--ambient'>\n");
        html.push_str("<h3>Ambient Card</h3>\n");
        html.push_str("<p>This is an ambient card with secondary background</p>\n");
        html.push_str("</div>\n");

        // List with Cells
        html.push_str("<h2>List with Cells</h2>\n");
        html.push_str("<div class='telegram-ui-list'>\n");
        html.push_str("<div class='telegram-ui-cell telegram-ui-cell--ios'>\n");
        html.push_str("<div class='middle'>Cell 1</div>\n");
        html.push_str("</div>\n");
        html.push_str("<div class='telegram-ui-cell telegram-ui-cell--ios'>\n");
        html.push_str("<div class='before'>📁</div>\n");
        html.push_str("<div class='middle'>Cell with icon</div>\n");
        html.push_str("<div class='after'>➡</div>\n");
        html.push_str("</div>\n");
        html.push_str("<div class='telegram-ui-cell telegram-ui-cell--ios'>\n");
        html.push_str("<div class='middle'>Last cell</div>\n");
        html.push_str("</div>\n");
        html.push_str("</div>\n");

        // Placeholder
        html.push_str("<h2>Placeholder</h2>\n");
        html.push_str("<div class='telegram-ui-placeholder'>\n");
        html.push_str("<h3>No Internet Connection</h3>\n");
        html.push_str("<p>Check your connection and try again</p>\n");
        html.push_str("</div>\n");

        section.set_inner_html(&html);

        Ok(Self { element: section })
    }
}
