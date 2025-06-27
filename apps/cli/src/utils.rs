use inquire::ui::{
    Attributes, Color, ErrorMessageRenderConfig, IndexPrefix, RenderConfig, StyleSheet, Styled,
};

pub fn get_render_config() -> RenderConfig<'static> {
    RenderConfig {
        prompt_prefix: Styled::new("?").with_fg(Color::DarkMagenta),
        answered_prompt_prefix: Styled::new("λ").with_fg(Color::DarkMagenta),
        prompt: StyleSheet::new().with_attr(Attributes::BOLD),
        default_value: StyleSheet::new().with_fg(Color::DarkGrey),
        placeholder: StyleSheet::empty(),
        help_message: StyleSheet::empty(),
        text_input: StyleSheet::empty(),
        error_message: ErrorMessageRenderConfig::empty(),
        answer: StyleSheet::empty(),
        canceled_prompt_indicator: Styled::new("<canceled>"),
        password_mask: '*',
        highlighted_option_prefix: Styled::new(">").with_fg(Color::DarkMagenta),
        scroll_up_prefix: Styled::new("^"),
        scroll_down_prefix: Styled::new("v"),
        selected_checkbox: Styled::new("[x]"),
        unselected_checkbox: Styled::new("[ ]"),
        option_index_prefix: IndexPrefix::None,
        option: StyleSheet::new().with_fg(Color::DarkGrey),
        selected_option: Some(StyleSheet::empty()),
    }
}
