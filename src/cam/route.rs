use yew_router::prelude::*;



#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route{
  #[at("/")]
  Home,
  #[at("/loan")]
  Loan,
  #[at("/*")]
  NotFound,
}