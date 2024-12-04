use crate::components::Columns::*;
use crate::components::Link::CustomLink;
use crate::components::VerticalChunk::*;
use crate::components::Section::*;

use leptos::*;

use crate::global_state::GlobalState;
use render_chapters::{render_articles_list, render_content_for_article};

#[component]
pub fn Body() -> impl IntoView {
    let GlobalState {
        btc_alignment_on_left,
        ..
    } = use_context().unwrap();

    view! {
      <Columns>
        <VerticalChunk>
          <h1 class="leading-[1] text-[2.5rem] font-merriweather mb-[2.3rem] sm:mb-[2.6rem] gap-4 flex justify-center items-center font-lora">
            //<img src="/images/table_of_contents.svg" class="w-[40px] sm:w-[60px]"/>
            <TitleSVG />
            //<img src="/images/table_of_contents.svg" class="flip-y w-[40px] sm:w-[60px]"/>
          </h1>

          {render_content_for_article!(
              "chapters", r#"
            <Title label="Chapters"/>
        "#
          )}
          <ul class="p-0 leading-9 lg:leading-10 text-2xl lg:text-3xl">
            {render_articles_list!("chapters")}
          </ul> <Spacer/>
          {render_content_for_article!(
              "bootcamps", r#"
            <Title label="Bootcamps"/>
          "#
          )}
          <ul
            class="p-0 leading-9 lg:leading-10 text-2xl lg:text-3xl"
            class=("text-right", move || !btc_alignment_on_left.get())
          >
            {render_articles_list!("bootcamps")}
          </ul>
        </VerticalChunk>
      </Columns>
    }
}

#[component]
pub fn Title(label: &'static str) -> impl IntoView {
    view! {
      <h1 class="text-[2rem] sm:text-[2.1rem] font-baskerville-italic mb-5 flex justify-between items-center">
        <img src="/images/title_line.svg" class="w-[30%] sm:w-36"/>
        {label}
        <img src="/images/title_line.svg" class="rotate-180 w-[30%] sm:w-36"/>
      </h1>
    }
}

#[component]
pub fn MenuItem(
    href: &'static str,
    article_type: &'static str,
    label: &'static str,
    #[prop(optional)] on_mobile: &'static str,
) -> impl IntoView {
    view! {
      <CustomLink
        base_href="/article/"
        href=href
        class="flex items-baseline justify-between"
      >
        <span class="block">{article_type}</span>
        <span class="dots"></span>
        <span class="sm:hidden">{if on_mobile == "" { label } else { on_mobile }}</span>
        <span class="hidden sm:block">{label}</span>
      </CustomLink>
    }
}

