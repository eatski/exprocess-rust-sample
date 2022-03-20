use yew_router::{Routable};



#[derive(Clone, Debug,PartialEq,Routable)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/:id")]
    Room { id: String },
}