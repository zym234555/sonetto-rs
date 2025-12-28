use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use sonettobuf::{CmdId, GetBlacklistReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_blacklist(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = GetBlacklistReply { info: vec![] };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetBlacklistCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
