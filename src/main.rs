use axum::{Router, routing::get};
use minijinja::Environment;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tokio::signal;


//for the handlers
mod handlers;
//for structs
mod structs;


//this structure holds all of the stuff in the env
//so far this is useless but will be good in the future
pub struct AppState {
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
    //it basically puts the AppState on heap, insteaod of stack and distributes pointers to it between all the async functions
    //it also keeps track of how many pointers it distributed and when that count is 0, it cleans it up

    //define the routes
    // will also have to be offloaded to a different file in the future probably
    let app: Router = Router::new()
        .route("/", get(handlers::handler_app))
        //logout route
        .route("/logout", get(handlers::handler_logout))
        //so that styling works - my css and config.js are in the static folder
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    //run server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
    //setup graceful shutdown here
    .with_graceful_shutdown(shutdown_signal()).
    await.
    unwrap();
}

//function to accomodate graceful shutdown in docker
async fn shutdown_signal() {
	let ctrl_c = async {
		signal::ctrl_c().await.expect("Failed to listen for CTRL+C");
	};
	let terminate = async {
		//for unix systems
		signal::unix::signal(signal::unix::SignalKind::terminate()).expect("Failed to listen for termination signal.").recv().await;
	};

	//what the hell is even this?
	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}
	tracing::info!("Graceful shutdown signal received");
}
