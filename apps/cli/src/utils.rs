use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

const RED: &str = "\x1b[91m";
const RESET: &str = "\x1b[0m";

pub fn get_render_config() -> RenderConfig<'static> {
    RenderConfig {
        prompt_prefix: Styled::new("?").with_fg(Color::DarkMagenta),
        answered_prompt_prefix: Styled::new("λ").with_fg(Color::DarkMagenta),
        prompt: StyleSheet::new().with_attr(Attributes::BOLD),
        default_value: StyleSheet::new().with_fg(Color::DarkGrey),
        help_message: StyleSheet::empty(),
        answer: StyleSheet::empty(),
        highlighted_option_prefix: Styled::new(">").with_fg(Color::DarkMagenta),
        scroll_up_prefix: Styled::new(" "),
        scroll_down_prefix: Styled::new(" "),
        option: StyleSheet::new().with_fg(Color::DarkGrey),
        selected_option: Some(StyleSheet::empty()),
        ..Default::default()
    }
}

pub fn error(e: anyhow::Error) {
    if let Some(source) = e.source() {
        eprintln!("{RED}# {source}{RESET}");
    } else {
        eprintln!("{RED}# {e}{RESET}");
    }
}
