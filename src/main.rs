use axum::extract::State;
use axum::http::StatusCode;
use axum::{Router, response::Html, routing::get};
use minijinja::{Environment, context};
use std::sync::Arc;
use tower_http::services::ServeDir;

//this structure holds all of the stuff in the env
//so far this is useless but will be good in the future
struct AppState {
    //this holds the environment for the entire runtime - its static type
    env: Environment<'static>,
}

//async
#[tokio::main]
async fn main() {
    //init the jinja templates
    let mut env = Environment::new();

    //componenents layout (components that are often reuset - eg.  the logo)
    // will have to handle errors in the future - probably separate this into a different file and folder alltogether
    env.add_template("components", include_str!("../templates/components.jinja"))
        .unwrap();
    //the base template
    env.add_template("base", include_str!("../templates/base.jinja"))
        .unwrap();
    //the static app - static so far as of 30.3.2026 idk what will happen in the future
    env.add_template("app", include_str!("../templates/app.jinja"))
        .unwrap();
    //login page
    env.add_template("login", include_str!("../templates/login.jinja"))
        .unwrap();

    //pass the env with the templates to handlers via state (Arc takes care of some stuff i don't really understand)
    //i guess it offloads the appp state to the heap? ARC = Atomically reference counted - yeah idk
    let app_state = Arc::new(AppState { env });

    //define the routes
    // will also have to be offloaded to a different file in the future probably
    let app: Router = Router::new()
        .route("/", get(handler_app))
        //so that styling works - my css and config.js are in the static folder
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    //run server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await;
}

async fn handler_app(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("app").unwrap();

    let rendered = template.render(context! 
    								{user => context! 
    									{name => "Petr Guláš"}, 
    								stats => context! 
    									{ active_assets => "02", signal_integrity => 98.4}, 
    								map_markers => vec![
    									context! {id => "PERKELE", status => "active", x => 70, y => 60}
    									],
    								assets => vec![
    									context! 
    									{name => "PidlačFidlač", 
    									status => "Active", 
    									coords => "49.2827° N, 123.1207° W", 
    									fuel => "84%", 
    									last_ping => "00:00:04"}]})
    									.unwrap();

    Ok(Html(rendered))
}
