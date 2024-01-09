use std::{rc::Rc, sync::Arc, time::Duration};

use crate::{constants::GREEN_DIV_HEIGHT, global_state::GlobalState};
use leptos::{
    ev::{click, resize},
    html::Div,
    *,
};
use leptos_router::{use_location, use_navigate, NavigateOptions, State};
use leptos_use::use_event_listener;
use web_sys::{MouseEvent, ScrollBehavior, ScrollIntoViewOptions};

#[component]
pub fn Solution(cx: Scope, children: Children) -> impl IntoView {
    let set_solution_open = use_context::<WriteSignal<bool>>(cx).unwrap();
    let solution_open = use_context::<ReadSignal<bool>>(cx).unwrap();
    let (content_height, set_content_height) = create_signal(cx, 0);
    let node_ref = create_node_ref::<Div>(cx);
    let button = create_node_ref::<Div>(cx);

    create_effect(cx, move |_| {
        if node_ref().is_some() {
            if solution_open() {
                if node_ref().unwrap().offset_height() == 0 {
                    set_timeout(
                        move || set_content_height(node_ref().unwrap().offset_height()),
                        Duration::from_secs(1),
                    )
                } else {
                    set_content_height(node_ref().unwrap().offset_height());
                }
            } else {
                set_content_height(0)
            }
        }

        set_timeout(
            move || {
                if node_ref().is_some() {
                    if solution_open() {
                        set_content_height(node_ref().unwrap().offset_height())
                    } else {
                        set_content_height(0)
                    }
                }
            },
            Duration::from_secs(1),
        )
    });

    create_effect(cx, move |_| {
        let _ = use_event_listener(cx, window(), resize, move |_| {
            if node_ref().is_some() {
                if solution_open() {
                    set_content_height(node_ref().unwrap().offset_height())
                } else {
                    set_content_height(0)
                }
            }
        });
    });

    let (bot_div, set_bot_div) = create_signal(cx, true);
    create_effect(cx, move |_| {
        if solution_open() {
            set_timeout(move || set_bot_div(false), Duration::from_secs(1))
        } else {
            set_timeout(move || set_bot_div(true), Duration::from_secs(1))
        }
    });

    let (transition, set_transition) = create_signal(cx, false);

    let navigate = use_navigate(cx);
    let GlobalState { show_areas, .. } = use_context::<GlobalState>(cx).unwrap();

    view! { cx,
      <div node_ref=button class="px-4 my-5 relative col-start-2">
        <SolutionSVG on_click=move |_| {

            // Get the element's bottom position relative to the document
            let element_pos = window().inner_height().unwrap().as_f64().unwrap() -  button().unwrap().get_bounding_client_rect().bottom();

            let should_scroll_to_button_first = element_pos > GREEN_DIV_HEIGHT as f64 + 40_f64+ 56_f64 ; // empty div beneath + solution button margin bot + padding bottom of page

            if solution_open() && should_scroll_to_button_first {
              let mut options = ScrollIntoViewOptions::new();
              options.behavior(ScrollBehavior::Smooth);
              document().get_element_by_id("exo").unwrap().scroll_into_view_with_scroll_into_view_options(&options);
            }
            set_transition(true);
            set_timeout(move || set_transition(false), Duration::from_millis(1100));
            let options = NavigateOptions {
              resolve: true,
              replace: false,
              scroll: false,
              state: State(None)
            };
            if let Ok(search_params) = window().location().search() {
              let mut new_url = String::new();
              if search_params.contains("&opened=true") {
                new_url = window().location().pathname().unwrap() + &search_params.replace("&opened=true", "&opened=false")
              }else if search_params.contains("&opened=false"){
                new_url = window().location().pathname().unwrap() + &search_params.replace("&opened=false", "&opened=true")
              }else if &search_params == ""{
                new_url = window().location().pathname().unwrap() + "?tab=0" + &format!("&opened={}", !solution_open())
              }else{
                new_url = window().location().pathname().unwrap() + &search_params + &format!("&opened={}", !solution_open())

              }
              let _ = navigate(&new_url, options);
            }
            set_solution_open(!solution_open())

        }/>
      </div>
      <div
        class="col-start-2 transition-[height] duration-1000 overflow-y-clip relative"
        class=("pointer-events-none", move || !solution_open())
        class=("animated-height-full", move || solution_open())
        style=move || {
            format!("height: {}px", content_height() + if solution_open() { 40 } else { 0 })
        }
      >

        <div
          node_ref=node_ref
          class=("-translate-y-full", move || !solution_open())
          class=("duration-1000", move || transition())
          class=("transition-all", move || transition())
        >
          {children(cx)}
        </div>

      </div>
       <Show fallback=|_| () when=move || !solution_open() || bot_div()>
        <div style=format!("height: {}px; background-color: {}", GREEN_DIV_HEIGHT, if show_areas() { "#00440050" } else { "" })>
        </div>
      </Show>
    }
}

