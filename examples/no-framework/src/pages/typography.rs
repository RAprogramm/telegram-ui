use telegram_ui::{Caption, Headline, Subtitle, Text, Title};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct Typography {
    pub element: HtmlElement,
}

impl Typography {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page typography-page");

        let mut html = String::new();
        html.push_str("<h1>Typography</h1>\n");

        // Headline
        html.push_str("<h2>Headline</h2>\n");
        html.push_str(&Headline::new().with_text("Headline Text").render());

        // Title
        html.push_str("<h2>Title</h2>\n");
        html.push_str(&Title::new().with_text("Title Text").render());

        // Subtitle
        html.push_str("<h2>Subtitle</h2>\n");
        html.push_str(&Subtitle::new().with_text("Subtitle Text").render());

        // Text
        html.push_str("<h2>Text</h2>\n");
        html.push_str(&Text::new().with_text("Regular text content").render());

        // Caption
        html.push_str("<h2>Caption</h2>\n");
        html.push_str(&Caption::new().with_text("Caption text").render());

        section.set_inner_html(&html);

        Ok(Self { element: section })
    }
}
