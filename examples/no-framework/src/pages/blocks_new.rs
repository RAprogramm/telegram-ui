use telegram_ui::{Chip, CircularProgress, Spoiler};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct BlocksNew {
    pub element: HtmlElement,
}

impl BlocksNew {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page blocks-new-page");

        let mut html = String::new();
        html.push_str("<h1>New Blocks Components</h1>\n");

        // Blockquote
        html.push_str("<h2>Blockquote</h2>\n");
        html.push_str(
            &telegram_ui::Blockquote::new()
                .children("This is a blockquote with styled text.")
                .render(),
        );
        html.push_str("<br><br>\n");
        html.push_str(
            &telegram_ui::Blockquote::new()
                .with_type("other")
                .children("<div>Custom HTML content here</div>")
                .render(),
        );
        html.push_str("<br><br>\n");

        // IconButton
        html.push_str("<h2>IconButton</h2>\n");
        html.push_str("<div class='button-group'>\n");
        html.push_str(
            &telegram_ui::IconButton::new()
                .size("s")
                .children("🔍")
                .render(),
        );
        html.push_str(
            &telegram_ui::IconButton::new()
                .size("m")
                .mode("bezeled")
                .children("⚙️")
                .render(),
        );
        html.push_str(
            &telegram_ui::IconButton::new()
                .size("l")
                .mode("gray")
                .children("➕")
                .render(),
        );
        html.push_str("</div>\n");

        // IconContainer
        html.push_str("<h2>IconContainer</h2>\n");
        html.push_str(&telegram_ui::IconContainer::new().children("🎨").render());
        html.push_str("<br><br>\n");

        // InlineButtons
        html.push_str("<h2>InlineButtons</h2>\n");
        html.push_str(
            &telegram_ui::InlineButtons::new()
                .add_item(&telegram_ui::Button::new().children("Cancel").render())
                .add_item(&telegram_ui::Button::new().children("OK").render())
                .render(),
        );
        html.push_str("<br><br>\n");

        // Timeline
        html.push_str("<h2>Timeline</h2>\n");
        html.push_str(
            &telegram_ui::Timeline::new()
                .add_item("<div class='timeline-item'>Step 1</div>")
                .add_item("<div class='timeline-item'>Step 2</div>")
                .add_item("<div class='timeline-item'>Step 3</div>")
                .render(),
        );
        html.push_str("<br><br>\n");

        section.set_inner_html(&html);

        Ok(Self { element: section })
    }
}