#[component]
pub fn SolutionSVG<F>(cx: Scope, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let solution_open = use_context::<ReadSignal<bool>>(cx).unwrap();
    let button = create_node_ref(cx);

    let on_click_arc: Arc<F> = Arc::new(on_click);
    create_effect(cx, move |_| {
        //let on_click_clone = on_click.clone();
        let cloned_on_click = Arc::clone(&on_click_arc);
        let _ = use_event_listener(cx, button, click, move |e| {
            cloned_on_click(e);
        });
    });

    view! { cx,
      <div
        id="solution-button"
        node_ref = button
        class="column solution_button_div cursor-pointer mb-12"
      >
        <svg class="mx-auto h-[37px] overflow-visible">
          <g class="solution_button_svg">
            <rect
              id="solution_button_focus_rect"
              class="focus_alpha_fill"
              x="-7"
              y="-7"
              width="123"
              height="50"
            ></rect>

            <rect
              id="solution_button_focus_rect"
              class="solution_button_transition"
              class=("active_solution_button_rect", move || !solution_open())
              class=("inactive_solution_button_rect", move || solution_open())

              width="109"
              height="36"
            ></rect>

            <path
              id="solution_button_lip"
              class="solution_button_transition"
              class=("active_solution_button_lip", move || !solution_open())
              class=("inactive_solution_button_lip", move || solution_open())
              d="M 0 10 v -10 h 109 v 10 M 0 26 v 10 h 109 v -10"
            ></path>

            <g
              id="solution_button_finger_pair"
              class="solution_button_transition"
              class=("active_solution_button_hands", move || !solution_open())
              class=("inactive_solution_button_hands", move || solution_open())
            >
              <SVGLeftFinger transform="translate(101.5, 18)"/>
              <SVGLeftFinger transform="scale(-1, 1) translate(-8, 20)"/>
              <SolutionText/>
            </g>
          </g>
        </svg>
      </div>
    }
}

