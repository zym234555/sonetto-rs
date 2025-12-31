use crate::AppState;
use crate::models::request::OrderReq;
use crate::models::response::{OrderRsp, OrderRspData};
use axum::extract::State;
use axum::response::Json;

pub async fn post(
    State(_): State<AppState>,
    axum::Json(req): axum::Json<OrderReq>,
) -> Json<OrderRsp> {
    tracing::info!(
        "Received order request: goods_id={}, game_order_id={}",
        req.goods_id,
        req.game_order_id
    );

    let order_id = format!(
        "{}{}",
        chrono::Utc::now().timestamp(),
        rand::random::<u32>() % 1000
    );

    let response = OrderRsp {
        code: 200,
        msg: "success".to_string(),
        data: OrderRspData {
            order_id,
            pay_notify_url: "".to_string(),
            ext_params: format!(
                r#"{{"sign":"{}","timestamp":"{}"}}"#,
                "54ba11fed654b46039956329afc44391",
                chrono::Utc::now().timestamp_millis()
            ),
        },
    };

    tracing::info!(
        "Returning order response: order_id={}",
        response.data.order_id
    );

    Json(response)
}
