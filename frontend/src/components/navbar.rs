use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="nav-brand">
                <A href="/">"RustWeb"</A>
            </div>
            <ul class="nav-links">
                <li><A href="/">"Home"</A></li>
                <li><A href="/forum">"Forum"</A></li>
                <li><A href="/auth">"Login/Register"</A></li>
                <li><A href="/profile">"Profile"</A></li>
            </ul>
        </nav>
    }
}
