use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::{
    actuators,
    error::{lock_err, GenericResult},
    io::RelaySwitchState,
    state::ProgramStateShared,
};

pub async fn run_server(program_state: ProgramStateShared) {
    let app: Router = setup_router(program_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2205").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn setup_router(program_state: ProgramStateShared) -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/switch/:device/:state", get(switch_handler))
        .with_state(program_state)
}

async fn switch_handler(
    Path((device, state)): Path<(String, RelaySwitchState)>,
    State(program_state): State<ProgramStateShared>,
) -> impl IntoResponse {
    let exec = || -> GenericResult<()> {
        let mut program_state = program_state.lock().map_err(lock_err)?;
        match device.as_str() {
            "lights" => actuators::switch_lights(state, &mut program_state)?,
            "pump" => actuators::switch_water_pump(state, &mut program_state)?,
            "fan" => actuators::switch_fan(state, &mut program_state)?,
            _ => (),
        }
        Ok(())
    };
    match exec() {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn handler() -> Html<&'static str> {
    Html("hi")
}
