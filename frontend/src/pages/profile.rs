use leptos::*;
use crate::components::card::Card;

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <div class="profile-page">
            <h1>"Your Profile"</h1>

            <Card title="User Info">
                <p>"Username: (Not logged in)"</p>
                <p>"Email: (Not logged in)"</p>
            </Card>

            <Card title="Your Records">
                <p>"No records yet. Start tracking your progress!"</p>
            </Card>

            <Card title="Statistics">
                <p>"Activity stats will appear here"</p>
            </Card>
        </div>
    }
}
