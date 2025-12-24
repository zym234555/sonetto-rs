use crate::models::response::JspStartGameRsp;
use axum::response::Json;
use common::{game_port, host};

pub async fn get() -> Json<JspStartGameRsp> {
    let rsp = JspStartGameRsp {
        bak_ip: String::from(host()),
        bak_port: game_port(),
        ip: String::from(host()),
        port: game_port(),
        state: 1,
        ..Default::default()
    };

    Json(rsp)
}