#[component]
fn SVGLeftFinger(cx: Scope, #[prop(default = "")] transform: &'static str) -> impl IntoView {
    view! { cx,
      <g transform=transform>
        <g id="finger_pointing_left" transform="scale(0.0386)">
          <path d="M 659,423 C 698,401 747,260 782,350 C 806,442 905,378 971,392 C 1062,405 1072,291 1117,239 C 1202,252 1300,297 1379,240 C 1424,229 1492,138 1525,174 C 1540,269 1622,283 1703,276 C 1830,277 1958,275 2084,258 C 2062,231 1961,259 1911,252 C 1844,247 1762,260 1705,225 C 1724,184 1776,161 1760,100 C 1757,53 1726,-86 1745,-71 C 1746,4 1758,84 1740,155 C 1699,88 1757,12 1724,-57 C 1689,-139 1672,-228 1634,-309 C 1623,-341 1550,-339 1617,-364 C 1694,-405 1779,-360 1860,-368 C 1890,-358 2000,-376 1911,-382 C 1825,-373 1743,-408 1657,-401 C 1613,-377 1473,-381 1533,-303 C 1439,-307 1342,-353 1250,-316 C 1173,-304 1094,-314 1019,-298 C 905,-290 786,-318 679,-270 C 587,-245 495,-219 402,-200 C 274,-148 149,-88 20,-38 C -35,27 72,104 139,76 C 213,53 284,40 358,11 C 422,-1 501,-6 562,6 C 527,72 453,138 474,218 C 495,275 581,268 543,344 C 544,407 610,418 659,423 Z"></path>
          <path
            class="pointing_finger_backhand"
            d="M 986,226 C 961,189 923,108 943,79 C 963,116 961,175 990,202 C 960,134 958,58 951,-13 C 948,25 939,62 936,0 C 942,-63 906,35 910,-32 C 899,-13 908,59 892,-3 C 887,-75 875,40 868,-20 C 854,-119 858,24 877,53 C 891,101 872,121 867,62 C 846,0 822,-112 898,-138 C 969,-178 813,-156 813,-121 C 796,-63 804,-227 790,-136 C 793,-94 775,-53 779,-124 C 756,-218 774,4 749,-110 C 733,-155 794,-289 721,-211 C 736,-269 663,-141 672,-205 C 632,-157 603,-169 597,-100 C 585,-93 599,-157 574,-103 C 539,-24 453,-40 380,-29 C 302,-4 221,16 145,47 C 92,56 183,21 115,36 C 73,47 34,14 102,17 C 205,-14 -11,30 53,-33 C 204,-70 331,-172 486,-193 C 579,-209 666,-245 757,-268 C 836,-267 915,-264 995,-275 C 1075,-285 1155,-286 1235,-273 C 1277,-268 1273,-310 1310,-271 C 1396,-260 1254,-295 1327,-301 C 1402,-297 1477,-289 1551,-281 C 1589,-266 1572,-207 1590,-170 C 1572,-189 1528,-280 1556,-206 C 1576,-173 1575,-131 1543,-185 C 1526,-175 1625,-69 1567,-44 C 1557,-99 1554,-88 1556,-43 C 1537,-73 1517,-159 1499,-151 C 1523,-127 1546,-15 1505,-98 C 1498,-133 1432,-161 1466,-106 C 1485,-102 1552,-24 1522,-5 C 1505,-28 1503,39 1488,-14 C 1493,22 1489,88 1477,11 C 1471,-20 1453,-85 1436,-103 C 1447,-61 1480,9 1461,37 C 1458,-12 1429,-22 1447,31 C 1463,61 1424,170 1434,87 C 1438,55 1434,4 1424,65 C 1418,107 1391,143 1406,78 C 1422,-36 1347,225 1369,101 C 1386,13 1306,248 1330,143 C 1354,101 1335,84 1321,135 C 1312,168 1263,201 1294,143 C 1277,178 1232,223 1259,156 C 1219,220 1265,67 1232,158 C 1221,194 1183,212 1213,161 C 1226,128 1223,81 1207,144 C 1202,173 1177,205 1184,156 C 1129,231 1186,102 1156,105 C 1181,145 1102,232 1137,143 C 1153,116 1117,22 1125,104 C 1150,146 1082,220 1115,137 C 1121,84 1092,101 1098,145 C 1069,213 1082,122 1087,126 C 1063,184 1036,168 1032,160 C 1029,183 1006,221 986,226 Z"
          ></path>
          <path d="M 1543,-126 C 1542,-165 1519,-131 1543,-126 Z"></path>
          <path d="M 1623,-8 C 1618,-32 1643,-8 1623,-8 Z"></path>
          <path d="M 1546,-312 C 1524,-337 1571,-308 1546,-312 Z"></path>
          <path d="M 1964,207 C 1993,175 1982,98 2009,80 C 2023,117 2015,181 2024,194 C 2035,146 2046,120 2023,99 C 2022,64 2002,18 1992,79 C 1986,111 1964,212 1962,121 C 1954,46 1953,-32 1936,-106 C 1923,-47 1923,41 1920,111 C 1919,143 1917,219 1928,144 C 1938,105 1958,-13 1942,8 C 1954,72 1943,152 1964,207 Z"></path>
          <path d="M 1788,202 C 1812,115 1804,21 1790,-66 C 1786,15 1794,99 1783,181 C 1790,179 1776,207 1788,202 Z"></path>
          <path d="M 1833,196 C 1857,124 1842,45 1840,-29 C 1821,-100 1839,123 1833,65 C 1838,103 1823,179 1833,196 Z"></path>
          <path d="M 2100,195 C 2100,130 2103,11 2058,-19 C 2054,17 2030,145 2059,47 C 2096,26 2082,180 2100,195 Z"></path>
          <path d="M 1875,188 C 1894,144 1883,-15 1872,4 C 1883,61 1869,148 1875,188 Z"></path>
          <path d="M 1740,-107 C 1743,-138 1682,-278 1711,-178 C 1722,-157 1725,-121 1740,-107 Z"></path>
          <g class="pointing_finger_knuckle_zone">
            <path d="M 990,352 C 952,356 956,306 985,331 C 1021,281 1052,364 990,352 Z"></path>
            <path d="M 908,352 C 879,324 851,222 893,312 C 893,321 928,348 908,352 Z"></path>
            <path d="M 876,345 C 849,307 905,342 876,345 Z"></path>
            <path d="M 946,344 C 929,305 974,344 946,344 Z"></path>
            <path d="M 852,343 C 835,297 872,356 852,343 Z"></path>
            <path d="M 834,314 C 815,262 860,325 834,314 Z"></path>
            <path d="M 1046,303 C 1055,281 1056,322 1046,303 Z"></path>
            <path d="M 907,301 C 879,279 842,146 887,235 C 889,243 930,326 907,301 Z"></path>
            <path d="M 847,281 C 847,264 860,293 847,281 Z"></path>
            <path d="M 927,273 C 896,248 868,115 910,214 C 908,221 954,288 927,273 Z"></path>
            <path d="M 943,259 C 922,217 882,152 894,111 C 907,154 946,235 943,259 Z"></path>
            <path d="M 632,247 C 621,208 567,121 601,213 C 626,255 600,272 588,218 C 576,188 538,63 582,150 C 590,144 565,8 582,90 C 600,146 633,206 641,262 Z"></path>
            <path d="M 797,253 C 772,230 738,92 781,188 C 780,193 820,268 797,253 Z"></path>
            <path d="M 712,252 C 692,205 732,259 712,252 Z"></path>
            <path d="M 575,247 C 559,212 526,154 540,126 C 547,165 581,216 575,247 Z"></path>
            <path d="M 761,235 C 736,203 720,99 750,190 C 749,195 782,261 761,235 Z"></path>
            <path d="M 733,228 C 710,200 713,147 731,210 C 733,217 747,255 733,228 Z"></path>
            <path d="M 537,231 C 517,207 505,108 531,185 C 536,189 561,258 537,231 Z"></path>
            <path d="M 965,234 C 934,194 888,62 912,55 C 924,114 959,189 965,234 Z"></path>
            <path d="M 807,213 C 785,175 752,79 767,72 C 775,114 816,187 807,213 Z"></path>
            <path d="M 645,198 C 622,154 587,67 601,36 C 603,89 648,157 645,198 Z"></path>
            <path d="M 817,178 C 805,154 772,24 803,109 C 805,126 835,168 817,178 Z"></path>
            <path d="M 658,164 C 649,140 602,34 631,48 C 638,79 670,156 658,164 Z"></path>
            <path d="M 749,125 C 740,56 771,173 749,125 Z"></path>
            <path d="M 679,129 C 654,105 624,-39 659,49 C 664,72 688,118 679,129 Z"></path>
            <path d="M 827,112 C 804,88 798,-1 823,73 C 823,77 842,123 827,112 Z"></path>
            <path d="M 846,91 C 834,61 829,-24 850,54 C 853,59 858,122 846,91 Z"></path>
            <path d="M 693,97 C 682,57 640,-9 663,-39 C 671,2 701,65 693,97 Z"></path>
            <path d="M 707,40 C 678,8 682,-114 703,-12 C 704,-6 727,67 707,40 Z"></path>
            <path d="M 920,42 C 922,24 931,31 920,42 Z"></path>
            <path d="M 730,11 C 700,-9 701,-148 721,-54 C 718,-40 747,13 730,11 Z"></path>
            <path d="M 638,0 C 620,-32 652,-38 638,0 Z"></path>
            <path d="M 748,-9 C 730,-34 720,-181 741,-90 C 741,-69 766,-15 748,-9 Z"></path>
            <path d="M 773,-39 C 757,-71 794,-31 773,-39 Z"></path>
          </g>
          <g class="pointing_finger_cuff_zone">
            <path d="M 1629,212 C 1594,180 1643,115 1644,183 C 1641,187 1642,222 1629,212 Z"></path>
            <path d="M 1680,190 C 1669,135 1704,173 1680,190 Z"></path>
            <path d="M 1575,183 C 1551,126 1628,180 1585,164 C 1582,167 1583,189 1575,183 Z"></path>
            <path d="M 1611,51 C 1611,18 1679,1 1631,46 C 1628,40 1612,67 1611,51 Z"></path>
            <path d="M 617,-77 C 607,-104 641,-69 617,-77 Z"></path>
            <path d="M 638,-115 C 630,-137 660,-112 638,-115 Z"></path>
            <path d="M 665,-119 C 656,-155 693,-119 665,-119 Z"></path>
          </g>
        </g>
      </g>
    }
}

