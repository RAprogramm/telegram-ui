use std::cell::RefCell;
use std::rc::Rc;
use telegram_ui::get_styles;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{
    window, Document, HtmlAnchorElement, HtmlButtonElement, HtmlDivElement, HtmlStyleElement,
};

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    let window = window().unwrap();
    let document = window.document().unwrap();

    apply_styles(&document);

    let app = document.get_element_by_id("app").unwrap();
    let app: HtmlDivElement = app.dyn_into().unwrap();

    render_app(&document, &app);
}

fn apply_styles(document: &Document) {
    let styles = get_styles();

    if let Some(head) = document.head() {
        let style = document.create_element("style").unwrap();
        let style: HtmlStyleElement = style.dyn_into().unwrap();
        style.set_text_content(Some(styles));
        let _ = head.append_child(&style);
    }
}

fn render_app(document: &Document, container: &HtmlDivElement) {
    let app_root = create_app_root(document);
    let _ = container.append_child(&app_root);
}

fn create_app_root(document: &Document) -> HtmlDivElement {
    let app_root = document.create_element("div").unwrap();
    let app_root: HtmlDivElement = app_root.dyn_into().unwrap();
    app_root.set_class_name("tgui-app-root");

    let list = document.create_element("div").unwrap();
    let list: HtmlDivElement = list.dyn_into().unwrap();
    list.set_class_name("tgui-list");

    let cell_section = create_section(document, "This is section header", "And this is footer");
    cell_section.set_class_name("tgui-section");

    let cells_data = vec![
        ("Chat Settings", "💬"),
        ("Data and Storage", "💾"),
        ("Devices", "💻"),
    ];

    for (text, icon) in cells_data {
        let cell = create_cell(document, text, icon);
        let _ = cell_section.append_child(&cell);
    }

    let _ = list.append_child(&cell_section);

    let form_section = create_section(document, "Form section", "");
    form_section.set_class_name("tgui-section");

    let input = create_input(document, "Android title", "Something here");
    let _ = form_section.append_child(&input);

    let _ = list.append_child(&form_section);

    let banner_section = create_section(document, "Banner inside section", "");
    banner_section.set_class_name("tgui-section");

    let banner = create_banner(document);
    let _ = banner_section.append_child(&banner);

    let _ = list.append_child(&banner_section);

    let timeline_section = create_section(document, "Timeline", "");
    timeline_section.set_class_name("tgui-section");

    let timeline = create_timeline(document);
    let _ = timeline_section.append_child(&timeline);

    let _ = list.append_child(&timeline_section);

    let tooltip_section = create_section(document, "Tooltip section", "");
    tooltip_section.set_class_name("tgui-section");

    let banner_with_tooltip = create_banner_with_tooltip(document);
    let _ = tooltip_section.append_child(&banner_with_tooltip);

    let _ = list.append_child(&tooltip_section);

    let modal_section = create_section(document, "Section with modal", "");
    modal_section.set_class_name("tgui-section");

    let placeholder = create_placeholder(document);
    let _ = modal_section.append_child(&placeholder);

    let _ = list.append_child(&modal_section);

    let _ = app_root.append_child(&list);

    app_root
}

fn create_section(document: &Document, header: &str, footer: &str) -> HtmlDivElement {
    let section = document.create_element("div").unwrap();
    let section: HtmlDivElement = section.dyn_into().unwrap();

    if !header.is_empty() {
        let header_el = document.create_element("div").unwrap();
        let header_el: HtmlDivElement = header_el.dyn_into().unwrap();
        header_el.set_class_name("tgui-section-header");
        header_el.set_text_content(Some(header));
        let _ = section.append_child(&header_el);
    }

    if !footer.is_empty() {
        let footer_el = document.create_element("div").unwrap();
        let footer_el: HtmlDivElement = footer_el.dyn_into().unwrap();
        footer_el.set_class_name("tgui-section-footer");
        footer_el.set_text_content(Some(footer));
        let _ = section.append_child(&footer_el);
    }

    section
}

