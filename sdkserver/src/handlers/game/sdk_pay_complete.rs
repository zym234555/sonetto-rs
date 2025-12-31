use axum::{extract::Query, response::Html};

use crate::models::request::CallbackQuery;

pub async fn get(Query(params): Query<CallbackQuery>) -> Html<String> {
    tracing::info!(
        "Payment callback: status={:?}, amount={:?}",
        params.payment_status,
        params.payment_amount
    );

    let status = if params.payment_status.as_deref() == Some("AUTHORISED") {
        "done"
    } else {
        params.payment_status.as_deref().unwrap_or("unknown")
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>callback</title>
</head>
<body>
    <script>
        function onWebViewReady() {{
            onXsPayFinish('{}', '{}', '{}', '{}');
        }}

        // Auto-call if onXsPayFinish exists
        if (typeof onXsPayFinish === 'function') {{
            onXsPayFinish('{}', '{}', '{}', '{}');
        }}

        // Also try calling when page loads
        window.onload = function() {{
            if (typeof onXsPayFinish === 'function') {{
                onXsPayFinish('{}', '{}', '{}', '{}');
            }}
        }};
    </script>
</body>
</html>"#,
        params.user_id.as_deref().unwrap_or(""),
        status,
        params.foreign_invoice.as_deref().unwrap_or(""),
        params.invoice_id.as_deref().unwrap_or(""),
        params.user_id.as_deref().unwrap_or(""),
        status,
        params.foreign_invoice.as_deref().unwrap_or(""),
        params.invoice_id.as_deref().unwrap_or(""),
        params.user_id.as_deref().unwrap_or(""),
        status,
        params.foreign_invoice.as_deref().unwrap_or(""),
        params.invoice_id.as_deref().unwrap_or("")
    );

    Html(html)
}
