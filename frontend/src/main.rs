pub mod components;

use stylist::Style;
use yew::prelude::*;

use components::number_button::number_button::NumberButton;
use components::operation_button::operation_button::OperationButton;
use components::input_field::input_field::InputField;

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


    let button_clicked = Callback::from(|button_value: String| println!("{}", button_value));

    // Returns HTML
    html! {
        <div class={stylesheet}>
            <h1 class={"text-center"}>{"Scientific Calculator"}</h1>
            <InputField text="" />
            <div class={"flex-col"}>
                <div class={"flex-row gap mb"}>
                    <NumberButton number="1" on_click={button_clicked} />
                    <NumberButton number="2" on_click={button_clicked} />
                    <NumberButton number="3" on_click={button_clicked} />
                </div>
                <div class={"flex-row gap mb"}>
                    <NumberButton number="4" on_click={button_clicked} />
                    <NumberButton number="5" on_click={button_clicked} />
                    <NumberButton number="6" on_click={button_clicked} />
                </div>
                <div class={"flex-row gap mb"}>
                    <NumberButton number="7" on_click={button_clicked} />
                    <NumberButton number="8" on_click={button_clicked} />
                    <NumberButton number="9" on_click={button_clicked} />
                </div>
                <div class={"flex-row gap mb"}>
                    <NumberButton number="0" on_click={button_clicked} />
                    <NumberButton number="." on_click={button_clicked} />
                    <OperationButton bg="blue" operation="=" />
                </div>
            </div>
        </div>
    }
}
