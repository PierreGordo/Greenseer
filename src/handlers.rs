use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use minijinja::context;
use std::sync::Arc;

//importing the AppState struct from main.rs
use crate::AppState;


//This function construct the HTML to serve with the template
pub async fn handler_app(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("app").unwrap();

    let rendered = template.render(context! 
    								{user => context! 
    									{name => "Petr Guláš"}, 
    								stats => context! 
    									{ active_assets => "02", signal_integrity => 98.4}, 
    								map_markers => vec![
    									context! {id => "PERKELE", status => "Active", x => 30, y => 10}
    									],
    								assets => vec![
    									context! 
    									{name => "PidlačFidlač", 
    									status => "Active", 
    									coords => "49.2827° N, 123.1207° W", 
    									fuel => "84%", 
    									last_ping => "00:00:04"}],
									asset_groups => context! {
										//this here is modular, so i can add whatever i want
										ground_vehicles => vec![
											context! {name => "Tatra 815", status => "Active"},
											context! {name => "Volvo Truck", status => "Offline"}
											],
										air_assets => vec![
											context! {name => "Dronus spiritus", status => "Active"},
											context! {name => "CAS stříleč", status => "Offline"}											
											]
										},
    								})
    									.unwrap();

    Ok(Html(rendered))
}

//handles logout 
pub async fn handler_logout(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {

	let template = state.env.get_template("login").unwrap();
	//no arguments here for the temmplate
	let rendered = template.render(context!{}).unwrap();

	Ok(Html(rendered))
}

