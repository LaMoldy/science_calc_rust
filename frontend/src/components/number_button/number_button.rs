use yew::{Component, Context, html, Html, Properties};
use yew::prelude::*;
use stylist::Style;

// Saves a file name
const STYLE_FILE: &str = include_str!("number_button.css");

#[derive(PartialEq, Properties)]
pub struct Props {
  pub number: String,
  pub on_click: Callback<String>
}

pub struct NumberButton;

impl Component for NumberButton {
  // Sets message and props
  type Message = ();
  type Properties = Props;

  // Creates the button
  fn create(_ctx: &Context<Self>) -> Self {
    NumberButton
  }

  // What the user sees when they use the site
  fn view(&self, ctx: &Context<Self>) -> Html {
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();

    ctx.props().on_click.emit(ctx.props().number);

    html! {
      <div class={stylesheet}>
        <button>{&ctx.props().number}</button>
      </div>
    }
  }
}
