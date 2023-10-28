use leptos::{ev::resize, html::Div, *};
use leptos_use::use_event_listener;
use web_sys::MouseEvent;

#[component]
pub fn Solution(cx: Scope, children: Children) -> impl IntoView {
    let set_solution_open = use_context::<WriteSignal<bool>>(cx).unwrap();
    let solution_open = use_context::<ReadSignal<bool>>(cx).unwrap();
    let (content_height, set_content_height) = create_signal(cx, 0);
    let node_ref = create_node_ref::<Div>(cx);
    create_effect(cx, move |_| {
        if node_ref().is_some() {
            if solution_open() {
                set_content_height(node_ref().unwrap().offset_height())
            } else {
                set_content_height(0)
            }
        }
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

    view! { cx,
      <div class="px-4 my-9 relative col-start-2">
        <SolutionSVG on_click=move |_| { set_solution_open(!solution_open()) }/>
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
          class="transition-all duration-1000"
          class=("-translate-y-full", move || !solution_open())
        >
          {children(cx)}
        </div>

      </div>
    }
}

#[component]
pub fn SolutionSVG<F>(cx: Scope, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let (active, set_active) = create_signal(cx, true);
    view! { cx,
      <div
        on:click=move |e| {
            set_active(!active());
            on_click(e)
        }

        class="column solution_button_div cursor-pointer mb-16"
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
              class=("active_solution_button_rect", move || active())
              class=("inactive_solution_button_rect", move || !active())

              width="109"
              height="36"
            ></rect>

            <path
              id="solution_button_lip"
              class="solution_button_transition"
              class=("active_solution_button_lip", move || active())
              class=("inactive_solution_button_lip", move || !active())
              d="M 0 10 v -10 h 109 v 10 M 0 26 v 10 h 109 v -10"
            ></path>

            <g
              id="solution_button_finger_pair"
              class="solution_button_transition"
              class=("active_solution_button_hands", move || active())
              class=("inactive_solution_button_hands", move || !active())
            >
              <SVGLeftFinger transform="translate(101.5, 18)"/>
              <SVGLeftFinger transform="scale(-1, 1) translate(-8, 20)"/>
            </g>
          </g>
        </svg>
        <span class="solution_button_inscription font-baskerville-italic">"solution"</span>
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
