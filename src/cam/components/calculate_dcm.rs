use yew::prelude::*;
use crate::cam::components::upload::ComponentUpload;

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

                // <label for="fileUpload" class="btn btn-primary">{"上传文件"}</label>
                // <input id="fileUpload" type="file"/>
                // <input type="text"/>
                // <input type="text"/>
                // <ComponentUpload />
            </>
        }
    }
}