fn create_cell(document: &Document, text: &str, icon: &str) -> HtmlDivElement {
    let cell = document.create_element("div").unwrap();
    let cell: HtmlDivElement = cell.dyn_into().unwrap();
    cell.set_class_name("tgui-cell");

    let before = document.create_element("div").unwrap();
    let before: HtmlDivElement = before.dyn_into().unwrap();
    before.set_class_name("tgui-cell-before");
    before.set_text_content(Some(icon));
    let _ = cell.append_child(&before);

    let content = document.create_element("div").unwrap();
    let content: HtmlDivElement = content.dyn_into().unwrap();
    content.set_class_name("tgui-cell-content");
    content.set_text_content(Some(text));
    let _ = cell.append_child(&content);

    cell
}

fn create_input(document: &Document, header: &str, placeholder: &str) -> HtmlDivElement {
    let input_div = document.create_element("div").unwrap();
    let input_div: HtmlDivElement = input_div.dyn_into().unwrap();
    input_div.set_class_name("tgui-input");

    let header_el = document.create_element("div").unwrap();
    let header_el: HtmlDivElement = header_el.dyn_into().unwrap();
    header_el.set_class_name("tgui-input-header");
    header_el.set_text_content(Some(header));
    let _ = input_div.append_child(&header_el);

    let input_el = document.create_element("input").unwrap();
    input_el.set_class_name("tgui-input-field");
    input_el.set_attribute("placeholder", placeholder).unwrap();
    let _ = input_div.append_child(&input_el);

    input_div
}

fn create_banner(document: &Document) -> HtmlDivElement {
    let banner = document.create_element("div").unwrap();
    let banner: HtmlDivElement = banner.dyn_into().unwrap();
    banner.set_class_name("tgui-banner");

    let before = document.create_element("div").unwrap();
    let before: HtmlDivElement = before.dyn_into().unwrap();
    before.set_class_name("tgui-banner-before");

    let image = document.create_element("img").unwrap();
    image
        .set_attribute("src", "https://xelene.me/telegram.gif")
        .unwrap();
    image.set_class_name("tgui-banner-image");
    let _ = before.append_child(&image);

    let _ = banner.append_child(&before);

    let content = document.create_element("div").unwrap();
    let content: HtmlDivElement = content.dyn_into().unwrap();
    content.set_class_name("tgui-banner-content");

    let header = document.create_element("div").unwrap();
    let header: HtmlDivElement = header.dyn_into().unwrap();
    header.set_class_name("tgui-banner-header");
    header.set_text_content(Some("Introducing TON Space"));
    let _ = content.append_child(&header);

    let subheader = document.create_element("div").unwrap();
    let subheader: HtmlDivElement = subheader.dyn_into().unwrap();
    subheader.set_class_name("tgui-banner-subheader");
    let _ = content.append_child(&subheader);

    let _ = banner.append_child(&content);

    let button = document.create_element("a").unwrap();
    let button: HtmlAnchorElement = button.dyn_into().unwrap();
    button.set_class_name("tgui-banner-button");
    button.set_text_content(Some("Try it out"));
    button.set_attribute("target", "_blank").unwrap();
    button.set_attribute("href", "https://ton.space/").unwrap();
    let _ = banner.append_child(&button);

    banner
}

fn create_timeline(document: &Document) -> HtmlDivElement {
    let timeline = document.create_element("div").unwrap();
    let timeline: HtmlDivElement = timeline.dyn_into().unwrap();
    timeline.set_class_name("tgui-timeline");

    let items = vec![
        ("Arrived", "Yesterday", 0),
        ("Departed", "Today", 1),
        ("In transit", "Tomorrow", 2),
        ("Processed to delivery center", "Next week", 3),
        ("Shipped", "Someday", 4),
    ];

    for (header, description, index) in items {
        let item = document.create_element("div").unwrap();
        let item: HtmlDivElement = item.dyn_into().unwrap();
        item.set_class_name("tgui-timeline-item");
        if index == 2 {
            item.set_attribute("data-active", "true").unwrap();
        }

        let icon = document.create_element("div").unwrap();
        let icon: HtmlDivElement = icon.dyn_into().unwrap();
        icon.set_class_name("tgui-timeline-item-icon");
        icon.set_text_content(Some("●"));
        let _ = item.append_child(&icon);

        let content = document.create_element("div").unwrap();
        let content: HtmlDivElement = content.dyn_into().unwrap();
        content.set_class_name("tgui-timeline-item-content");

        let item_header = document.create_element("div").unwrap();
        let item_header: HtmlDivElement = item_header.dyn_into().unwrap();
        item_header.set_class_name("tgui-timeline-item-header");
        item_header.set_text_content(Some(header));
        let _ = content.append_child(&item_header);

        let item_desc = document.create_element("div").unwrap();
        let item_desc: HtmlDivElement = item_desc.dyn_into().unwrap();
        item_desc.set_class_name("tgui-timeline-item-description");
        item_desc.set_text_content(Some(description));
        let _ = content.append_child(&item_desc);

        let _ = item.append_child(&content);

        let _ = timeline.append_child(&item);
    }

    timeline
}

