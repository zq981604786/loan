use yew::prelude::*;
use gloo::console::log;

pub struct  ComponentTable{}

#[derive(Properties,PartialEq)]
pub struct Props{
    pub title:Vec<String>,
    pub data:Vec<i32>,
}

impl Component for ComponentTable {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let data = ctx.props().data.clone().iter();
        html!{
            <table class="table">
                <thead>
                    {for ctx.props().title.clone().iter().map(|item| html!{
                        <tr>
                            <th scope="col">{item}</th>
                        </tr>
                    })}
                </thead>
                <tbody class="table-group-divider">
                    {for ctx.props().data.clone().iter().map(|item| html!{
                        <tr>
                            <th scope="col">{item}</th>
                        </tr>
                    })}
                </tbody>
            </table>
        }
    }
}