mod pages;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement};

const NAV_CATEGORIES: &[(&str, &[(&str, &str)])] = &[
    (
        "interaction",
        &[
            ("buttons", "Buttons"),
            ("forms", "Forms"),
            ("forms_new", "Forms New"),
            ("feedback", "Feedback"),
        ],
    ),
    (
        "structure",
        &[
            ("layout", "Layout"),
            ("typography", "Typography"),
            ("blocks_new", "Blocks"),
        ],
    ),
    (
        "overlays",
        &[
            ("overlays", "Overlays"),
            ("service", "Service"),
            ("service_new", "Service New"),
        ],
    ),
    ("components", &[("all", "All Components")]),
];

const ALL_PAGES: &[(&str, &str, &str)] = &[
    ("buttons", "Interaction", "interaction"),
    ("forms", "Interaction", "interaction"),
    ("forms_new", "Interaction", "interaction"),
    ("feedback", "Interaction", "interaction"),
    ("layout", "Structure", "structure"),
    ("typography", "Structure", "structure"),
    ("blocks_new", "Structure", "structure"),
    ("overlays", "Overlays & Service", "overlays"),
    ("service", "Overlays & Service", "overlays"),
    ("service_new", "Overlays & Service", "overlays"),
    ("all", "Reference", "components"),
];

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = window().expect("no global window");
    let document = window.document().expect("no document");
    let body = document.body().expect("no body");

    let styles = telegram_ui::get_styles();
    let style_elem = document
        .create_element("style")?
        .dyn_into::<HtmlElement>()?;
    style_elem.set_text_content(Some(styles));
    document.head().unwrap().append_child(&style_elem)?;

    let container = document.create_element("div")?.dyn_into::<HtmlElement>()?;
    container.set_class_name("tgui-app-root tgui-platform-base tg-theme-auto");
    body.append_child(&container)?;

    // Build header with segmented control
    let header = document.create_element("div")?.dyn_into::<HtmlElement>()?;
    header.set_inner_html(&build_segmented_control());
    container.append_child(&header)?;

    // Build content area
    let content = document.create_element("div")?.dyn_into::<HtmlElement>()?;
    content.set_class_name("app-content");
    content.set_id("page-content");
    container.append_child(&content)?;

    let _ = show_page(&document, &content, "buttons");

    // Build bottom navigation
    let bottom_nav = document.create_element("nav")?.dyn_into::<HtmlElement>()?;
    bottom_nav.set_class_name("tgui-tab-bar tgui-tab-bar--stretched");
    bottom_nav.set_id("bottom-nav");
    bottom_nav.set_inner_html(&build_bottom_nav());
    container.append_child(&bottom_nav)?;

    // Navigation click handler
    let bottom_nav_clone = bottom_nav.clone();
    let doc_clone = document.clone();
    let content_clone = content.clone();

    let closure = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        e.prevent_default();
        if let Some(target) = e.target() {
            if let Ok(elem) = target.dyn_into::<HtmlElement>() {
                if elem.class_list().contains("nav-category-title") {
                    if let Some(category) = elem.get_attribute("data-category") {
                        update_segmented_control(&document, &header, &category);
                        show_first_page_in_category(&document, &content_clone, &category).ok();
                    }
                } else if elem.class_list().contains("nav-item") {
                    let active_arr = js_sys::Array::new();
                    active_arr.push(&"active".into());

                    if let Ok(items) = bottom_nav_clone.query_selector_all(".nav-item") {
                        for idx in 0..items.length() {
                            if let Some(node) = items.get(idx) {
                                if let Ok(el) = node.dyn_into::<HtmlElement>() {
                                    el.class_list().remove(&active_arr);
                                }
                            }
                        }
                    }

                    let _ = elem.class_list().add(&active_arr);

                    if let Some(page) = elem.get_attribute("data-page") {
                        show_page(&doc_clone, &content_clone, &page).ok();
                    }
                }
            }
        }
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);

    bottom_nav.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

