use crate::error::AppError;
use crate::packet::ClientPacket;
use crate::send_reply;
use crate::state::ConnectionContext;
use sonettobuf::{Act163EpisodeInfo, Act163Info, CmdId, Get163InfosReply};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn on_act163_get_info(
    ctx: Arc<Mutex<ConnectionContext>>,
    req: ClientPacket,
) -> Result<(), AppError> {
    let ep_infos = vec![
        Act163EpisodeInfo {
            episode_id: Some(1210501),
            pass_before_story: Some(true),
            pass_evidence: Some(false),
            pass_after_story: Some(false),
        },
        Act163EpisodeInfo {
            episode_id: Some(1210502),
            pass_before_story: Some(true),
            pass_evidence: Some(false),
            pass_after_story: Some(false),
        },
        Act163EpisodeInfo {
            episode_id: Some(1210503),
            pass_before_story: Some(true),
            pass_evidence: Some(true),
            pass_after_story: Some(true),
        },
        Act163EpisodeInfo {
            episode_id: Some(1210504),
            pass_before_story: Some(true),
            pass_evidence: Some(true),
            pass_after_story: Some(true),
        },
        Act163EpisodeInfo {
            episode_id: Some(1210505),
            pass_before_story: Some(true),
            pass_evidence: Some(false),
            pass_after_story: Some(false),
        },
        Act163EpisodeInfo {
            episode_id: Some(1210506),
            pass_before_story: Some(true),
            pass_evidence: Some(false),
            pass_after_story: Some(false),
        },
        Act163EpisodeInfo {
            episode_id: Some(1210507),
            pass_before_story: Some(true),
            pass_evidence: Some(false),
            pass_after_story: Some(false),
        },
        Act163EpisodeInfo {
            episode_id: Some(1210508),
            pass_before_story: Some(true),
            pass_evidence: Some(true),
            pass_after_story: Some(true),
        },
    ];

    let infos = Act163Info {
        activity_id: Some(12105),
        episode_info: ep_infos,
        read_clue_ids: vec![],
    };
    let act163 = Get163InfosReply {
        act_info: Some(infos),
    };

    let mut conn = ctx.lock().await;
    conn.send_reply(CmdId::Get163InfosCmd, act163, 0, req.up_tag)
        .await?;
    Ok(())
}
