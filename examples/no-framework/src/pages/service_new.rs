use telegram_ui::{HorizontalScroll, Tappable, VisuallyHidden};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct ServiceNew {
    pub element: HtmlElement,
}

impl ServiceNew {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page service-new-page");

        let mut html = String::new();
        html.push_str("<h1>New Service Components</h1>\n");

        // HorizontalScroll
        html.push_str("<h2>HorizontalScroll</h2>\n");
        html.push_str(
            &HorizontalScroll::new()
                .children(
                    "<div class='scroll-item'>Item 1</div>
                     <div class='scroll-item'>Item 2</div>
                     <div class='scroll-item'>Item 3</div>
                     <div class='scroll-item'>Item 4</div>
                     <div class='scroll-item'>Item 5</div>",
                )
                .render(),
        );
        html.push_str("<br><br>\n");

        // Tappable
        html.push_str("<h2>Tappable</h2>\n");
        html.push_str(
            &Tappable::new()
                .children("<div class='tappable-box'>Click me</div>")
                .render(),
        );
        html.push_str("<br>\n");
        html.push_str(
            &Tappable::new()
                .interactive(false)
                .children("<div class='tappable-box tappable-disabled'>Disabled</div>")
                .render(),
        );
        html.push_str("<br><br>\n");

        // VisuallyHidden (hidden but accessible)
        html.push_str("<h2>VisuallyHidden</h2>\n");
        html.push_str("<p>Screen reader only text:</p>\n");
        html.push_str(
            &VisuallyHidden::new()
                .children("This text is hidden visually but available to screen readers")
                .render(),
        );
        html.push_str("<br><br>\n");

        section.set_inner_html(&html);

        Ok(Self { element: section })
    }
}