fn build_segmented_control() -> String {
    let _categories: Vec<_> = NAV_CATEGORIES.iter().map(|(id, _)| *id).collect();

    let mut html = String::new();
    html.push_str("<div class='tgui-segmented-control tgui-segmented-control--m'>");
    html.push_str("<div class='tgui-segmented-control__container'>");

    for (i, (category_id, _)) in NAV_CATEGORIES.iter().enumerate() {
        let mut classes = vec!["tgui-segment".to_string()];

        if i == 0 {
            classes.push("tgui-segment--first".to_string());
        }
        if i == NAV_CATEGORIES.len() - 1 {
            classes.push("tgui-segment--last".to_string());
        }

        let class_str = classes.join(" ");

        html.push_str(&format!(
            "<button class='{}' data-category='{}'>",
            class_str, category_id
        ));
        html.push_str(&format!(
            "<span class='tgui-segment__label'>{}</span>",
            format_category_name(category_id)
        ));
        html.push_str("</button>");
    }

    html.push_str("</div>");
    html.push_str("</div>");

    html
}

fn update_segmented_control(_document: &web_sys::Document, header: &HtmlElement, category: &str) {
    if let Some(container) = header
        .query_selector(".tgui-segmented-control__container")
        .ok()
        .flatten()
    {
        if let Ok(items) = container.query_selector_all(".tgui-segment") {
            for idx in 0..items.length() {
                if let Some(node) = items.get(idx) {
                    if let Ok(el) = node.dyn_into::<HtmlElement>() {
                        let active_arr = js_sys::Array::new();
                        active_arr.push(&"active".into());

                        el.class_list().remove(&active_arr);

                        if let Some(data_cat) = el.get_attribute("data-category") {
                            if &data_cat == category {
                                el.class_list().add(&active_arr);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn show_first_page_in_category(
    document: &web_sys::Document,
    content: &HtmlElement,
    category: &str,
) -> Result<(), JsValue> {
    if let Some((page_id, _, _cat)) = ALL_PAGES.iter().find(|(_, _, c)| *c == category) {
        show_page(document, content, page_id)
    } else {
        show_page(document, content, "home")
    }
}

fn build_bottom_nav() -> String {
    let mut html = String::new();
    html.push_str("<div class='tgui-tab-bar__content'>");
    html.push_str("<div class='tgui-tab-bar__tabs'>");

    for (page_id, label, _) in ALL_PAGES {
        html.push_str(&format!(
            "<button class='tgui-nav-item' data-page='{}'><span class='tgui-nav-label'>{}</span></button>",
            page_id, label
        ));
    }

    html.push_str("</div>");
    html.push_str("</div>");
    html.push_str("</div>");

    html
}

fn format_category_name(name: &str) -> String {
    match name {
        "interaction" => "Interaction",
        "structure" => "Structure",
        "overlays" => "Overlays & Service",
        "components" => "Reference",
        _ => name,
    }
    .to_string()
}

fn show_page(
    document: &web_sys::Document,
    content: &HtmlElement,
    page: &str,
) -> Result<(), JsValue> {
    content.set_inner_html("");

    let page_elem = match page {
        "buttons" => pages::Buttons::new(document)?.element,
        "forms" => pages::Forms::new(document)?.element,
        "forms_new" => pages::FormsNew::new(document)?.element,
        "feedback" => pages::Feedback::new(document)?.element,
        "layout" => pages::Layout::new(document)?.element,
        "typography" => pages::Typography::new(document)?.element,
        "blocks_new" => pages::BlocksNew::new(document)?.element,
        "overlays" => pages::Overlays::new(document)?.element,
        "service" => pages::Service::new(document)?.element,
        "service_new" => pages::ServiceNew::new(document)?.element,
        "all" => pages::AllComponents::new(document)?.element,
        _ => pages::Buttons::new(document)?.element,
    };

    content.append_child(&page_elem)?;
    Ok(())
}
