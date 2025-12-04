use leptos::*;

#[component]
pub fn PostPreview(
    _id: String,
    title: String,
    author: Option<String>,
    created_at: String,
) -> impl IntoView {
    view! {
        <div class="post-preview">
            <h4>{title}</h4>
            <p class="post-meta">
                {author.unwrap_or_else(|| "Anonymous".to_string())}
                " • "
                {created_at}
            </p>
        </div>
    }
}
