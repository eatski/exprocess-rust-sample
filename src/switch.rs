use yew::{html::IntoPropValue};
use yew_router::{ prelude::*};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/!"]
    Home,
    #[to = "/{*:id}"]
    Room(String),
}
impl AppRoute {
    pub fn into_public(self) -> PublicUrlSwitch {
        PublicUrlSwitch(self)
    }

    pub fn into_route(self) -> Route {
        Route::from(self.into_public())
    }
}

/// Helper type which just wraps around the actual `AppRoute` but handles a public url prefix.
/// We need to have this because we're hosting the example at `/router/` instead of `/`.
/// This type allows us have the best of both worlds.
#[derive(Clone, Debug)]
pub struct PublicUrlSwitch(AppRoute);
impl PublicUrlSwitch {

    fn base_path() -> String {
        String::from("")
    }

    pub fn route(self) -> AppRoute {
        self.0
    }

}
impl Switch for PublicUrlSwitch {
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        if let Some(part) = part.strip_prefix(&Self::base_path()) {
            let (route, state) = AppRoute::from_route_part(part.to_owned(), state);
            (route.map(Self), state)
        } else {
            (None, None)
        }
    }

    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE> {
        route.push_str(&Self::base_path());
        self.0.build_route_section(route)
    }
}

// this allows us to pass `AppRoute` to components which take `PublicUrlSwitch`.

impl IntoPropValue<PublicUrlSwitch> for AppRoute {
    fn into_prop_value(self: AppRoute) -> PublicUrlSwitch {
        self.into_public()
    }
}

// type aliases to make life just a bit easier

pub type AppRouter = Router<PublicUrlSwitch>;
