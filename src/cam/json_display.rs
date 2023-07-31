use yew::prelude::*;
use serde_json::Value as JsonValue;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: JsonValue,
}

pub struct JsonDisplay {
    pub data: JsonValue,
    expanded: bool,
}

impl Component for JsonDisplay {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            expanded:false,
            data:_ctx.props().data.clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!{
            <div class="m-2">
            </div>
        }
    }

}