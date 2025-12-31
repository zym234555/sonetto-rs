use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use prost::Message;
use sonettobuf::{CmdId, SetPortraitRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_set_portrait(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = SetPortraitRequest::decode(&req.data[..])?;
    tracing::info!("Received SetPortraitRequest: {:?}", request);

    let portrait = request.portrait.ok_or(AppError::InvalidRequest)?;

    let _ = {
        let ctx_guard = ctx.lock().await;
        let player_id = ctx_guard.player_id.ok_or(AppError::NotLoggedIn)?;
        let pool = &ctx_guard.state.db;

        sqlx::query("UPDATE player_info SET portrait = ? WHERE player_id = ?")
            .bind(portrait)
            .bind(player_id)
            .execute(pool)
            .await?;

        tracing::info!("User {} updated portrait to {}", player_id, portrait);

        player_id
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_empty_reply(CmdId::SetPortraitCmd, Vec::new(), 0, req.up_tag)
        .await?;

    Ok(())
}
