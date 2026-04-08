use crate::Page;
use telegram_ui::{Alert, AlertKind, Spinner, SpinnerSize};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct Feedback {
    pub element: HtmlElement,
}

impl Feedback {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page feedback-page");

        let mut html = String::new();
        html.push_str("<h1>Feedback Components</h1>\n");

        // Spinner
        html.push_str("<h2>Spinner</h2>\n");
        html.push_str("<div class='spinner-group'>\n");
        html.push_str(&Spinner::new().set_size(SpinnerSize::S).render());
        html.push_str(&Spinner::new().set_size(SpinnerSize::M).render());
        html.push_str(&Spinner::new().set_size(SpinnerSize::L).render());
        html.push_str("</div>\n");

        // Alert
        html.push_str("<h2>Alert</h2>\n");
        html.push_str(
            &Alert::new()
                .set_kind(AlertKind::Info)
                .set_message("This is an info alert")
                .render(),
        );
        html.push_str(
            &Alert::new()
                .set_kind(AlertKind::Success)
                .set_message("This is a success alert")
                .render(),
        );
        html.push_str(
            &Alert::new()
                .set_kind(AlertKind::Warning)
                .set_message("This is a warning alert")
                .render(),
        );
        html.push_str(
            &Alert::new()
                .set_kind(AlertKind::Error)
                .set_message("This is an error alert")
                .render(),
        );

        // Skeleton
        html.push_str("<h2>Skeleton</h2>\n");
        html.push_str(
            "<div class='telegram-ui-skeleton' style='width: 100%; height: 20px;'></div>\n",
        );
        html.push_str("<div class='telegram-ui-skeleton' style='width: 60%; height: 20px; margin-top: 8px;'></div>\n");

        // Empty State
        html.push_str("<h2>Empty State</h2>\n");
        html.push_str("<div class='telegram-ui-empty-state'>\n");
        html.push_str("<h3>No items yet</h3>\n");
        html.push_str("<p>Get started by adding a new item</p>\n");
        html.push_str("</div>\n");

        section.set_inner_html(&html);

        Ok(Self { element: section })
    }
}

impl Page for Feedback {
    fn element(&self) -> &HtmlElement {
        &self.element
    }
}
