use yew::prelude::*;

pub fn layout(main: Html) -> Html {
    html! {
        <>
        <header>
            <nav class="navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <a class="navbar-item" href="/">
                        <img class="image is-24x24 mr-2" alt="logo" src="/assets/favicon.ico" />
                        <strong>{"Roll Role"}</strong>
                    </a>
                </div>
                <div class="navbar-menu">
                    <div class="navbar-end">
                        <div class="navbar-item">
                            <div class="buttons">
                                <a class="button" href="https://github.com/itskihaga/exprocess-rust-sample">
                                    {"Contact"}
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        </header>
        <main class="my-main">
            {main}
        </main>
        </>
    }
}
