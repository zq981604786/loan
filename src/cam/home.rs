use yew::prelude::*;

use crate::cam::components::loan::ComponentLoan;


#[function_component]
pub fn Home() -> Html {
    html!{
        <>
            <ComponentLoan/>
        </>
    }
}


