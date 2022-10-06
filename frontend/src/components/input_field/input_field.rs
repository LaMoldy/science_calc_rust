use yew::{html, Html, Component, Context, Properties};
use stylist::Style;

const STYLE_FILE: &str = include_str!("input_field.css");

#[derive(PartialEq, Properties)]
pub struct Props {
  pub text: String
}

pub struct InputField {
  value: &'static str
}

impl Component for InputField {
type Message = ();
type Properties = Props;

fn create(_ctx: &Context<Self>) -> Self {
  Self {
    value: "0".as_ref()
  }
}

  fn view(&self, ctx: &Context<Self>) -> Html {
    let stylesheet: Style = Style::new(STYLE_FILE).unwrap();
    //self.value = &ctx.props().text.as_str();

    html! {
      <div class={stylesheet}>
        <div class={"mb"}>
          <input type="text" value={self.value} readonly=true />
        </div>
      </div>
    }
  }
}
