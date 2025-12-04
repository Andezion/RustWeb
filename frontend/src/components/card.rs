use leptos::*;

#[component]
pub fn Card(
    #[prop(into, optional)] title: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card">
            {move || title.clone().map(|t| view! { <h3 class="card-title">{t}</h3> })}
            <div class="card-content">
                {children()}
            </div>
        </div>
    }
}
