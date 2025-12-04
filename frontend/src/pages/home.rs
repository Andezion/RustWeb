use leptos::*;
use crate::components::card::Card;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home-page">
            <h1>"Welcome to RustWeb Sports Forum"</h1>
            <p>"A platform for athletes to connect, share records, and discuss sports"</p>

            <div class="home-sections">
                <Card title="Latest News">
                    <p>"Stay updated with the latest sports news and announcements"</p>
                </Card>

                <Card title="Forum">
                    <p>"Join discussions with other athletes and enthusiasts"</p>
                    <a href="/forum">"Go to Forum →"</a>
                </Card>

                <Card title="Your Stats">
                    <p>"Track your progress and compare with others"</p>
                    <a href="/profile">"View Profile →"</a>
                </Card>
            </div>

            <div class="sports-list">
                <h2>"Popular Sports"</h2>
                <ul>
                    <li><a href="/sport/armwrestling">"Arm Wrestling"</a></li>
                    <li><a href="/sport/running">"Running"</a></li>
                    <li><a href="/sport/weightlifting">"Weightlifting"</a></li>
                    <li><a href="/sport/powerlifting">"Powerlifting"</a></li>
                </ul>
            </div>
        </div>
    }
}
