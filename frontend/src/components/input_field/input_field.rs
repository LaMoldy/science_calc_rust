use yew::{html, Html, Component, Properties};
use stylist::Style;

const STYLE_FILE: &str = include_str!("input_field.css");

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct InputField;

impl Component for InputField {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
      InputField
    }

  fn view(&self, _ctx: &yew::Context<Self>) -> Html {
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();
      html! {
        <div class={stylesheet}>
          <div class={"mb"}>
            <input type="text" readonly=true/>  
          </div>
        </div>
     }
  }
}
