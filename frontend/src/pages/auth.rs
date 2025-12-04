use leptos::*;
use crate::api::{register, login, RegisterRequest, LoginRequest};
use crate::components::card::Card;

#[component]
pub fn AuthPage() -> impl IntoView {
    let (register_username, set_register_username) = create_signal(String::new());
    let (register_email, set_register_email) = create_signal(String::new());
    let (register_password, set_register_password) = create_signal(String::new());
    let (login_email, set_login_email) = create_signal(String::new());
    let (login_password, set_login_password) = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());

    let handle_register = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let username = register_username.get();
        let email = register_email.get();
        let password = register_password.get();

        spawn_local(async move {
            match register(RegisterRequest { username, email, password }).await {
                Ok(user) => {
                    set_message.set(format!("Registration successful! Welcome, {}", user.username));
                }
                Err(e) => {
                    set_message.set(format!("Registration failed: {}", e));
                }
            }
        });
    };

    let handle_login = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let email = login_email.get();
        let password = login_password.get();

        spawn_local(async move {
            match login(LoginRequest { email, password }).await {
                Ok(response) => {
                    set_message.set(format!("Login successful! Token: {}", response.token));
                }
                Err(e) => {
                    set_message.set(format!("Login failed: {}", e));
                }
            }
        });
    };

    view! {
        <div class="auth-page">
            <h1>"Authentication"</h1>

            {move || {
                let msg = message.get();
                if !msg.is_empty() {
                    view! {
                        <div class="message">{msg}</div>
                    }.into_view()
                } else {
                    view! {}.into_view()
                }
            }}

            <div class="auth-forms">
                <Card title="Register">
                    <form on:submit=handle_register>
                        <div class="form-group">
                            <label>"Username:"</label>
                            <input
                                type="text"
                                placeholder="Username"
                                on:input=move |ev| set_register_username.set(event_target_value(&ev))
                                prop:value=register_username
                            />
                        </div>
                        <div class="form-group">
                            <label>"Email:"</label>
                            <input
                                type="email"
                                placeholder="Email"
                                on:input=move |ev| set_register_email.set(event_target_value(&ev))
                                prop:value=register_email
                            />
                        </div>
                        <div class="form-group">
                            <label>"Password:"</label>
                            <input
                                type="password"
                                placeholder="Password"
                                on:input=move |ev| set_register_password.set(event_target_value(&ev))
                                prop:value=register_password
                            />
                        </div>
                        <button type="submit">"Register"</button>
                    </form>
                </Card>

                <Card title="Login">
                    <form on:submit=handle_login>
                        <div class="form-group">
                            <label>"Email:"</label>
                            <input
                                type="email"
                                placeholder="Email"
                                on:input=move |ev| set_login_email.set(event_target_value(&ev))
                                prop:value=login_email
                            />
                        </div>
                        <div class="form-group">
                            <label>"Password:"</label>
                            <input
                                type="password"
                                placeholder="Password"
                                on:input=move |ev| set_login_password.set(event_target_value(&ev))
                                prop:value=login_password
                            />
                        </div>
                        <button type="submit">"Login"</button>
                    </form>
                </Card>
            </div>
        </div>
    }
}
