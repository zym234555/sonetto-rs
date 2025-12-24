use crate::cmd::system::util::*;
use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use crate::utils::push::send_red_dot_push;

use common::time::ServerTime;
use database::db::game::sign_in;
use sonettobuf::CmdId;

use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_login(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    tracing::info!("→ Starting login handler");

    let login = parse_login_request(&req.data)?;
    let user_id = extract_user_id(&login.account_id)?;

    tracing::info!("→ Login attempt user_id={}", user_id);

    let (stored_token, token_expires_at) = {
        let db = {
            let ctx = ctx.lock().await;
            ctx.state.db.clone()
        };

        let row = sqlx::query("SELECT token, token_expires_at FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(&db)
            .await?
            .ok_or_else(|| AppError::Custom("User not found".into()))?;

        (
            row.try_get::<String, _>("token")?,
            row.try_get::<Option<i64>, _>("token_expires_at")?,
        )
    };

    if stored_token != login.token {
        return login_error(&ctx, "Invalid token", req.up_tag).await;
    }

    let now = ServerTime::now_ms() as i64;
    if token_expires_at.is_some_and(|exp| now > exp) {
        return login_error(&ctx, "Token expired", req.up_tag).await;
    }

    tracing::info!("✓ Token validated for user_id={}", user_id);

    {
        let mut ctx_guard = ctx.lock().await;
        ctx_guard.load_player_state(user_id).await?;
    }

    {
        let db = {
            let ctx = ctx.lock().await;
            ctx.state.db.clone()
        };

        let (is_new_day, is_new_week, _is_new_month) =
            sign_in::process_daily_login(&db, user_id).await?;

        if is_new_day {
            sign_in::reset_daily_counters(&db, user_id).await?;
        }

        if is_new_week {
            sign_in::reset_weekly_counters(&db, user_id).await?;
        }
    }

    {
        let mut ctx_guard = ctx.lock().await;
        let now = ServerTime::now_ms();

        ctx_guard
            .update_and_save_player_state(|state| {
                state.last_sign_in_time = Some(now);
                state.last_sign_in_day = ServerTime::server_day(now);

                if state.is_new_server_day(now) {
                    state.last_daily_reset_time = Some(now);
                }

                if state.is_new_week(now) {
                    state.last_weekly_reset_time = Some(now);
                }

                if state.is_new_month(now) {
                    state.last_monthly_reset_time = Some(now);
                }
            })
            .await?;
    }

    send_red_dot_push(Arc::clone(&ctx), user_id, Some(vec![2218, 2220, 2221])).await?;
    send_red_dot_push(Arc::clone(&ctx), user_id, Some(vec![2240])).await?;
    send_red_dot_push(Arc::clone(&ctx), user_id, Some(vec![2230])).await?;
    send_critter_push(Arc::clone(&ctx), user_id).await?;

    {
        let mut ctx_guard = ctx.lock().await;
        let payload = build_login_reply(user_id);

        ctx_guard
            .send_raw_reply_fixed(CmdId::LoginRequestCmd, payload, 0, req.up_tag)
            .await?;
    }

    ConnectionContext::register(Arc::clone(&ctx)).await;

    tracing::info!("✓ Login successful for user_id={}", user_id);
    Ok(())
}
