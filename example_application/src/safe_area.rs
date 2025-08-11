use crate::app::*;
use capacitor_bindings::{
    error::Error,
    safe_area::{Config, LightOrDark, Options, SafeArea},
};
use yew::prelude::*;

#[function_component(SafeAreaView)]
pub fn safe_area_view() -> Html {
    html!(
        <details>
            <summary>
                {"Insets"}
                <span class="icon">{"↓"}</span>
            </summary>
            <div style="display: flex; flex-direction: column;">
                <br/>
                <button onclick={|_| do_sync(enable_light) }> {"Enable Light"}</button>
                <button onclick={|_| do_sync(enable_dark) }> {"Enable Dark"}</button>
                <button onclick={|_| do_sync(disable) }> {"Disable"}</button>


            </div>
        </details>
    )
}

fn enable_dark() -> Result<(), Error> {
    SafeArea::enable(Options {
        config: Config {
            custom_colors_for_system_bars: true,
            status_bar_color: "#00ffff".to_string(),
            status_bar_content: LightOrDark::Dark,
            navigation_bar_color: "#ffff00".to_string(),
            navigation_bar_content: LightOrDark::Dark,
            offset: 0,
        },
    })
}

fn enable_light() -> Result<(), Error> {
    SafeArea::enable(Options {
        config: Config {
            custom_colors_for_system_bars: true,
            status_bar_color: "#ff0000".to_string(),
            status_bar_content: LightOrDark::Light,
            navigation_bar_color: "#0000ff".to_string(),
            navigation_bar_content: LightOrDark::Light,
            offset: 0,
        },
    })
}

fn disable() -> Result<(), Error> {
    SafeArea::disable(Options::default())
}
