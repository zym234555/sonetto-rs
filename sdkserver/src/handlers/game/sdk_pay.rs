use crate::models::request::PaymentPageQuery;
use axum::extract::Query;
use axum::response::Html;

pub async fn get(Query(params): Query<PaymentPageQuery>) -> Html<String> {
    let goods_name = params.goods_name.as_deref().unwrap_or("Item");
    let amount = params.amount.as_deref().unwrap_or("0");
    let currency = params.currency.as_deref().unwrap_or("USD");

    tracing::info!(
        "Serving payment page: goods={}, amount={} {}",
        goods_name,
        amount,
        currency
    );

    let callback_url = format!(
        "/SDKStaticPage/pcpay/callback.html?orderKey=SONETTO_FREE_{}&paymentStatus=AUTHORISED&paymentAmount={}&paymentCurrency={}&mac2=free_local_server",
        chrono::Utc::now().timestamp_millis(),
        amount,
        currency
    );

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="refresh" content="0;url={}">
    <title>Processing Payment...</title>
</head>
<body>
    <p>Processing payment, please wait...</p>
    <script>
        window.location.href = "{}";
    </script>
</body>
</html>"#,
        callback_url, callback_url
    );

    Html(html)
}
