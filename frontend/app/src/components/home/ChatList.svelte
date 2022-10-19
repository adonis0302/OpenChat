<script lang="ts">
    import CurrentUser from "./CurrentUser.svelte";
    import Search from "../Search.svelte";
    import Loading from "../Loading.svelte";
    import ChatSummary from "./ChatSummary.svelte";
    import ThreadsSection from "./ThreadsSection.svelte";
    import { _ } from "svelte-i18n";
    import type {
        ChatSummary as ChatSummaryType,
        GroupMatch,
        GroupSearchResponse,
        MessageMatch,
        SearchAllMessagesResponse,
        CreatedUser,
        UserSummary,
        DataContent,
        OpenChat,
    } from "openchat-client";
    import { createEventDispatcher, getContext, onMount, tick } from "svelte";
    import SearchResult from "./SearchResult.svelte";
    import { push } from "svelte-spa-router";
    import NotificationsBar from "./NotificationsBar.svelte";
    import Markdown from "./Markdown.svelte";
    import { chatListScroll } from "../../stores/scrollPos";
    import { menuCloser } from "../../actions/closeMenu";

    const client = getContext<OpenChat>("client");

    export let groupSearchResults: Promise<GroupSearchResponse> | undefined = undefined;
    export let userSearchResults: Promise<UserSummary[]> | undefined = undefined;
    export let messageSearchResults: Promise<SearchAllMessagesResponse> | undefined = undefined;
    export let searchTerm: string = "";
    export let searching: boolean = false;
    export let searchResultsAvailable: boolean = false;
    export let createdUser: CreatedUser;

    const dispatch = createEventDispatcher();

    let chatsWithUnreadMsgs: number;

    $: selectedChatId = client.selectedChatId;
    $: numberOfThreadsStore = client.numberOfThreadsStore;
    $: chatsLoading = client.chatsLoading;
    $: chatSummariesStore = client.chatSummariesStore;
    $: messagesRead = client.messagesRead;
    $: chatSummariesListStore = client.chatSummariesListStore;
    $: userStore = client.userStore;
    $: user = $userStore[createdUser.userId];
    $: userId = createdUser.userId;
    $: lowercaseSearch = searchTerm.toLowerCase();

    function chatMatchesSearch(chat: ChatSummaryType): boolean {
        if (chat.kind === "group_chat") {
            return (
                chat.name.toLowerCase().indexOf(lowercaseSearch) >= 0 ||
                chat.description.toLowerCase().indexOf(lowercaseSearch) >= 0
            );
        }

        if (chat.kind === "direct_chat") {
            const username = $userStore[chat.them]?.username;
            return username ? username.toLowerCase().indexOf(lowercaseSearch) >= 0 : false;
        }
        return false;
    }

    $: chats =
        searchTerm !== ""
            ? $chatSummariesListStore.filter(chatMatchesSearch)
            : $chatSummariesListStore;

    $: {
        document.title = chatsWithUnreadMsgs > 0 ? `OpenChat (${chatsWithUnreadMsgs})` : "OpenChat";
    }

    function chatWith(userId: string): void {
        dispatch("chatWith", userId);
        closeSearch();
    }

    function loadMessage(msg: MessageMatch): void {
        dispatch("loadMessage", msg);
    }

    /**
     * All we need to do here is push the route
     * the routing will take care of the rest
     */
    function selectGroup({ chatId }: GroupMatch): void {
        push(`/${chatId}`);
        closeSearch();
    }

    function messageMatchDataContent({ chatId, sender }: MessageMatch): DataContent {
        const chat = $chatSummariesStore[chatId];
        if (chat === undefined) {
            return { blobUrl: undefined };
        }
        return chat.kind === "group_chat" ? chat : $userStore[sender];
    }

    function messageMatchTitle({ chatId, sender }: MessageMatch): string {
        const chat = $chatSummariesStore[chatId];
        if (chat === undefined) {
            return "";
        }
        return chat.kind === "group_chat" ? chat.name : $userStore[sender].username ?? "";
    }

    function closeSearch() {
        dispatch("searchEntered", "");
    }

    function chatSelected(ev: CustomEvent<string>): void {
        chatScrollTop = chatListElement.scrollTop;
        push(`/${ev.detail}`);
        closeSearch();
    }

    let chatListElement: HTMLElement;
    let chatScrollTop = 0;

    onMount(() => {
        tick().then(() => {
            if (chatListElement) {
                chatListElement.scrollTop = $chatListScroll;
            }
        });

        let unsub = messagesRead.subscribe((_val) => {
            chatsWithUnreadMsgs = chats
                ? chats.reduce(
                      (num, chat) =>
                          client.unreadMessageCount(
                              chat.chatId,
                              chat.latestMessage?.event.messageIndex
                          ) > 0
                              ? num + 1
                              : num,
                      0
                  )
                : 0;
        });

        return () => {
            unsub();
            chatListScroll.set(chatScrollTop);
        };
    });
