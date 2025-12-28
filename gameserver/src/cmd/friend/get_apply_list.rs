use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use sonettobuf::{CmdId, GetApplyListReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_apply_list(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let reply = GetApplyListReply { info: vec![] };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetApplyListCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
