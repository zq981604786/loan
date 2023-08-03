use yew::prelude::*;
use gloo::console::log;

pub struct  ComponentTable{}
use crate::cam::model::loan_interest_count_record::LoanInterestCountRecord;
#[derive(Properties,PartialEq)]
pub struct Props{
    pub title:Vec<String>,
    pub data:Vec<LoanInterestCountRecord>,
}

impl Component for ComponentTable {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let data = ctx.props().data.clone().iter();
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
                    {for ctx.props().data.iter().map(|item| html!{
                        <tr>
                            <th scope="col">{item.id}</th>
                        </tr>
                    })}
                </tbody>
            </table>
        }
    }
}