fn create_banner_with_tooltip(document: &Document) -> HtmlDivElement {
    let banner = document.create_element("div").unwrap();
    let banner: HtmlDivElement = banner.dyn_into().unwrap();
    banner.set_class_name("tgui-banner");

    let content = document.create_element("div").unwrap();
    let content: HtmlDivElement = content.dyn_into().unwrap();
    content.set_class_name("tgui-banner-content");

    let header = document.create_element("div").unwrap();
    let header: HtmlDivElement = header.dyn_into().unwrap();
    header.set_class_name("tgui-banner-header");
    header.set_text_content(Some("Tooltip on button"));
    let _ = content.append_child(&header);

    let subheader = document.create_element("div").unwrap();
    let subheader: HtmlDivElement = subheader.dyn_into().unwrap();
    subheader.set_class_name("tgui-banner-subheader");
    subheader.set_text_content(Some("Press the button to show the tooltip"));
    let _ = content.append_child(&subheader);

    let _ = banner.append_child(&content);

    let button = document.create_element("button").unwrap();
    let button: HtmlButtonElement = button.dyn_into().unwrap();
    button.set_class_name("tgui-tooltip-button");
    button.set_text_content(Some("Show"));

    let shown = Rc::new(RefCell::new(false));
    let shown_ref = shown.clone();

    let button_clone = button.clone();
    let closure = Closure::wrap(Box::new(move || {
        let mut shown_mut = shown_ref.borrow_mut();
        *shown_mut = !*shown_mut;
        let new_shown = *shown_mut;
        button_clone.set_text_content(Some(if new_shown { "Hide" } else { "Show" }));
        button_clone
            .set_attribute("data-shown", if new_shown { "true" } else { "false" })
            .unwrap();
    }) as Box<dyn FnMut()>);

    button
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    let _ = banner.append_child(&button);

    banner
}

fn create_placeholder(document: &Document) -> HtmlDivElement {
    let placeholder = document.create_element("div").unwrap();
    let placeholder: HtmlDivElement = placeholder.dyn_into().unwrap();
    placeholder.set_class_name("tgui-placeholder");

    let image = document.create_element("img").unwrap();
    image
        .set_attribute("src", "https://xelene.me/telegram.gif")
        .unwrap();
    image.set_class_name("tgui-placeholder-image");
    let _ = placeholder.append_child(&image);

    let content = document.create_element("div").unwrap();
    let content: HtmlDivElement = content.dyn_into().unwrap();
    content.set_class_name("tgui-placeholder-content");

    let header = document.create_element("div").unwrap();
    let header: HtmlDivElement = header.dyn_into().unwrap();
    header.set_class_name("tgui-placeholder-header");
    header.set_text_content(Some("This is placeholder"));
    let _ = content.append_child(&header);

    let description = document.create_element("div").unwrap();
    let description: HtmlDivElement = description.dyn_into().unwrap();
    description.set_class_name("tgui-placeholder-description");
    description.set_text_content(Some("And this is placeholder description"));
    let _ = content.append_child(&description);

    let _ = placeholder.append_child(&content);

    let action = document.create_element("button").unwrap();
    let action: HtmlButtonElement = action.dyn_into().unwrap();
    action.set_class_name("tgui-placeholder-button");
    action.set_text_content(Some("Open modal"));
    let _ = placeholder.append_child(&action);

    placeholder
}
