use axum::extract::FromRef;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub mongo: mongodb::Client,
    pub leptos_options: leptos::LeptosOptions,
    pub routes: Vec<leptos_router::RouteListing>,
}
