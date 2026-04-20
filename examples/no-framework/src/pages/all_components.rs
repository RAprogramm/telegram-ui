use telegram_ui::{
    Button, ButtonMode, ButtonSize, Card, Cell, Checkbox, Divider, List, Segment,
    SegmentedControl, Tab, TabBar,
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

pub struct AllComponents {
    pub element: HtmlElement,
}

impl AllComponents {
    pub fn new(document: &web_sys::Document) -> Result<Self, JsValue> {
        let section = document
            .create_element("section")?
            .dyn_into::<HtmlElement>()?;
        section.set_class_name("page all-components-page");

        let mut html = String::new();

        // Buttons
        html.push_str("<h2>Buttons</h2>\n");
        html.push_str("<div class='button-group'>");
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

        html.push_str("<h3>Sizes</h3>\n");
        html.push_str("<div class='button-group'>");
        html.push_str(&Button::new().size(ButtonSize::S).children("S").render());
        html.push_str(&Button::new().size(ButtonSize::M).children("M").render());
        html.push_str(&Button::new().size(ButtonSize::L).children("L").render());
        html.push_str("</div>\n");

        html.push_str("<h3>States</h3>\n");
        html.push_str("<div class='button-group'>");
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .mode(ButtonMode::Filled)
                .stretched(true)
                .children("Stretched")
                .render(),
        );
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .mode(ButtonMode::Filled)
                .disabled(true)
                .children("Disabled")
                .render(),
        );
        html.push_str(
            &Button::new()
                .size(ButtonSize::M)
                .mode(ButtonMode::Filled)
                .loading(true)
                .children("Loading")
                .render(),
        );
        html.push_str("</div>\n");

        // Cards
        html.push_str("<h2>Cards</h2>\n");
        html.push_str(&Card::new().children("Regular card").render());
        html.push_str(&Card::new().ambient(true).children("Ambient card").render());

        // Cells
        html.push_str("<h2>Cells</h2>\n");
        html.push_str(
            &Cell::new()
                .before("👤")
                .middle("Item 1")
                .after("➡")
                .render(),
        );
        html.push_str(
            &Cell::new()
                .before("✉️")
                .middle("Item 2")
                .after("10:30")
                .render(),
        );
        html.push_str(
            &Cell::new()
                .before("📞")
                .middle("Item 3")
                .after("Right")
                .render(),
        );

        // Lists
        html.push_str("<h2>List with Cells</h2>\n");
        html.push_str(
            &List::new()
                .add_cell("<Cell before='👤' middle='User' after='➡'></Cell>")
                .add_cell("<Cell before='💬' middle='Chat' after='➡'></Cell>")
                .render(),
        );

        // Dividers
        html.push_str("<h2>Dividers</h2>\n");
        html.push_str(&Divider::new().render());
        html.push_str("<p>Content between dividers</p>");
        html.push_str(&Divider::new().render());

        // TabBar (new)
        html.push_str("<h2>TabBar</h2>\n");
        html.push_str(
            &TabBar::new()
                .active_tab(0)
                .tab(Tab::new(0, "Home", "🏠"))
                .tab(Tab::new(1, "Search", "🔍"))
                .tab(Tab::new(2, "Profile", "👤"))
                .render(),
        );

        // SegmentedControl (new)
        html.push_str("<h2>SegmentedControl</h2>\n");
        html.push_str(
            &SegmentedControl::new()
                .active_value("all")
                .option(Segment::new("all", "All"))
                .option(Segment::new("users", "Users"))
                .option(Segment::new("groups", "Groups"))
                .render(),
        );

        section.set_inner_html(&html);
        Ok(Self { element: section })
    }
}
