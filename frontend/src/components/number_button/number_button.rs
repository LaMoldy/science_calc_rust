use yew::{Component, Context, html, Html, Properties};
use stylist::Style;

const STYLE_FILE: &str = include_str!("number_button.css");

#[derive(PartialEq, Properties)]
pub struct Props {
  pub number: String
}

pub struct NumberButton;

impl Component for NumberButton {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    NumberButton
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();
    html! {
      <div class={stylesheet}>
        <button>{&ctx.props().number}</button>
      </div>
    }
  }
}