#[component]
pub fn TitleSVG() -> impl IntoView {
    view! {
      <svg class="max-w-[80%]" xmlns="http://www.w3.org/2000/svg" width="350px" viewBox="0 0 333.92395 30.203999">
        <path aria-label="Table of Contents" d="M 9.83,4.97 L 3.64,5.18 2.38,12.71 H 0 L 0.61,3.06 H 23.04 L 23.47,12.71 H 21.2 L 20.02,5.18 13.79,4.97 V 27.68 L 18.9,28.04 V 29.81 H 5.15 V 28.04 L 9.83,27.68 Z M 27.42,24.41 Q 27.42,21.02 30.8,19.44 Q 34.19,17.82 39.37,17.71 V 16.7 Q 39.37,15.01 39.01,14.04 Q 38.65,13.07 37.75,12.64 Q 36.89,12.17 35.27,12.17 Q 33.43,12.17 31.96,12.71 Q 30.52,13.21 29.08,13.97 L 28.18,12.1 Q 28.64,11.7 29.9,11.05 Q 31.16,10.4 32.82,9.9 Q 34.48,9.4 36.13,9.4 Q 38.69,9.4 40.16,10.12 Q 41.68,10.8 42.32,12.35 Q 42.97,13.9 42.97,16.49 V 28.04 H 45.1 V 29.63 Q 44.38,29.81 43.26,29.99 Q 42.18,30.17 41.35,30.17 Q 40.34,30.17 39.98,29.84 Q 39.62,29.56 39.62,28.51 V 27.32 Q 38.54,28.4 36.96,29.3 Q 35.41,30.2 33.43,30.2 Q 30.84,30.2 29.11,28.73 Q 27.42,27.22 27.42,24.41 Z M 34.98,27.76 Q 35.92,27.76 37.14,27.18 Q 38.36,26.6 39.37,25.81 V 19.62 Q 35.45,19.62 33.43,20.84 Q 31.45,22.03 31.45,23.98 Q 31.45,25.92 32.39,26.86 Q 33.36,27.76 34.98,27.76 Z M 47.42,1.01 L 53.18,0.32 H 53.26 L 54.08,0.86 V 11.7 Q 55.16,10.66 56.64,9.97 Q 58.15,9.29 59.99,9.29 Q 62.15,9.29 63.91,10.33 Q 65.71,11.38 66.76,13.57 Q 67.84,15.77 67.84,19.12 Q 67.84,22.21 66.54,24.73 Q 65.28,27.25 62.9,28.73 Q 60.53,30.2 57.4,30.2 Q 54.91,30.2 52.86,29.74 Q 50.81,29.23 50.38,28.84 V 2.92 L 47.42,2.52 Z M 58.51,11.84 Q 57.14,11.84 55.96,12.49 Q 54.8,13.1 54.08,13.9 V 27.14 Q 54.3,27.65 55.31,27.97 Q 56.35,28.26 57.65,28.26 Q 60.42,28.26 62.11,26.03 Q 63.8,23.8 63.8,19.51 Q 63.8,15.66 62.33,13.75 Q 60.85,11.84 58.51,11.84 Z M 75.1,2.95 L 72.18,2.56 V 1.01 L 77.98,0.32 H 78.05 L 78.84,0.86 V 27.76 L 81.9,28.04 V 29.81 H 72.18 V 28.04 L 75.1,27.72 Z M 94.81,30.2 Q 90.46,30.2 88.15,27.36 Q 85.88,24.52 85.88,19.76 Q 85.88,16.63 87.07,14.26 Q 88.26,11.88 90.38,10.58 Q 92.54,9.29 95.28,9.29 Q 98.63,9.29 100.46,11.16 Q 102.34,13 102.44,16.45 Q 102.44,18.65 102.19,19.8 H 89.81 Q 89.88,23.36 91.46,25.56 Q 93.05,27.72 95.96,27.72 Q 97.4,27.72 98.95,27.22 Q 100.54,26.68 101.4,25.96 L 102.08,27.54 Q 101.04,28.62 98.99,29.41 Q 96.94,30.2 94.81,30.2 Z M 98.48,17.89 Q 98.59,17.14 98.59,16.34 Q 98.56,14 97.58,12.67 Q 96.65,11.3 94.6,11.3 Q 92.51,11.3 91.25,12.82 Q 90.02,14.33 89.84,17.89 Z M 117.07,19.76 Q 117.07,16.52 118.4,14.15 Q 119.74,11.77 121.9,10.55 Q 124.09,9.29 126.58,9.29 Q 131.11,9.29 133.38,12.17 Q 135.68,15.05 135.68,19.73 Q 135.68,23 134.35,25.38 Q 133.02,27.76 130.82,28.98 Q 128.66,30.2 126.18,30.2 Q 121.64,30.2 119.34,27.32 Q 117.07,24.44 117.07,19.76 Z M 126.4,28.15 Q 128.88,28.15 130.21,26.1 Q 131.54,24.05 131.54,20.02 Q 131.54,16.02 130.32,13.68 Q 129.1,11.34 126.4,11.34 Q 123.91,11.34 122.54,13.39 Q 121.21,15.44 121.21,19.48 Q 121.21,23.47 122.47,25.81 Q 123.73,28.15 126.4,28.15 Z M 143.34,12.2 H 139.96 V 10.55 L 143.34,9.86 V 8.17 Q 143.34,5.94 144.49,4.07 Q 145.68,2.2 147.55,1.12 Q 149.46,0 151.58,0 Q 152.81,0 153.71,0.32 V 3.92 Q 153.38,3.64 152.41,3.38 Q 151.48,3.1 150.32,3.1 Q 148.63,3.1 147.84,4 Q 147.08,4.86 147.08,7.02 V 9.86 H 152.05 V 12.2 H 147.08 V 27.76 L 151.08,28.04 V 29.81 H 140.28 V 28.04 L 143.34,27.72 Z M 178.38,30.2 Q 174.42,30.2 171.61,28.51 Q 168.84,26.78 167.4,23.69 Q 165.96,20.59 165.96,16.42 Q 165.96,12.38 167.65,9.29 Q 169.34,6.19 172.3,4.5 Q 175.25,2.77 178.92,2.77 Q 181.69,2.77 185.58,3.64 L 186.77,3.85 186.41,10.33 H 184 L 183.1,5.76 Q 182.59,5.33 181.33,5.04 Q 180.11,4.72 178.31,4.72 Q 176,4.72 174.2,6.05 Q 172.4,7.34 171.36,9.9 Q 170.35,12.46 170.35,16.02 Q 170.35,19.4 171.25,22.18 Q 172.15,24.95 173.99,26.6 Q 175.82,28.22 178.52,28.22 Q 180.43,28.22 181.44,27.9 Q 182.48,27.58 183.2,26.96 L 184.72,23.44 186.98,23.72 186.12,28.84 Q 185.36,28.87 183.89,29.34 Q 182.59,29.74 181.37,29.95 Q 180.18,30.2 178.38,30.2 Z M 191.4,19.76 Q 191.4,16.52 192.73,14.15 Q 194.06,11.77 196.22,10.55 Q 198.42,9.29 200.9,9.29 Q 205.44,9.29 207.71,12.17 Q 210.01,15.05 210.01,19.73 Q 210.01,23 208.68,25.38 Q 207.35,27.76 205.15,28.98 Q 202.99,30.2 200.51,30.2 Q 195.97,30.2 193.67,27.32 Q 191.4,24.44 191.4,19.76 Z M 200.72,28.15 Q 203.21,28.15 204.54,26.1 Q 205.87,24.05 205.87,20.02 Q 205.87,16.02 204.65,13.68 Q 203.42,11.34 200.72,11.34 Q 198.24,11.34 196.87,13.39 Q 195.54,15.44 195.54,19.48 Q 195.54,23.47 196.8,25.81 Q 198.06,28.15 200.72,28.15 Z M 216.95,12.6 L 214.36,11.95 V 10.01 L 219.5,9.36 H 219.61 L 220.37,10.01 V 11.52 L 220.33,12.38 Q 221.63,11.23 223.75,10.3 Q 225.88,9.36 227.82,9.36 Q 230.09,9.36 231.31,10.22 Q 232.57,11.09 233.08,12.89 Q 233.58,14.65 233.58,17.68 V 27.76 L 236.1,28.01 V 29.81 H 227.57 V 28.04 L 229.8,27.76 V 17.64 Q 229.8,15.52 229.51,14.33 Q 229.22,13.1 228.43,12.53 Q 227.64,11.92 226.13,11.92 Q 224.83,11.92 223.36,12.56 Q 221.88,13.21 220.73,14.08 V 27.72 L 223.18,28.04 V 29.81 H 214.68 V 28.04 L 216.95,27.72 Z M 246.35,30.17 Q 244.3,30.17 243.29,29.23 Q 242.28,28.3 242.28,26.03 V 12.2 H 239.69 V 10.58 Q 239.87,10.55 240.7,10.33 Q 241.52,10.12 241.81,9.94 Q 242.39,9.61 242.71,8.64 Q 243,7.96 243.4,6.44 Q 243.79,4.93 243.86,4.64 H 245.99 L 246.06,9.83 H 252.07 V 12.2 H 246.06 V 23.26 Q 246.06,25.31 246.17,26.14 Q 246.31,26.96 246.71,27.22 Q 247.14,27.47 248.15,27.47 Q 249.12,27.47 250.27,27.22 Q 251.46,26.96 252.11,26.68 L 252.65,28.3 Q 251.78,28.94 249.84,29.56 Q 247.93,30.17 246.35,30.17 Z M 265.2,30.2 Q 260.84,30.2 258.54,27.36 Q 256.27,24.52 256.27,19.76 Q 256.27,16.63 257.46,14.26 Q 258.65,11.88 260.77,10.58 Q 262.93,9.29 265.67,9.29 Q 269.02,9.29 270.85,11.16 Q 272.72,13 272.83,16.45 Q 272.83,18.65 272.58,19.8 H 260.2 Q 260.27,23.36 261.85,25.56 Q 263.44,27.72 266.35,27.72 Q 267.79,27.72 269.34,27.22 Q 270.92,26.68 271.79,25.96 L 272.47,27.54 Q 271.43,28.62 269.38,29.41 Q 267.32,30.2 265.2,30.2 Z M 268.87,17.89 Q 268.98,17.14 268.98,16.34 Q 268.94,14 267.97,12.67 Q 267.04,11.3 264.98,11.3 Q 262.9,11.3 261.64,12.82 Q 260.41,14.33 260.23,17.89 Z M 279.59,12.6 L 277,11.95 V 10.01 L 282.14,9.36 H 282.25 L 283.01,10.01 V 11.52 L 282.97,12.38 Q 284.27,11.23 286.39,10.3 Q 288.52,9.36 290.46,9.36 Q 292.73,9.36 293.95,10.22 Q 295.21,11.09 295.72,12.89 Q 296.22,14.65 296.22,17.68 V 27.76 L 298.74,28.01 V 29.81 H 290.21 V 28.04 L 292.44,27.76 V 17.64 Q 292.44,15.52 292.15,14.33 Q 291.86,13.1 291.07,12.53 Q 290.28,11.92 288.77,11.92 Q 287.47,11.92 286,12.56 Q 284.52,13.21 283.37,14.08 V 27.72 L 285.82,28.04 V 29.81 H 277.32 V 28.04 L 279.59,27.72 Z M 308.99,30.17 Q 306.94,30.17 305.93,29.23 Q 304.92,28.3 304.92,26.03 V 12.2 H 302.33 V 10.58 Q 302.51,10.55 303.34,10.33 Q 304.16,10.12 304.45,9.94 Q 305.03,9.61 305.35,8.64 Q 305.64,7.96 306.04,6.44 Q 306.43,4.93 306.5,4.64 H 308.63 L 308.7,9.83 H 314.71 V 12.2 H 308.7 V 23.26 Q 308.7,25.31 308.81,26.14 Q 308.95,26.96 309.35,27.22 Q 309.78,27.47 310.79,27.47 Q 311.76,27.47 312.91,27.22 Q 314.1,26.96 314.75,26.68 L 315.29,28.3 Q 314.42,28.94 312.48,29.56 Q 310.57,30.17 308.99,30.17 Z M 322.33,26.93 Q 322.69,27.47 323.84,27.9 Q 325.03,28.33 326.33,28.33 Q 328.38,28.33 329.35,27.54 Q 330.32,26.75 330.32,25.34 Q 330.32,24.37 329.71,23.65 Q 329.1,22.93 327.98,22.39 Q 326.9,21.82 324.85,21.02 Q 322.08,19.98 320.78,18.54 Q 319.52,17.06 319.52,14.69 Q 319.52,13.1 320.46,11.92 Q 321.43,10.73 323.09,10.08 Q 324.78,9.4 326.83,9.4 Q 328.49,9.4 329.71,9.65 Q 330.97,9.9 331.73,10.15 Q 332.48,10.4 332.74,10.48 V 14.94 H 330.54 L 329.82,12.28 Q 329.6,11.84 328.67,11.56 Q 327.73,11.23 326.58,11.23 Q 324.96,11.23 323.95,11.95 Q 322.98,12.67 322.98,13.9 Q 322.98,14.98 323.48,15.7 Q 324.02,16.38 324.82,16.81 Q 325.61,17.24 326.98,17.82 L 327.7,18.11 Q 329.71,18.9 330.97,19.62 Q 332.23,20.34 333.06,21.53 Q 333.92,22.68 333.92,24.37 Q 333.92,27.04 331.84,28.62 Q 329.78,30.2 326.18,30.2 Q 324.42,30.2 322.48,29.74 Q 320.57,29.23 319.34,28.73 V 24.12 H 321.68 Z"/>
      </svg>

    }
}
