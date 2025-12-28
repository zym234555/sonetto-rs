use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::state::ConnectionContext;
use sonettobuf::{CmdId, FriendInfo, GetRecommendedFriendsReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_get_recommended_friends(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let now = common::time::ServerTime::now_ms();
    let friends = vec![FriendInfo {
        user_id: Some(1337),
        level: Some(80),
        time: Some(now as u64),
        name: Some("Sonetto Bot".to_string()),
        portrait: Some(171603),
        desc: Some("Server commands sent here".to_string()),
        infos: vec![],
        bg: None,
    }];

    let reply = GetRecommendedFriendsReply {
        info: friends,
        message: Some("Sonetto Bot".to_string()),
    };

    let mut ctx_guard = ctx.lock().await;
    ctx_guard
        .send_reply(CmdId::GetRecommendedFriendsCmd, reply, 0, req.up_tag)
        .await?;

    Ok(())
}
