use leptos::SignalGet;
use leptos_router::Location;

pub fn get_chapter(location: Location) -> String {
    location
        .pathname
        .get()
        .split('/')
        .collect::<Vec<&str>>()
        .get(2)
        .unwrap()
        .to_string()
}
