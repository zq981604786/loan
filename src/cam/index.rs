use yew::prelude::*;
use yew_router::prelude::*;


use crate::cam::route::Route;
use crate::cam::home::Home;
use crate::cam::loan::Loan;
use crate::cam::not_found::NotFound;

fn switch(routes:Route)->Html{
  match routes {
    Route::Home => html! {<Home/>},
    Route::Loan => html! {<Loan/>},
    _ => html! {<NotFound/>},
  }
}

#[function_component]
pub fn App() -> Html{
  html!{
    <BrowserRouter>
      <Switch<Route> render={|route| switch(route)} />
    </BrowserRouter>
  }
}