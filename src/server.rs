use axum::{
    extract::{Path, State},
    http::{header, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    actuators, control,
    error::GenericResult,
    io::RelaySwitchState,
    sensors,
    state::{lock_state, ProgramStateShared},
};

pub async fn run_server(program_state: ProgramStateShared) {
    let app: Router = setup_router(program_state.clone());
    let port = lock_state(&program_state)
        .map(|state| state.config.server_settings.port)
        .unwrap_or(2205);
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn setup_router(program_state: ProgramStateShared) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/api/info", get(info_handler))
        .route("/api/switch/:device/:state", get(switch_handler))
        .route("/api/pump/:quantity", get(pump_handler))
        .route("/api/refresh_image", get(image_refresh_handler))
        .route(
            "/api/watering_history/:entries",
            get(watering_history_handler),
        )
        .route("/api/graceful_shutdown", get(graceful_shutdown_handler))
        .route("/image", get(image_handler))
        .route("/*path", get(site_handler))
        .route("/", get(root_handler))
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

#[derive(rust_embed::RustEmbed)]
#[folder = "html/growpi/dist/"]
struct Asset;

async fn site_handler(Path(path): Path<String>) -> Response {
    serve_site(Some(path))
}
async fn root_handler() -> Response {
    serve_site(None)
}

fn serve_site(path: Option<String>) -> Response {
    let path = match path {
        Some(path) => path,
        None => "index.html".to_string(),
    };
    let asset = Asset::get(&path).or_else(|| Asset::get("index.html"));

    let content = asset
        .as_ref()
        .and_then(|file| std::str::from_utf8(&file.data).ok());
    let content = content.unwrap_or("Sorry, couldn't load that asset :(");

    let mut response = content.to_string().into_response();

    let mime_type = mime_guess::from_path(path).first_or_text_plain();
    let header_value = HeaderValue::from_str(mime_type.as_ref());
    if let Ok(header_value) = header_value {
        response
            .headers_mut()
            .append(header::CONTENT_TYPE, header_value);
    }
    response
}

async fn image_handler() -> Response {
    let bytes = std::fs::read(control::imaging::get_image_path());
    let response = bytes.map(|bytes| {
        let mut r = bytes.into_response();
        r.headers_mut()
            .append(header::CONTENT_TYPE, HeaderValue::from_static("image/jpeg"));
        r
    });
    response.unwrap_or_else(|e| format!("Error: {}", e).into_response())
}
async fn image_refresh_handler(State(program_state): State<ProgramStateShared>) -> Response {
    let _ = control::imaging::save_latest_image(program_state).await;
    StatusCode::OK.into_response()
}

async fn watering_history_handler(
    Path(entries): Path<usize>,
    State(program_state): State<ProgramStateShared>,
) -> Response {
    let records = lock_state(&program_state).map(|state| {
        state
            .history
            .watering_records
            .iter()
            .rev()
            .take(entries)
            .cloned()
            .collect::<Vec<_>>()
    });
    let records = records.map(Json);
    match records {
        Ok(records) => records.into_response(),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

async fn graceful_shutdown_handler() -> Response {
    match system_shutdown::shutdown() {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}
