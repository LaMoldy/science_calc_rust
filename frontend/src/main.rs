pub mod components;

use stylist::Style;
use yew::prelude::*;
use components::number_button::number_button::NumberButton;
use components::operation_button::operation_button::OperationButton;

// Saves a file name
const STYLE_FILE: &str = include_str!("main.css");

// Starts the app
fn main() {
    yew::start_app::<App>();
}

// Makes an app component
#[function_component(App)]
pub fn app() -> Html {
    // Makes a stylesheet
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();
    // Returns HTML
    html! {
        <div class={stylesheet}>
            <h1 class={"text-center"}>{"Scientific Calculator"}</h1>
            <div class={"flex-col"}>
                <div class={"flex-row gap mb"}>
                    <NumberButton number="1" />
                    <NumberButton number="2" />
                    <NumberButton number="3" />
                </div>
                <div class={"flex-row gap mb"}>
                    <NumberButton number="4" />
                    <NumberButton number="5" />
                    <NumberButton number="6" />
                </div>
                <div class={"flex-row gap mb"}>
                    <NumberButton number="7" />
                    <NumberButton number="8" />
                    <NumberButton number="9" />
                </div>
                <div class={"flex-row gap mb"}>
                    <NumberButton number="0" />
                    <NumberButton number="." />
                    <OperationButton bg="blue" operation="=" />
                </div>
            </div>
        </div>
    }
}
