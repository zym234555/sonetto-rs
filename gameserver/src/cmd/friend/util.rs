use crate::error::AppError;
use crate::state::ConnectionContext;
use sonettobuf::{ChatMsg, CmdId};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn send_bot_welcome(
    ctx: &mut ConnectionContext,
    player_id: u64,
    msg_id: u64,
) -> Result<(), AppError> {
    if ctx.bot_welcome_sent {
        return Ok(());
    }

    let msg = ChatMsg {
        msg_id: Some(msg_id),
        sender_id: Some(1337),
        channel_type: Some(1),
        sender_name: Some("Sonetto Bot".to_string()),
        portrait: Some(171805),
        content: Some(
            "Welcome to Sonetto-rs\nGM commands available\nType: /help for commands".to_string(),
        ),
        send_time: None,
        level: Some(80),
        recipient_id: Some(player_id),
        msg_type: Some(0),
        ext_data: Some(String::new()),
    };

    ctx.send_push(
        CmdId::ChatMsgPushCmd,
        sonettobuf::ChatMsgPush { msg: vec![msg] },
    )
    .await?;

    ctx.bot_welcome_sent = true;
    Ok(())
}

pub async fn handle_command(ctx: Arc<Mutex<ConnectionContext>>, input: &str) -> Option<String> {
    if !input.starts_with("/") {
        return None;
    }

    match crate::cmd::gm::execute_command(ctx, input).await {
        Ok(response) => Some(response),
        Err(e) => Some(format!("Error: {:?}", e)),
    }
}
