use yew_router::{Switch, router::Router};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/!"]
    Home,
    #[to = "/{*:id}"]
    Room(String),
}

pub type AppRouter = Router<AppRoute>;