use yew::prelude::*;

use crate::cam::components::loan::ComponentLoan;


#[function_component]
pub fn Home() -> Html {
    html!{
        <div class="container" style="overflow-x: auto;">
            <ComponentLoan/>
        </div>
    }
}


