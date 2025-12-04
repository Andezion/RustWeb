use leptos::*;

#[derive(Clone, Debug)]
pub struct AuthState {
    pub token: RwSignal<Option<String>>,
    pub username: RwSignal<Option<String>>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            token: create_rw_signal(None),
            username: create_rw_signal(None),
        }
    }

    pub fn login(&self, token: String, username: String) {
        self.token.set(Some(token));
        self.username.set(Some(username));
    }

    pub fn logout(&self) {
        self.token.set(None);
        self.username.set(None);
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.get().is_some()
    }
}
