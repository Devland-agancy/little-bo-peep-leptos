use leptos::{create_rw_signal, RwSignal, Scope, SignalGet, SignalSet};

#[derive(Copy, Clone, Debug)]
pub struct GlobalState {
    pub show_areas: RwSignal<bool>,
    pub burger_background: RwSignal<bool>,
    pub tab: RwSignal<usize>,
    pub labels: RwSignal<Vec<&'static str>>,
    pub margin_scroll_value: RwSignal<f64>,
    pub route: RwSignal<&'static str>,
    pub show_section_divider: RwSignal<bool>,
    pub on_mobile: RwSignal<bool>,
    pub solutions_state: RwSignal<Vec<bool>>,
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
            route: create_rw_signal(cx, ""),
            on_mobile: create_rw_signal(cx, true),
            solutions_state: create_rw_signal(cx, vec![]),
        }
    }

    pub fn init_solutions_state(current_ss: RwSignal<Vec<bool>>, count: usize) {
        current_ss.set(vec![false; count]);
    }

    pub fn update_solutions_state(
        current_ss: RwSignal<Vec<bool>>,
        update_index: usize,
        update_value: bool,
    ) {
        current_ss.set(
            current_ss
                .get()
                .iter()
                .enumerate()
                .map(|(i, ss)| {
                    if i == update_index {
                        return update_value;
                    }
                    *ss
                })
                .collect(),
        );
    }
}
