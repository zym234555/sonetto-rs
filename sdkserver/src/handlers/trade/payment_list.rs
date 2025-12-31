use crate::AppState;
use crate::models::request::PaymentListReq;
use crate::models::response::{PaymentListRsp, PaymentListRspData, PaymentMethod};
use axum::extract::State;
use axum::response::Json;

pub async fn post(
    State(_): State<AppState>,
    axum::Json(req): axum::Json<PaymentListReq>,
) -> Json<PaymentListRsp> {
    tracing::info!("Received payment list request for user: {}", req.user_id);

    let response = PaymentListRsp {
        code: 200,
        msg: "success".to_string(),
        data: PaymentListRspData {
            payments: vec![PaymentMethod {
                payment_method_type: "ALL".to_string(),
                payment_method: "1012".to_string(),
                payment_method_name: "Sonetto-Rs".to_string(),
                icon_url: "https://gamecms-res-hw.sl916.com/payment-method/worldpay.png"
                    .to_string(),
                pay_channel_id: 9,
                other_payment_methods: None,
                ext_payment_method_params: None,
            }],
            web_pre_pay_url: format!(
                "http://127.0.0.1:21000/sdk-pc-pay/pcpay.html?timestamp={}",
                chrono::Utc::now().timestamp_millis()
            ),
        },
    };

    tracing::info!("Returning {} payment methods", response.data.payments.len());

    Json(response)
}
