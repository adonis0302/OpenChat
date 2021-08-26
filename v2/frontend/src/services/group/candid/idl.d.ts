import type { IDL } from "@dfinity/candid";
import {
    _SERVICE,
    GroupMessage,
    GroupReplyContext,
    UserId,
    ReplyContext,
    MessageContent,
    FileContent,
    TextContent,
    MediaContent,
    TimestampMillis,
    BlobReference,
    EventIndex,
    GroupChatEventWrapper,
    EventsByIndexArgs,
    EventsByIndexResponse,
    EventsResponse,
    EventsSuccessResult,
    EventsArgs,
    GroupChatEvent,
    AddParticipantsResponse,
} from "./types";
export {
    _SERVICE as GroupService,
    GroupMessage as ApiGroupMessage,
    UserId as ApiUserId,
    GroupReplyContext as ApiGroupReplyContext,
    MessageContent as ApiMessageContent,
    FileContent as ApiFileContent,
    TextContent as ApiTextContent,
    MediaContent as ApiMediaContent,
    TimestampMillis as ApiTimestampMillis,
    BlobReference as ApiBlobReference,
    EventIndex as ApiEventIndex,
    GroupChatEventWrapper as ApiEventWrapper,
    EventsByIndexArgs as ApiEventsByIndexArgs,
    EventsByIndexResponse as ApiEventsByIndexResponse,
    EventsResponse as ApiEventsResponse,
    EventsSuccessResult as ApiEventsSuccessResult,
    EventsArgs as ApiEventsArgs,
    GroupChatEvent as ApiGroupChatEvent,
    AddParticipantsResponse as ApiAddParticipantsResponse,
};

export const idlFactory: IDL.InterfaceFactory;
