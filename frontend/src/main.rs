pub mod components;

use yew::prelude::*;
use components::number_button::number_button::NumberButton;
use components::operation_button::operation_button::OperationButton;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <NumberButton number="1" />
            <NumberButton number="2" />
            <NumberButton number="3" />

            <NumberButton number="4" />
            <NumberButton number="5" />
            <NumberButton number="6" />

            <NumberButton number="7" />
            <NumberButton number="8" />
            <NumberButton number="9" />

            <NumberButton number="0" />
            <NumberButton number="." />
            <OperationButton operation="=" />
        </div>
    }
}
