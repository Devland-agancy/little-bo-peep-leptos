use leptos::{create_rw_signal, RwSignal, Scope};

#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub show_areas: RwSignal<bool>,
    pub burger_background: RwSignal<bool>,
    pub tab: RwSignal<usize>,
    pub labels: RwSignal<Vec<&'static str>>,
    pub margin_scroll_value: RwSignal<f64>,
    pub show_section_divider: RwSignal<bool>,
}

impl GlobalState {
    pub fn new(cx: Scope) -> Self {
        Self {
            show_areas: create_rw_signal(cx, false),
            show_section_divider: create_rw_signal(cx, false),
            burger_background: create_rw_signal(cx, true),
            tab: create_rw_signal(cx, 0),
            labels: create_rw_signal(cx, vec![]),
            margin_scroll_value: create_rw_signal(cx, 0_f64),
        }
    }
}
