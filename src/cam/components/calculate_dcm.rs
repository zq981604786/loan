use yew::prelude::*;


pub struct ComponentCalculateDcm{}

impl Component for ComponentCalculateDcm{
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
       Self{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <>
                <input type="text"/>
                <input type="text"/>
            </>
        }
    }
}