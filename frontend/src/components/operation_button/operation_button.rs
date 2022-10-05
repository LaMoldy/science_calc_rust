use yew::{Component, Context, html, Html, Properties};
use stylist::Style;

const STYLE_FILE: &str = include_str!("operation_button.css");

#[derive(PartialEq, Properties)]
pub struct Props {
  pub operation: String
}

pub struct OperationButton;

impl Component for OperationButton {
  type Message = ();
  type Properties = Props;

  fn create(_ctx: &Context<Self>) -> Self {
    OperationButton
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();
    html! {
      <div class={stylesheet}>
        <button>{&ctx.props().operation}</button>
      </div>
    }
  }
}
