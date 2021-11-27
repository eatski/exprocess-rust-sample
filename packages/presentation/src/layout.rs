use yew::prelude::*;

pub fn layout(main: Html) -> Html {
    html! {
        <>
        <header>
            <nav class="navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <a class="navbar-item" href="/">
                        <img class="image is-24x24 mr-2" src="/assets/favicon.ico" />
                        <strong>{"Roll Role"}</strong>
                    </a>
                </div>
            </nav>
        </header>
        
        <main class="my-main">
            {main}
        </main>
        <footer class="footer my-bottom">
            <div class="content has-text-centered">
                <a href="https://twitter.com/gagagaitsu">{"開発者のTwitter"}</a>
            </div>
        </footer>
        </>
    }
}