</script>

{#if user}
    <CurrentUser
        on:userAvatarSelected
        on:logout
        on:whatsHot
        on:showAbout
        on:showFaq
        on:showRoadmap
        on:showArchitecture
        on:showFeatures
        on:showWhitepaper
        {user}
        on:profile
        on:newGroup />
    <Search {searching} {searchTerm} on:searchEntered />

    <div use:menuCloser bind:this={chatListElement} class="body">
        {#if $chatsLoading}
            <Loading />
        {:else}
            <div class="chat-summaries">
                {#if searchResultsAvailable && chats.length > 0}
                    <h3 class="search-subtitle">{$_("yourChats")}</h3>
                {/if}
                {#if $numberOfThreadsStore > 0}
                    <ThreadsSection />
                {/if}
                {#each chats as chatSummary, i (chatSummary.chatId)}
                    <ChatSummary
                        index={i}
                        {chatSummary}
                        {userId}
                        selected={$selectedChatId === chatSummary.chatId}
                        visible={searchTerm !== "" || !chatSummary.archived}
                        on:chatSelected={chatSelected}
                        on:pinChat
                        on:unpinChat
                        on:archiveChat
                        on:unarchiveChat
                        on:toggleMuteNotifications
                        on:deleteDirectChat />
                {/each}

                {#if groupSearchResults !== undefined}
                    <div class="search-matches">
                        {#await groupSearchResults then resp}
                            {#if resp.kind === "success" && resp.matches.length > 0}
                                <h3 class="search-subtitle">{$_("publicGroups")}</h3>
                                {#each resp.matches as group, i (group.chatId)}
                                    <SearchResult
                                        index={i}
                                        avatarUrl={client.groupAvatarUrl(group)}
                                        on:click={() => selectGroup(group)}>
                                        <h4 class="search-item-title">
                                            {group.name}
                                        </h4>
                                        <p title={group.description} class="search-item-desc">
                                            {group.description}
                                        </p>
                                    </SearchResult>
                                {/each}
                            {/if}
                        {/await}
                    </div>
                {/if}
                {#if userSearchResults !== undefined}
                    <div class="search-matches">
                        {#await userSearchResults then resp}
                            {#if resp.length > 0}
                                <h3 class="search-subtitle">{$_("users")}</h3>
                                {#each resp as user, i (user.userId)}
                                    <SearchResult
                                        index={i}
                                        avatarUrl={client.userAvatarUrl(user)}
                                        on:click={() => chatWith(user.userId)}>
                                        <h4 class="search-item-title">
                                            @{user.username}
                                        </h4>
                                    </SearchResult>
                                {/each}
                            {/if}
                        {/await}
                    </div>
                {/if}
                {#if messageSearchResults !== undefined}
                    <div class="search-matches">
                        {#await messageSearchResults then resp}
                            {#if resp.kind == "success" && resp.matches.length > 0}
                                <h3 class="search-subtitle">{$_("messages")}</h3>
                                {#each resp.matches as msg, i (`${msg.chatId}_${msg.messageIndex}`)}
                                    <SearchResult
                                        index={i}
                                        avatarUrl={client.groupAvatarUrl(
                                            messageMatchDataContent(msg)
                                        )}
                                        showSpinner={false}
                                        on:click={() => loadMessage(msg)}>
                                        <h4 class="search-item-title">
                                            {messageMatchTitle(msg)}
                                        </h4>
                                        <div class="search-item-desc">
                                            <Markdown
                                                text={client.getContentAsText($_, msg.content)}
                                                oneLine={true}
                                                suppressLinks={true} />
                                        </div>
                                    </SearchResult>
                                {/each}
                            {/if}
                        {/await}
                    </div>
                {/if}
            </div>
        {/if}
    </div>
    <NotificationsBar />
{/if}

<style type="text/scss">
    .body {
        overflow: auto;
        @include nice-scrollbar();
        @include mobile() {
            padding: 0 $sp3;
        }
    }
    .chat-summaries {
        overflow: auto;
        overflow-x: hidden;
    }

    .search-subtitle {
        margin-bottom: $sp3;
        margin-left: 0;
        color: var(--chatSearch-section-txt);
    }

    .search-matches {
        margin-top: $sp4;
    }
    .search-item-title {
        margin-bottom: $sp3;
    }
    .search-item-desc {
        color: var(--chatSummary-txt2);
        @include font(light, normal, fs-80);
        @include ellipsis();
    }
</style>