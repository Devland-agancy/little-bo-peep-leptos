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
    pub solution_transition_duration: RwSignal<Vec<i32>>,
    pub btc_alignment_on_left: RwSignal<bool>,
    pub math_rendered: RwSignal<bool>,
    pub margin_mode: RwSignal<bool>,
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
            solution_transition_duration: create_rw_signal(cx, vec![]),
            btc_alignment_on_left: create_rw_signal(cx, false),
            math_rendered: create_rw_signal(cx, false),
            margin_mode: create_rw_signal(cx, false),
        }
    }

    pub fn init_solutions_state<T>(current_ss: RwSignal<Vec<T>>, count: usize, init_value: T)
    where
        T: Copy,
    {
        current_ss.set(vec![init_value; count]);
    }

    pub fn update_solutions_state<T>(
        current_ss: RwSignal<Vec<T>>,
        update_index: usize,
        update_value: T,
    ) where
        T: Copy,
    {
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
