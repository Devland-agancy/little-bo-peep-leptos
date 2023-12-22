pub const MENU_ITEMS: &'static [(&'static str, &'static str, &'static str)] = &[
    ("Chapter 1: A Few Refreshers", "", "ch_1"),
    ("Chapter 2: The Slope Formula", "Chapter 2: Slopes", "ch_2"),
];

//first value is for top second for bottom
pub const TITLE_MARGIN: &'static (&'static str, &'static str) = &("15px", "5px");

pub const SHOW_CLICKABLE_ITEMS_BORDERS: bool = true;

pub const GREEN_DIV_HEIGHT: u8 = 150;

// the max width of the main column of text when trying to fit a small screen, in other words the breakpoint of screen width that activates small screen mode , this can be edited in tailwind.config.js in here screens: { sm: "520px" }, after than run npx tailwindcss -i input.css -o style/output.css

// DESKTOP_TEXT_WIDTH found in input.css file in .gridColWidth class , 1fr {width} 1fr
