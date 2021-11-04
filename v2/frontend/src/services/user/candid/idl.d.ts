import type { IDL } from "@dfinity/candid";
import {
    _SERVICE,
    ReplyContext,
    DirectReplyContextArgs,
    ChatSummaryUpdates,
    GroupChatSummaryUpdates,
    DirectChatSummaryUpdates,
    DirectChatEventWrapper,
    CreateGroupArgs,
    CreateGroupResponse,
    UpdatesArgs,
    UpdatesResponse,
    InitialStateResponse,
    ChatSummary,
    GroupChatSummary,
    DirectChatSummary,
    Message,
    DirectChatEvent,
    UserId,
    MessageContent,
    FileContent,
    TextContent,
    ImageContent,
    VideoContent,
    AudioContent,
    CryptocurrencyContent,
    CryptocurrencyTransfer,
    ICPTransfer,
    CyclesTransfer,
    PendingCyclesTransfer,
    CompletedCyclesTransfer,
    FailedCyclesTransfer,
    PendingICPTransfer,
    CompletedICPTransfer,
    FailedICPTransfer,
    DeletedContent,
    TimestampMillis,
    BlobReference,
    Participant,
    UpdatedChatSummary,
    EventIndex,
    EventWrapper,
    EventsByIndexArgs,
    EventsByIndexResponse,
    EventsResponse,
    EventsSuccessResult,
    EventsArgs,
    PutChunkArgs,
    PutChunkResponse,
    SendMessageArgs,
    SendMessageResponse,
    EditMessageResponse,
    BlockUserResponse,
    UnblockUserResponse,
    LeaveGroupResponse,
    MarkReadResponse,
    SetAvatarResponse,
    ToggleReactionResponse,
    DeleteMessagesResponse,
    DeletedMessage,
    UpdatedMessage,
    JoinGroupResponse,
    SearchAllMessagesResponse,
    MessageMatch,
    MuteNotificationsResponse,
    UnmuteNotificationsResponse,
    Alert,
    AlertDetails,
    CryptocurrencyDeposit,
    Role,
} from "./types";
export {
    _SERVICE as UserService,
    SendMessageArgs as ApiSendMessageArgs,
    SendMessageResponse as ApiSendMessageResponse,
    EditMessageResponse as ApiEditMessageResponse,
    Message as ApiMessage,
    ReplyContext as ApiReplyContext,
    DirectReplyContextArgs as ApiDirectReplyContextArgs,
    ChatSummaryUpdates as ApiChatSummaryUpdates,
    GroupChatSummaryUpdates as ApiGroupChatSummaryUpdates,
    DirectChatSummaryUpdates as ApiDirectChatSummaryUpdates,
    CreateGroupArgs as ApiCreateGroupArgs,
    CreateGroupResponse as ApiCreateGroupResponse,
    UpdatesArgs as ApiUpdatesArgs,
    UpdatesResponse as ApiUpdatesResponse,
    InitialStateResponse as ApiInitialStateResponse,
    ChatSummary as ApiChatSummary,
    DirectChatEvent as ApiDirectChatEvent,
    DirectChatEventWrapper as ApiDirectChatEventWrapper,
    GroupChatSummary as ApiGroupChatSummary,
    DirectChatSummary as ApiDirectChatSummary,
    UserId as ApiUserId,
    MessageContent as ApiMessageContent,
    FileContent as ApiFileContent,
    TextContent as ApiTextContent,
    ImageContent as ApiImageContent,
    VideoContent as ApiVideoContent,
    AudioContent as ApiAudioContent,
    DeletedContent as ApiDeletedContent,
    CryptocurrencyContent as ApiCryptocurrencyContent,
    CryptocurrencyTransfer as ApiCryptocurrencyTransfer,
    ICPTransfer as ApiICPTransfer,
    CyclesTransfer as ApiCyclesTransfer,
    PendingCyclesTransfer as ApiPendingCyclesTransfer,
    CompletedCyclesTransfer as ApiCompletedCyclesTransfer,
    FailedCyclesTransfer as ApiFailedCyclesTransfer,
    PendingICPTransfer as ApiPendingICPTransfer,
    CompletedICPTransfer as ApiCompletedICPTransfer,
    FailedICPTransfer as ApiFailedICPTransfer,
    TimestampMillis as ApiTimestampMillis,
    BlobReference as ApiBlobReference,
    Participant as ApiParticipant,
    UpdatedChatSummary as ApiUpdatedChatSummary,
    EventIndex as ApiEventIndex,
    EventWrapper as ApiEventWrapper,
    EventsByIndexArgs as ApiEventsByIndexArgs,
    EventsByIndexResponse as ApiEventsByIndexResponse,
    EventsResponse as ApiEventsResponse,
    EventsSuccessResult as ApiEventsSuccessResult,
    EventsArgs as ApiEventsArgs,
    PutChunkArgs as ApiPutChunkArgs,
    PutChunkResponse as ApiPutChunkResponse,
    BlockUserResponse as ApiBlockUserResponse,
    UnblockUserResponse as ApiUnblockUserResponse,
    LeaveGroupResponse as ApiLeaveGroupResponse,
    MarkReadResponse as ApiMarkReadResponse,
    SetAvatarResponse as ApiSetAvatarResponse,
    ToggleReactionResponse as ApiToggleReactionResponse,
    DeleteMessagesResponse as ApiDeleteMessageResponse,
    DeletedMessage as ApiDeletedMessage,
    UpdatedMessage as ApiUpdatedMessage,
    JoinGroupResponse as ApiJoinGroupResponse,
    SearchAllMessagesResponse as ApiSearchAllMessagesResponse,
    MessageMatch as ApiMessageMatch,
    MuteNotificationsResponse as ApiMuteNotificationsResponse,
    UnmuteNotificationsResponse as ApiUnmuteNotificationsResponse,
    Alert as ApiAlert,
    AlertDetails as ApiAlertDetails,
    CryptocurrencyDeposit as ApiCryptocurrencyDeposit,
    Role as ApiRole,
};

export const idlFactory: IDL.InterfaceFactory;
