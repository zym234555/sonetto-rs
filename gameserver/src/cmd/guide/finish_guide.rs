use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use prost::Message;
use sonettobuf::{CmdId, FinishGuideReply, FinishGuideRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_finish_guide(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = FinishGuideRequest::decode(&req.data[..])?;

    tracing::info!("Received FinishGuideRequest: {:?}", request);

    let reply = FinishGuideReply {};

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::FinishGuideCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
