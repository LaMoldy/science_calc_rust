use yew::{Component, Context, html, Html, Properties};
use stylist::{Style, css};

// Save a file name
const STYLE_FILE: &str = include_str!("operation_button.css");

#[derive(PartialEq, Properties)]
pub struct Props {
  pub bg: String,
  pub operation: String
}

pub struct OperationButton;

impl Component for OperationButton {
  // Sets message and props
  type Message = ();
  type Properties = Props;

  // Creates the view
  fn create(_ctx: &Context<Self>) -> Self {
    OperationButton
  }

  // The view the user sees
  fn view(&self, ctx: &Context<Self>) -> Html {
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();

    html! {
      <div class={stylesheet}>
        if &ctx.props().bg == "blue" {
          <button class={css!("background-color: #109fff; color: #ffffff;")}>{&ctx.props().operation}</button>
        }
        else {
          <button class={css!("background-color: #ffffff;")}>{&ctx.props().operation}</button>
        }
      </div>
    }
  }
}
