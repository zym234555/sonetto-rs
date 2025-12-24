use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use prost::Message;
use sonettobuf::{CmdId, ReadMailBatchReply, ReadMailBatchRequest};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_read_mail_batch(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let request = ReadMailBatchRequest::decode(&req.data[..])?;

    let r#type = request.r#type.ok_or(AppError::InvalidRequest)?;

    tracing::info!("Received ReadMailBatchRequest type {}", r#type);

    {
        let mut ctx_guard = ctx.lock().await;

        let reply = ReadMailBatchReply {
            incr_ids: vec![279048737],
        };
        ctx_guard
            .send_reply(CmdId::ReadMailBatchCmd, reply, 0, req.up_tag)
            .await?;
    }

    Ok(())
}
