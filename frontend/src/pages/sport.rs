use leptos::*;
use leptos_router::*;
use crate::components::card::Card;

#[component]
pub fn SportPage() -> impl IntoView {
    let params = use_params_map();
    let sport_id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    view! {
        <div class="sport-page">
            <h1>{move || format!("Sport: {}", sport_id())}</h1>

            <Card title="Latest News">
                <p>"Latest news for this sport will appear here"</p>
            </Card>

            <Card title="Records">
                <p>"View world records and rankings"</p>
            </Card>

            <Card title="Submit Your Result">
                <form>
                    <div class="form-group">
                        <label>"Category:"</label>
                        <input type="text" placeholder="e.g., 60kg weight class" />
                    </div>
                    <div class="form-group">
                        <label>"Result:"</label>
                        <input type="text" placeholder="Your result" />
                    </div>
                    <button type="submit">"Submit"</button>
                </form>
            </Card>

            <Card title="Exercises & Tips">
                <p>"Training tips and recommended exercises"</p>
            </Card>
        </div>
    }
}