#[component]
fn SolutionText(cx: Scope) -> impl IntoView {
    view! { cx,
      <path
        style="transform: translate(8.5%, 30%)"
        d="M2.232 7.808C2.232 7.456 2.296 7.13067 2.424 6.832C2.56267 6.53333 2.74933 6.27733 2.984 6.064C3.21867 5.84 3.496 5.66933 3.816 5.552C4.14667 5.424 4.504 5.36 4.888 5.36C5.4 5.36 5.82667 5.488 6.168 5.744C6.50933 6 6.68 6.33067 6.68 6.736C6.68 6.96 6.616 7.136 6.488 7.264C6.37067 7.392 6.20533 7.456 5.992 7.456C5.74667 7.456 5.57067 7.38667 5.464 7.248C5.35733 7.09867 5.26667 6.93867 5.192 6.768C5.11733 6.58667 5.03733 6.42133 4.952 6.272C4.86667 6.12267 4.72267 6.048 4.52 6.048C4.28533 6.048 4.088 6.14933 3.928 6.352C3.768 6.55467 3.688 6.80533 3.688 7.104C3.688 7.456 3.78933 7.78667 3.992 8.096C4.20533 8.39467 4.44 8.70933 4.696 9.04C4.952 9.37067 5.18133 9.73333 5.384 10.128C5.59733 10.5227 5.704 10.9867 5.704 11.52C5.704 11.8827 5.624 12.224 5.464 12.544C5.31467 12.864 5.10667 13.1467 4.84 13.392C4.584 13.6267 4.28533 13.8133 3.944 13.952C3.60267 14.0907 3.25067 14.16 2.888 14.16C2.248 14.16 1.736 14.0213 1.352 13.744C0.978667 13.4667 0.792 13.104 0.792 12.656C0.792 12.3467 0.861333 12.1067 1 11.936C1.14933 11.7547 1.34667 11.664 1.592 11.664C2.05067 11.664 2.312 11.9787 2.376 12.608C2.408 12.9493 2.46133 13.1787 2.536 13.296C2.62133 13.4027 2.78667 13.456 3.032 13.456C3.416 13.456 3.72 13.344 3.944 13.12C4.168 12.8853 4.28 12.5653 4.28 12.16C4.28 11.7333 4.17333 11.3547 3.96 11.024C3.74667 10.6933 3.512 10.3733 3.256 10.064C3 9.744 2.76533 9.408 2.552 9.056C2.33867 8.704 2.232 8.288 2.232 7.808ZM10.0858 14.16C9.72308 14.16 9.38175 14.0907 9.06175 13.952C8.75242 13.8027 8.48042 13.5893 8.24575 13.312C8.01108 13.0347 7.82442 12.6987 7.68575 12.304C7.54708 11.8987 7.47775 11.4453 7.47775 10.944C7.47775 10.2507 7.60575 9.568 7.86175 8.896C8.12842 8.224 8.47508 7.62667 8.90175 7.104C9.32842 6.58133 9.81375 6.16 10.3578 5.84C10.9124 5.52 11.4831 5.36 12.0698 5.36C12.4538 5.36 12.8058 5.43467 13.1258 5.584C13.4564 5.72267 13.7338 5.93067 13.9578 6.208C14.1924 6.47467 14.3738 6.8 14.5018 7.184C14.6404 7.55733 14.7098 7.97333 14.7098 8.432C14.7098 9.104 14.5764 9.78133 14.3098 10.464C14.0431 11.1467 13.6911 11.76 13.2538 12.304C12.8271 12.848 12.3364 13.296 11.7818 13.648C11.2271 13.9893 10.6618 14.16 10.0858 14.16ZM10.1818 13.36C10.5444 13.36 10.9018 13.1733 11.2538 12.8C11.6164 12.4267 11.9364 11.9573 12.2138 11.392C12.5018 10.816 12.7311 10.192 12.9018 9.52C13.0831 8.848 13.1738 8.208 13.1738 7.6C13.1738 7.03467 13.0724 6.64533 12.8698 6.432C12.6778 6.21867 12.4111 6.112 12.0698 6.112C11.8351 6.112 11.5951 6.19733 11.3498 6.368C11.1151 6.528 10.8911 6.752 10.6778 7.04C10.4644 7.328 10.2618 7.65867 10.0698 8.032C9.88842 8.40533 9.72842 8.8 9.58975 9.216C9.45108 9.632 9.34442 10.0587 9.26975 10.496C9.19508 10.9227 9.15775 11.3333 9.15775 11.728C9.15775 12.3253 9.25375 12.7467 9.44575 12.992C9.63775 13.2373 9.88308 13.36 10.1818 13.36ZM18.3968 2.944C18.4501 2.752 18.5034 2.56 18.5568 2.368C18.6101 2.16533 18.6368 2.016 18.6368 1.92C18.6368 1.54667 18.2314 1.36 17.4208 1.36L17.5488 0.879999L20.4768 0.735999L17.6128 11.248C17.5274 11.536 17.4634 11.792 17.4208 12.016C17.3781 12.2293 17.3568 12.4107 17.3568 12.56C17.3568 12.656 17.3834 12.7413 17.4368 12.816C17.4901 12.8907 17.5754 12.928 17.6928 12.928C17.8634 12.928 18.0448 12.8587 18.2368 12.72C18.4288 12.5707 18.6101 12.4 18.7808 12.208C18.9621 12.0053 19.1221 11.8027 19.2608 11.6C19.3994 11.3867 19.5008 11.2213 19.5648 11.104L20.0448 11.376C19.8848 11.7387 19.6768 12.0907 19.4208 12.432C19.1648 12.7627 18.8928 13.056 18.6048 13.312C18.3168 13.568 18.0234 13.776 17.7248 13.936C17.4368 14.0853 17.1701 14.16 16.9248 14.16C16.5514 14.16 16.2741 14.08 16.0928 13.92C15.9221 13.76 15.8368 13.4933 15.8368 13.12C15.8368 12.9067 15.8688 12.6187 15.9328 12.256C15.9968 11.8933 16.1088 11.3973 16.2688 10.768L18.3968 2.944ZM23.1884 13.008C23.359 13.008 23.583 12.8907 23.8604 12.656C24.1377 12.4107 24.431 12.0907 24.7404 11.696C25.0497 11.2907 25.3484 10.832 25.6364 10.32C25.935 9.808 26.1857 9.27467 26.3884 8.72L27.5724 5.52H29.0604L27.8764 9.184C27.7057 9.71733 27.5617 10.176 27.4444 10.56C27.327 10.9333 27.231 11.2533 27.1564 11.52C27.0817 11.776 27.0284 11.9893 26.9964 12.16C26.9644 12.32 26.9484 12.448 26.9484 12.544C26.9484 12.8213 27.0444 12.96 27.2364 12.96C27.343 12.96 27.471 12.912 27.6204 12.816C27.7697 12.7093 27.919 12.5707 28.0684 12.4C28.2284 12.2187 28.3777 12.0213 28.5164 11.808C28.6657 11.584 28.7937 11.3493 28.9004 11.104L29.3804 11.344C29.2417 11.696 29.0604 12.0427 28.8364 12.384C28.6124 12.7147 28.367 13.0133 28.1004 13.28C27.8444 13.5467 27.5724 13.76 27.2844 13.92C26.9964 14.08 26.7137 14.16 26.4364 14.16C26.1057 14.16 25.8817 14.0427 25.7644 13.808C25.647 13.5733 25.5884 13.2373 25.5884 12.8C25.5884 12.6187 25.5937 12.4267 25.6044 12.224C25.6257 12.0107 25.6577 11.8027 25.7004 11.6L25.6524 11.552C25.3644 12.064 25.0817 12.4853 24.8044 12.816C24.5377 13.1467 24.2764 13.4133 24.0204 13.616C23.7644 13.8187 23.5137 13.9573 23.2684 14.032C23.023 14.1173 22.783 14.16 22.5484 14.16C22.047 14.16 21.695 14.0213 21.4924 13.744C21.3004 13.4667 21.2044 13.1253 21.2044 12.72C21.2044 12.272 21.2897 11.7813 21.4604 11.248C21.631 10.704 21.8124 10.192 22.0044 9.712C22.1964 9.232 22.3564 8.82667 22.4844 8.496C22.623 8.16533 22.7297 7.888 22.8044 7.664C22.8897 7.44 22.9484 7.25867 22.9804 7.12C23.023 6.98133 23.0444 6.86933 23.0444 6.784C23.0444 6.74133 23.0337 6.704 23.0124 6.672C23.0017 6.62933 22.9644 6.608 22.9004 6.608C22.7617 6.608 22.559 6.73067 22.2924 6.976C22.0364 7.22133 21.6844 7.616 21.2364 8.16L20.8204 7.936C20.9804 7.62667 21.183 7.31733 21.4284 7.008C21.6737 6.69867 21.935 6.42133 22.2124 6.176C22.4897 5.93067 22.7777 5.73333 23.0764 5.584C23.375 5.43467 23.6524 5.36 23.9084 5.36C24.4417 5.36 24.7084 5.66933 24.7084 6.288C24.7084 6.66133 24.6017 7.14667 24.3884 7.744C24.1857 8.33067 23.919 9.03467 23.5884 9.856C23.375 10.3787 23.199 10.848 23.0604 11.264C22.9217 11.68 22.8524 12.0587 22.8524 12.4C22.8524 12.624 22.8844 12.784 22.9484 12.88C23.0124 12.9653 23.0924 13.008 23.1884 13.008ZM33.7288 6.848L32.4008 11.248C32.2194 11.824 32.1234 12.2613 32.1128 12.56C32.1128 12.656 32.1341 12.7413 32.1768 12.816C32.2301 12.8907 32.3154 12.928 32.4328 12.928C32.6248 12.928 32.8274 12.8587 33.0408 12.72C33.2541 12.5707 33.4568 12.4 33.6488 12.208C33.8514 12.0053 34.0274 11.8027 34.1768 11.6C34.3368 11.3867 34.4541 11.2213 34.5288 11.104L34.9928 11.376C34.8114 11.7387 34.5821 12.0907 34.3048 12.432C34.0274 12.7627 33.7341 13.056 33.4248 13.312C33.1154 13.568 32.8008 13.776 32.4808 13.936C32.1714 14.0853 31.8888 14.16 31.6328 14.16C31.2594 14.16 30.9874 14.08 30.8168 13.92C30.6568 13.76 30.5768 13.4933 30.5768 13.12C30.5768 12.9067 30.6141 12.6187 30.6888 12.256C30.7634 11.8933 30.8914 11.3973 31.0728 10.768L32.2568 6.848H30.8808L31.0888 6.112H32.4808L33.3128 3.36H34.7848L33.9528 6.112H35.6968L35.4728 6.848H33.7288ZM37.9085 8.72C38.0578 8.304 38.1645 7.968 38.2285 7.712C38.2925 7.456 38.3245 7.25333 38.3245 7.104C38.3245 6.95467 38.2978 6.85867 38.2445 6.816C38.2018 6.76267 38.1538 6.736 38.1005 6.736C37.9832 6.736 37.8552 6.79467 37.7165 6.912C37.5778 7.01867 37.4392 7.14667 37.3005 7.296C37.1618 7.44533 37.0392 7.59467 36.9325 7.744C36.8258 7.89333 36.7512 8.01067 36.7085 8.096L36.2445 7.808C36.3725 7.57333 36.5485 7.312 36.7725 7.024C37.0072 6.736 37.2685 6.46933 37.5565 6.224C37.8445 5.97867 38.1432 5.776 38.4525 5.616C38.7618 5.44533 39.0552 5.36 39.3325 5.36C39.5992 5.36 39.7965 5.44 39.9245 5.6C40.0632 5.74933 40.1325 6.02667 40.1325 6.432C40.1325 6.74133 40.0365 7.15733 39.8445 7.68C39.6632 8.20267 39.4232 8.87467 39.1245 9.696L38.5325 11.312C38.5112 11.3653 38.4792 11.4507 38.4365 11.568C38.4045 11.6747 38.3672 11.792 38.3245 11.92C38.2925 12.0373 38.2605 12.1547 38.2285 12.272C38.2072 12.3893 38.1965 12.4853 38.1965 12.56C38.1965 12.656 38.2232 12.7413 38.2765 12.816C38.3298 12.8907 38.4152 12.928 38.5325 12.928C38.6925 12.928 38.8578 12.864 39.0285 12.736C39.1992 12.5973 39.3645 12.4373 39.5245 12.256C39.6845 12.0747 39.8285 11.8933 39.9565 11.712C40.0845 11.52 40.1805 11.3707 40.2445 11.264L40.7245 11.536C40.5965 11.792 40.4205 12.0747 40.1965 12.384C39.9832 12.6933 39.7378 12.9813 39.4605 13.248C39.1832 13.504 38.8792 13.7227 38.5485 13.904C38.2178 14.0747 37.8765 14.16 37.5245 14.16C37.2152 14.16 36.9805 14.08 36.8205 13.92C36.6712 13.76 36.5965 13.4933 36.5965 13.12C36.5965 12.8107 36.6445 12.48 36.7405 12.128C36.8472 11.7653 36.9965 11.312 37.1885 10.768L37.9085 8.72ZM40.2605 2.736C39.9938 2.736 39.7592 2.64533 39.5565 2.464C39.3645 2.28267 39.2685 2.048 39.2685 1.76C39.2685 1.472 39.3645 1.22667 39.5565 1.024C39.7485 0.821333 39.9832 0.719999 40.2605 0.719999C40.5592 0.719999 40.8045 0.821333 40.9965 1.024C41.1885 1.22667 41.2845 1.46667 41.2845 1.744C41.2845 2.02133 41.1832 2.256 40.9805 2.448C40.7778 2.64 40.5378 2.736 40.2605 2.736ZM44.3045 14.16C43.9418 14.16 43.6005 14.0907 43.2805 13.952C42.9712 13.8027 42.6992 13.5893 42.4645 13.312C42.2298 13.0347 42.0432 12.6987 41.9045 12.304C41.7658 11.8987 41.6965 11.4453 41.6965 10.944C41.6965 10.2507 41.8245 9.568 42.0805 8.896C42.3472 8.224 42.6938 7.62667 43.1205 7.104C43.5472 6.58133 44.0325 6.16 44.5765 5.84C45.1312 5.52 45.7018 5.36 46.2885 5.36C46.6725 5.36 47.0245 5.43467 47.3445 5.584C47.6752 5.72267 47.9525 5.93067 48.1765 6.208C48.4112 6.47467 48.5925 6.8 48.7205 7.184C48.8592 7.55733 48.9285 7.97333 48.9285 8.432C48.9285 9.104 48.7952 9.78133 48.5285 10.464C48.2618 11.1467 47.9098 11.76 47.4725 12.304C47.0458 12.848 46.5552 13.296 46.0005 13.648C45.4458 13.9893 44.8805 14.16 44.3045 14.16ZM44.4005 13.36C44.7632 13.36 45.1205 13.1733 45.4725 12.8C45.8352 12.4267 46.1552 11.9573 46.4325 11.392C46.7205 10.816 46.9498 10.192 47.1205 9.52C47.3018 8.848 47.3925 8.208 47.3925 7.6C47.3925 7.03467 47.2912 6.64533 47.0885 6.432C46.8965 6.21867 46.6298 6.112 46.2885 6.112C46.0538 6.112 45.8138 6.19733 45.5685 6.368C45.3338 6.528 45.1098 6.752 44.8965 7.04C44.6832 7.328 44.4805 7.65867 44.2885 8.032C44.1072 8.40533 43.9472 8.8 43.8085 9.216C43.6698 9.632 43.5632 10.0587 43.4885 10.496C43.4138 10.9227 43.3765 11.3333 43.3765 11.728C43.3765 12.3253 43.4725 12.7467 43.6645 12.992C43.8565 13.2373 44.1018 13.36 44.4005 13.36ZM56.4875 6.192C56.2315 6.192 55.9382 6.32533 55.6075 6.592C55.2875 6.848 54.9568 7.19467 54.6155 7.632C54.2848 8.05867 53.9648 8.54933 53.6555 9.104C53.3462 9.65867 53.0848 10.224 52.8715 10.8L51.6875 14H50.2795L51.4635 10.336C51.6555 9.74933 51.8155 9.248 51.9435 8.832C52.0715 8.416 52.1728 8.06933 52.2475 7.792C52.3328 7.504 52.3915 7.27467 52.4235 7.104C52.4555 6.92267 52.4715 6.77867 52.4715 6.672C52.4715 6.384 52.3808 6.24 52.1995 6.24C52.0822 6.24 51.9435 6.31467 51.7835 6.464C51.6235 6.61333 51.4582 6.81067 51.2875 7.056C51.1275 7.29067 50.9728 7.56267 50.8235 7.872C50.6742 8.18133 50.5462 8.496 50.4395 8.816L49.9595 8.576C50.3328 7.47733 50.7488 6.66667 51.2075 6.144C51.6768 5.62133 52.2102 5.36 52.8075 5.36C53.1488 5.36 53.3888 5.456 53.5275 5.648C53.6768 5.84 53.7515 6.16533 53.7515 6.624C53.7515 7.168 53.6608 7.65333 53.4795 8.08L53.5275 8.128C54.0715 7.168 54.6208 6.46933 55.1755 6.032C55.7408 5.584 56.3542 5.36 57.0155 5.36C57.4848 5.36 57.8422 5.472 58.0875 5.696C58.3328 5.92 58.4555 6.24 58.4555 6.656C58.4555 6.79467 58.4395 6.94933 58.4075 7.12C58.3862 7.29067 58.3435 7.49867 58.2795 7.744C58.2155 7.97867 58.1248 8.26133 58.0075 8.592C57.9008 8.92267 57.7675 9.328 57.6075 9.808C57.3622 10.5333 57.1595 11.1413 56.9995 11.632C56.8395 12.1227 56.7595 12.4693 56.7595 12.672C56.7595 12.832 56.8128 12.912 56.9195 12.912C57.0688 12.912 57.2875 12.7893 57.5755 12.544C57.8635 12.2987 58.2528 11.904 58.7435 11.36L59.1595 11.584C58.9888 11.904 58.7648 12.2187 58.4875 12.528C58.2208 12.8267 57.9382 13.0987 57.6395 13.344C57.3515 13.5893 57.0635 13.7867 56.7755 13.936C56.4875 14.0853 56.2368 14.16 56.0235 14.16C55.7568 14.16 55.5435 14.0427 55.3835 13.808C55.2342 13.5733 55.1595 13.264 55.1595 12.88C55.1595 12.7307 55.1702 12.5813 55.1915 12.432C55.2235 12.272 55.2822 12.048 55.3675 11.76C55.4528 11.472 55.5755 11.088 55.7355 10.608C55.9062 10.1173 56.1355 9.472 56.4235 8.672C56.5195 8.39467 56.5995 8.16 56.6635 7.968C56.7275 7.776 56.7755 7.61067 56.8075 7.472C56.8502 7.32267 56.8768 7.19467 56.8875 7.088C56.9088 6.98133 56.9195 6.88 56.9195 6.784C56.9195 6.38933 56.7755 6.192 56.4875 6.192Z"
        fill="black"
      ></path>
    }
}
