use axum::{
    routing::{get, post},
    http::StatusCode,
    response::Redirect,
    extract::Json,
    Router
};
use serde::{Deserialize, Serialize};
use std::{fs, process::Command};
use base64::{engine::general_purpose, Engine as _};
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct Save2Json {
    binfile: String,
    slot: u32,
}

#[derive(Deserialize)]
struct Json2Save {
    binfile: String,
    jsonfile: String,
    slot: u32,
}

#[derive(Serialize)]
struct GenericResponse {
    status: String,
    payload: String,
}

#[cfg(target_os = "windows")]
const BIN_PATH: &str = "./mh3se-cli.exe";
#[cfg(not(target_os = "windows"))]
const BIN_PATH: &str = "./mh3se-cli";

async fn save2json(Json(payload): Json<Save2Json>) -> Result<Json<GenericResponse>, StatusCode> {
    let json_fname = gen_id();
    let bin_fname = gen_id();

    println!("[save2json] Decoding base64 save file...");
    let bin_data = general_purpose::STANDARD.decode(&payload.binfile).map_err(|_| StatusCode::BAD_REQUEST)?;

    println!("[save2json] Writing save file to tmp/{}...", bin_fname);
    fs::write(format!("tmp/{}", bin_fname), bin_data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("[save2json] Executing {} decode tmp/{} tmp/{} {}...", BIN_PATH, bin_fname, json_fname, payload.slot);
    let decode_cmd = Command::new(BIN_PATH)
        .arg("decode")
        .arg(format!("tmp/{}", bin_fname))
        .arg(format!("tmp/{}", json_fname))
        .arg(payload.slot.to_string())
        .output()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("[save2json] Sending back json result and removing temporary files...");
    if decode_cmd.status.success() {
        let json_output = fs::read_to_string(format!("tmp/{}", json_fname)).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        fs::remove_file(format!("tmp/{}", bin_fname)).ok();
        fs::remove_file(format!("tmp/{}", json_fname)).ok();
        Ok(Json(GenericResponse{status: String::from_utf8(decode_cmd.stdout).unwrap(), payload: json_output}))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn json2save(Json(payload): Json<Json2Save>) -> Result<Json<GenericResponse>, StatusCode> {
    let json_fname = gen_id();
    let in_bin_fname = gen_id();
    let out_bin_fname = gen_id();

    println!("[json2save] Decoding base64 save file...");
    let bin_data = general_purpose::STANDARD.decode(&payload.binfile).map_err(|_| StatusCode::BAD_REQUEST)?;

    println!("[json2save] Writing save file to tmp/{}...", in_bin_fname);
    fs::write(format!("tmp/{}", in_bin_fname), bin_data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("[json2save] Writing json file to tmp/{}...", json_fname);
    fs::write(format!("tmp/{}", json_fname), &payload.jsonfile).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("[json2save] Executing {} encode tmp/{} tmp/{} tmp/{} {}...", BIN_PATH, json_fname, in_bin_fname, out_bin_fname, payload.slot);
    let encode_cmd = Command::new(BIN_PATH)
        .arg("encode")
        .arg(format!("tmp/{}", json_fname))
        .arg(format!("tmp/{}", in_bin_fname))
        .arg(format!("tmp/{}", out_bin_fname))
        .arg(payload.slot.to_string())
        .output()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("[json2save] Sending back newly built save file and removing temporary files...");
    if encode_cmd.status.success() {
        let out_bin = fs::read(format!("tmp/{}", out_bin_fname)).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        fs::remove_file(format!("tmp/{}", json_fname)).ok();
        fs::remove_file(format!("tmp/{}", in_bin_fname)).ok();
        fs::remove_file(format!("tmp/{}", out_bin_fname)).ok();
        Ok(Json(GenericResponse{status: String::from_utf8(encode_cmd.stdout).unwrap(), payload: general_purpose::STANDARD.encode(out_bin)}))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

fn gen_id() -> String {
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    thread_rng().sample_iter(&Alphanumeric).take(8).map(char::from).collect()
}

#[tokio::main]
async fn main() {
    fs::create_dir_all("tmp").unwrap();
    let app = Router::new()
        .route("/", get(|| async { Redirect::temporary("/web/index.html") }))
        .route("/save2json", post(save2json))
        .route("/json2save", post(json2save))
        .nest_service("/web", ServeDir::new("web"));
    println!("[main] Opening server on http://127.0.0.1:8000/...");
    axum::serve(tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap(), app).await.unwrap();
}
