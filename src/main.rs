use leptos::provide_context;
use thingy::app::App;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use thingy::state;
    use thingy::{app::*, fallback::file_and_error_handler};
    use log::info;

    let _ = dotenv::dotenv();

    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");


    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // app state
    let uri = std::env::var("MONGODB_URI").expect("Missing MONGODB_URI");
    let client = mongodb::Client::with_uri_str(uri).await.unwrap();
    let app_state = state::AppState { mongo: client, leptos_options: leptos_options.clone(), routes: routes.clone(), };

    // build our application with a route
    let app = Router::new()
        // .leptos_routes(&leptos_options, routes, App)
        .route(
            "/api/*fn_name",
            axum::routing::get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, axum::routing::get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        // .with_state(leptos_options)
        .with_state(app_state);

    println!("{:#?}", app);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
async fn server_fn_handler(
    axum::extract::State(app_state): axum::extract::State<thingy::state::AppState>,
    path: axum::extract::Path<String>,
    headers: axum::http::header::HeaderMap,
    raw_query: axum::extract::RawQuery,
    request: axum::http::Request<axum::body::Body>,
) -> impl axum::response::IntoResponse {
    use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};

    // log::log!("{:?}", path);

    handle_server_fns_with_context(
        move || {
            // provide_context(cx, auth_session.clone());
            provide_context(app_state.mongo.clone());
        },
        request,
    )
    .await
}

#[cfg(feature = "ssr")]
async fn leptos_routes_handler(
    axum::extract::State(app_state): axum::extract::State<thingy::state::AppState>,
    req: axum::http::Request<axum::body::Body>,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            // provide_context(auth_session.clone());
            provide_context(app_state.mongo.clone());
        },
        App,
    );
    handler(req).await.into_response()
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
