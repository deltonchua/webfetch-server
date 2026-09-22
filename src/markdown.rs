use htmd::HtmlToMarkdown;

pub fn converter() -> HtmlToMarkdown {
    HtmlToMarkdown::builder()
        .skip_tags(vec![
            "script", "style", "noscript", "template", "svg", "canvas", "img", "picture", "audio",
            "video", "source", "track", "iframe", "frame", "frameset", "object", "embed", "applet",
            "form", "input", "button", "select", "option", "optgroup", "textarea", "fieldset",
            "legend", "head", "meta", "link", "base", "title", "nav", "dialog", "menu",
        ])
        .build()
}
