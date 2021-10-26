use candid::CandidType;
use serde::Deserialize;
use std::cmp::max;
use types::{ChatId, GroupChatSummaryUpdates, TimestampMillis, Timestamped};
use utils::range_set::{convert_to_message_index_ranges, RangeSet};

#[derive(CandidType, Deserialize)]
pub struct GroupChat {
    pub chat_id: ChatId,
    pub date_joined: TimestampMillis,
    pub read_by_me: Timestamped<RangeSet>,
    pub notifications_muted: Timestamped<bool>,
}

impl GroupChat {
    pub fn new(chat_id: ChatId, now: TimestampMillis) -> GroupChat {
        GroupChat {
            chat_id,
            date_joined: now,
            read_by_me: Timestamped::new(RangeSet::new(), now),
            notifications_muted: Timestamped::new(false, now),
        }
    }

    pub fn last_updated(&self) -> TimestampMillis {
        max(self.read_by_me.timestamp, self.notifications_muted.timestamp)
    }
}

impl From<&GroupChat> for GroupChatSummaryUpdates {
    fn from(s: &GroupChat) -> Self {
        GroupChatSummaryUpdates {
            chat_id: s.chat_id,
            last_updated: s.last_updated(),
            name: None,
            description: None,
            avatar_id: None,
            latest_message: None,
            latest_event_index: None,
            participant_count: None,
            read_by_me: Some(convert_to_message_index_ranges(s.read_by_me.value.clone())),
            notifications_muted: Some(s.notifications_muted.value),
        }
    }
}
