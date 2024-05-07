use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    actuators,
    error::GenericResult,
    io::RelaySwitchState,
    sensors,
    state::{lock_state, ProgramStateShared},
};

pub async fn run_server(program_state: ProgramStateShared) {
    let app: Router = setup_router(program_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2205").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn setup_router(program_state: ProgramStateShared) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/info", get(info_handler))
        .route("/switch/:device/:state", get(switch_handler))
        .route("/pump/:quantity", get(pump_handler))
        .with_state(program_state)
        .layer(cors)
}

async fn pump_handler(
    Path(quantity): Path<u16>,
    State(program_state): State<ProgramStateShared>,
) -> impl IntoResponse {
    let exec = || -> GenericResult<()> {
        let mut program_state = lock_state(&program_state)?;
        actuators::pump_water(quantity, &mut program_state)?;
        Ok(())
    };
    match exec() {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn switch_handler(
    Path((device, state)): Path<(String, RelaySwitchState)>,
    State(program_state): State<ProgramStateShared>,
) -> impl IntoResponse {
    let exec = || -> GenericResult<()> {
        let mut program_state = lock_state(&program_state)?;
        match device.as_str() {
            "lights" => actuators::switch_lights(state, &mut program_state)?,
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

#[derive(Serialize, Deserialize)]
struct Info {
    temperature: f32,
    soil_moisture: f32,
    fan_state: RelaySwitchState,
    light_state: RelaySwitchState,
    pump_state: RelaySwitchState,
}

async fn info_handler(
    State(program_state): State<ProgramStateShared>,
) -> Result<Json<Info>, String> {
    let mut program_state = lock_state(&program_state).map_err(|e| e.to_string())?;
    let temperature = sensors::get_temperature(&program_state.config).map_err(|e| e.to_string())?;
    let soil_moisture =
        sensors::get_soil_moisture(&program_state.config).map_err(|e| e.to_string())?;
    let fan_state = actuators::get_fan_state(&mut program_state).map_err(|e| e.to_string())?;
    let light_state = actuators::get_light_state(&mut program_state).map_err(|e| e.to_string())?;
    let pump_state =
        actuators::get_water_pump_state(&mut program_state).map_err(|e| e.to_string())?;
    Ok(Json(Info {
        temperature,
        soil_moisture,
        fan_state,
        light_state,
        pump_state,
    }))
}
