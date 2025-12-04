use leptos::*;
use leptos_router::*;
use leptos_meta::*;

use crate::pages::{home::Home, auth::AuthPage, forum::ForumPage, profile::ProfilePage, sport::SportPage};
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        <Title text="RustWeb - Sports Forum"/>
        <Router>
            <Navbar />
            <main class="container">
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/auth" view=AuthPage/>
                    <Route path="/forum" view=ForumPage/>
                    <Route path="/profile" view=ProfilePage/>
                    <Route path="/sport/:id" view=SportPage/>
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
