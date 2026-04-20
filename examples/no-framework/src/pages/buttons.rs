use telegram_ui::{Button, ButtonMode, ButtonSize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct Buttons {
    pub element: HtmlElement,
}

impl Buttons {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page buttons-page");

        let mut html = String::new();
        html.push_str("<h1>Buttons</h1>\n");

        // Button Modes
        html.push_str("<h2>Button Modes</h2>\n");
        html.push_str("<div class='button-group'>\n");
        html.push_str(
            &Button::new()
                .mode(ButtonMode::Filled)
                .children("Filled")
                .render(),
        );
        html.push_str(
            &Button::new()
                .mode(ButtonMode::Bezeled)
                .children("Bezeled")
                .render(),
        );
        html.push_str(
            &Button::new()
                .mode(ButtonMode::Plain)
                .children("Plain")
                .render(),
        );
        html.push_str(
            &Button::new()
                .mode(ButtonMode::Gray)
                .children("Gray")
                .render(),
        );
        html.push_str(
            &Button::new()
                .mode(ButtonMode::Outline)
                .children("Outline")
                .render(),
        );
        html.push_str(
            &Button::new()
                .mode(ButtonMode::White)
                .children("White")
                .render(),
        );
        html.push_str("</div>\n");

        // Button Sizes
        html.push_str("<h2>Button Sizes</h2>\n");
        html.push_str("<div class='button-group'>\n");
        html.push_str(&Button::new().size(ButtonSize::S).children("Small").render());
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .children("Medium")
                .render(),
        );
        html.push_str(&Button::new().size(ButtonSize::L).children("Large").render());
        html.push_str("</div>\n");

        // Special States
        html.push_str("<h2>Special States</h2>\n");
        html.push_str("<div class='button-group'>\n");
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .mode(ButtonMode::Filled)
                .children("Stretched")
                .stretched(true)
                .render(),
        );
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .mode(ButtonMode::Filled)
                .children("Disabled")
                .disabled(true)
                .render(),
        );
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .mode(ButtonMode::Filled)
                .children("Loading")
                .loading(true)
                .render(),
        );
        html.push_str("</div>\n");

        // With icons
        html.push_str("<h2>With Icons</h2>\n");
        html.push_str("<div class='button-group'>\n");
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .mode(ButtonMode::Filled)
                .children("Search")
                .before("🔍")
                .render(),
        );
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .mode(ButtonMode::Filled)
                .children("Next")
                .after("➡")
                .render(),
        );
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .mode(ButtonMode::Filled)
                .children("Both")
                .before("🔍")
                .after("➡")
                .render(),
        );
        html.push_str("</div>\n");

        section.set_inner_html(&html);

        Ok(Self { element: section })
    }
}
