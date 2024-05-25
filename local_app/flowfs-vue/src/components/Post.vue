<template>
  <div
    v-if="!editorVisible"
    class="post"
    :class="{ highlighted: highlighted }"
    :data-post-id="post.id"
    :id="post.id"
  >
    <div class="post-header">
      <universal-link
        class="floating-avatar"
        :to="getProfileLocation(post.account)"
        :title="author.getDisplayName()"
      >
        <template #link-content>
          <avatar :profile="post.account"></avatar>
        </template>
      </universal-link>
      <universal-link
        class="display-name-link"
        :to="getProfileLocation(post.account)"
        :title="author.getDisplayName()"
      >
        <template #link-content>
          <profile-display-name :profile="author"></profile-display-name>
        </template>
      </universal-link>
      <div
        class="actor-address"
        :title="getActorHandle(post.account)"
      >
        {{ getActorHandle(post.account) }}
      </div>
      <button
        v-if="inThread && post.in_reply_to_id"
        class="icon"
        title="Go to previous post"
        @mouseover="highlight(post.in_reply_to_id)"
        @mouseleave="highlight(null)"
        @click.prevent="scrollTo(post.in_reply_to_id as string)"
      >
        <icon-left-up></icon-left-up>
      </button>
      <span
        class="icon icon-small"
        :title="getVisibilityDisplay()"
      >
        <visibility-icon :visibility="post.visibility"></visibility-icon>
      </span>
      <span
        v-if="post.pinned"
        class="icon icon-small"
        title="Featured"
      >
        <icon-pin></icon-pin>
      </span>
      <span v-if="post.edited_at">edited</span>
      <router-link
        v-if="currentUser && inThread"
        class="timestamp"
        :to="{ name: 'post', params: { postId: post.id } }"
        :title="formatDateTime(post.created_at)"
      >
        <span @click.prevent="scrollTo(post.id)">
          {{ humanizeDate(post.created_at) }}
        </span>
      </router-link>
      <universal-link
        v-else
        class="timestamp"
        :to="getPostLocation(post)"
        :title="formatDateTime(post.created_at)"
      >
        <template #link-content>
          {{ humanizeDate(post.created_at) }}
        </template>
      </universal-link>
    </div>
    <div class="post-subheader" v-if="getReplyMentions().length > 0">
      <span>replying to</span>
      <universal-link
        v-for="mention in getReplyMentions()"
        :key="mention.id"
        :to="getProfileLocation(mention)"
        :title="getActorHandle(mention)"
      >
        <template #link-content>
          @{{ mention.username }}
        </template>
      </universal-link>
    </div>
    <post-content v-if="post.content" :post="post"></post-content>
    <div class="post-attachments" v-if="post.media_attachments.length > 0">
      <post-attachment
        v-for="attachment in post.media_attachments"
        :attachment="attachment"
        :is-sensitive="post.sensitive"
        :key="attachment.id"
      ></post-attachment>
    </div>
    <universal-link
      v-for="linkedPost in post.links"
      class="post-quote"
      :to="getPostLocation(linkedPost)"
      :key="linkedPost.id"
    >
      <template #link-content>
        <post-preview :post="linkedPost"></post-preview>
      </template>
    </universal-link>
    <div class="post-footer">
      <router-link
        v-if="!inThread"
        class="icon"
        title="View replies"
        :to="{ name: 'post', params: { postId: post.id }}"
      >
        <icon-comment></icon-comment>
        <span>{{ post.replies_count }}</span>
      </router-link>
      <button
        v-else-if="inThread && canReply()"
        class="icon"
        title="Reply"
        @click="toggleReplyForm()"
      >
        <icon-comment></icon-comment>
        <span>{{ post.replies_count }}</span>
      </button>
      <span v-else class="icon">
        <icon-comment></icon-comment>
        <span>{{ post.replies_count }}</span>
      </span>
      <button
        v-if="canRepost()"
        class="icon"
        :class="{ highlighted: post.reblogged }"
        :disabled="isProcessingRepost"
        :title="post.reblogged ? 'Delete repost' : 'Repost'"
        @click="toggleRepost()"
      >
        <icon-repost></icon-repost>
        <span>{{ post.reblogs_count }}</span>
      </button>
      <span v-else class="icon">
        <icon-repost></icon-repost>
        <span>{{ post.reblogs_count }}</span>
      </span>
      <button
        v-if="canLike()"
        class="icon"
        :class="{ highlighted: post.favourited }"
        :disabled="isProcessingLike"
        :title="post.favourited ? 'Unlike': 'Like'"
        @click="toggleLike()"
      >
        <icon-like></icon-like>
        <span>{{ post.favourites_count }}</span>
      </button>
      <span v-else class="icon">
        <icon-like></icon-like>
        <span>{{ post.favourites_count }}</span>
      </span>
      <a
        v-if="getIpfsUrl()"
        class="icon"
        title="Saved to IPFS"
        :href="getIpfsUrl() || ''"
        target="_blank"
        rel="noreferrer"
      >
        <icon-ipfs></icon-ipfs>
      </a>
      <div
        class="dropdown-menu-wrapper"
        v-click-away="hideMenu"
      >
        <button class="icon" title="More" @click="toggleMenu()">
          <icon-more></icon-more>
        </button>
        <menu v-if="menuVisible" class="dropdown-menu">
          <li>
            <a
              :href="post.uri"
              class="icon"
              title="Copy link to post"
              @click.prevent="hideMenu(); copyPostUri()"
            >
              <icon-link></icon-link>
              <span>Copy link to post</span>
            </a>
          </li>
          <li v-if="canPin()">
            <button
              class="icon"
              title="Add to featured"
              @click="hideMenu(); onPin()"
            >
              <icon-pin></icon-pin>
              <span>Add to featured</span>
            </button>
          </li>
          <li v-if="canUnpin()">
            <button
              class="icon"
              title="Remove from featured"
              @click="hideMenu(); onUnpin()"
            >
              <icon-unpin></icon-unpin>
              <span>Remove from featured</span>
            </button>
          </li>
          <li v-if="canSaveToIpfs()">
            <button
              class="icon"
              title="Save to IPFS"
              @click="hideMenu(); saveToIpfs()"
            >
              <icon-ipfs></icon-ipfs>
              <span>Save to IPFS</span>
            </button>
          </li>
          <li v-if="canRepostWithComment()">
            <button
              class="icon"
              title="Repost with comment"
              @click="hideMenu(); onRepostWithComment()"
            >
              <icon-quote></icon-quote>
              <span>Repost with comment</span>
            </button>
          </li>
          <li v-if="canEditPost()">
            <button
              class="icon"
              title="Edit post"
              @click="hideMenu(); onEditPost()"
            >
              <icon-edit></icon-edit>
              <span>Edit post</span>
            </button>
          </li>
          <li v-if="canDeletePost()">
            <button
              class="icon"
              title="Delete post"
              @click="hideMenu(); onDeletePost()"
            >
              <icon-trash></icon-trash>
              <span>Delete post</span>
            </button>
          </li>
          <li v-if="canMute()">
            <button
              class="icon"
              title="Mute author"
              @click="onMute()"
            >
              <icon-mute></icon-mute>
              <span>Mute author</span>
            </button>
          </li>
          <li v-if="canUnmute()">
            <button
              class="icon"
              title="Unmute author"
              @click="onUnmute()"
            >
              <icon-unmute></icon-unmute>
              <span>Unmute author</span>
            </button>
          </li>
        </menu>
      </div>
      <div class="crypto-widget">
        <crypto-address
          v-if="selectedPaymentOption?.address"
          :address="selectedPaymentOption.address"
        ></crypto-address>
        <universal-link
          v-if="selectedPaymentOption?.subscription"
          :to="selectedPaymentOption.subscription"
          title="Become a subscriber"
          class="subscribe-btn"
        >
          <template #link-content>subscribe</template>
        </universal-link>
        <button
          v-for="option in getPaymentOptions()"
          :key="option.code"
          class="icon"
          :title="'Send ' + option.name"
          @click="togglePaymentAddress(option)"
        >
          <crypto-icon :code="option.code"></crypto-icon>
        </button>
      </div>
    </div>
    <post-editor
      v-if="replyFormVisible"
      class="comment-form"
      :post="null"
      :in-reply-to="post"
      :repost-of="null"
      @post-saved="onReplyCreated"
      @post-editor-closed="replyFormVisible = false"
    >
    </post-editor>
    <post-editor
      v-if="repostFormVisible"
      class="comment-form"
      :post="null"
      :in-reply-to="null"
      :repost-of="post"
      @post-saved="onRepostCreated"
      @post-editor-closed="repostFormVisible = false"
    >
    </post-editor>
  </div>
  <post-editor
    v-else
    class="post-edit-form"
    :post="post"
    :in-reply-to="null"
    :repost-of="null"
    @post-saved="onPostUpdated"
    @post-editor-closed="editorVisible = false"
  >
  </post-editor>
</template>

<script setup lang="ts">
/* eslint-disable vue/no-mutating-props */
import { ref } from "vue"
import { $, $computed, $ref } from "vue/macros"
import { useRouter, RouteLocationRaw } from "vue-router"

import {
  getPostSource,
  deletePost,
  favourite,
  unfavourite,
  createRepost,
  deleteRepost,
  pinPost,
  unpinPost,
  makePermanent,
  Mention,
  Post,
  Visibility,
  VISIBILITY_MAP,
} from "@/api/posts"
import { mute, unmute } from "@/api/relationships"
import {
  Permissions,
  Profile,
  ProfileWrapper,
} from "@/api/users"
import IconIpfs from "@/assets/extra-icons/ipfs.svg?component"
import IconEdit from "@/assets/feather/edit-3.svg?component"
import IconLink from "@/assets/feather/link.svg?component"
import IconMore from "@/assets/feather/more-horizontal.svg?component"
import IconRepost from "@/assets/feather/repeat.svg?component"
import IconTrash from "@/assets/feather/trash.svg?component"
import IconMute from "@/assets/feather/volume-x.svg?component"
import IconUnmute from "@/assets/feather/volume-2.svg?component"
import IconComment from "@/assets/forkawesome/comment-o.svg?component"
import IconLike from "@/assets/forkawesome/thumbs-o-up.svg?component"
import IconLeftUp from "@/assets/tabler/corner-left-up.svg?component"
import IconPin from "@/assets/tabler/pin.svg?component"
import IconUnpin from "@/assets/tabler/pinned-off.svg?component"
import IconQuote from "@/assets/tabler/quote.svg?component"
import Avatar from "@/components/Avatar.vue"
import CryptoAddress from "@/components/CryptoAddress.vue"
import CryptoIcon from "@/components/CryptoIcon.vue"
import PostAttachment from "@/components/PostAttachment.vue"
import PostContent from "@/components/PostContent.vue"
import PostEditor from "@/components/PostEditor.vue"
import PostPreview from "@/components/PostPreview.vue"
import ProfileDisplayName from "@/components/ProfileDisplayName.vue"
import VisibilityIcon from "@/components/VisibilityIcon.vue"
import UniversalLink from "@/components/UniversalLink.vue"
import { useActorHandle } from "@/composables/handle"
import { useInstanceInfo } from "@/composables/instance"
import { useSubscribe } from "@/composables/subscribe"
import { useCurrentUser } from "@/composables/user"
import { getCurrencyByLabel, Currency, ETHEREUM, MONERO } from "@/utils/cryptocurrencies"
import { formatDateTime, humanizeDate } from "@/utils/dates"

interface PaymentOption {
  code: string;
  name: string;
  address: string | null;
  subscription: string | RouteLocationRaw | null;
}

const router = useRouter()
const { getActorHandle, getActorLocation } = useActorHandle()
const { currentUser, ensureAuthToken } = $(useCurrentUser())
const { instance } = $(useInstanceInfo())
const { getSubscriptionLink } = useSubscribe()

const props = defineProps<{
  post: Post,
  highlighted: boolean,
  inThread: boolean,
}>()

/* eslint-disable-next-line func-call-spacing */
const emit = defineEmits<{
  (event: "highlight", postId: string | null): void,
  (event: "navigate-to", postId: string): void,
  (event: "comment-created", post: Post): void,
  (event: "post-deleted"): void,
}>()

const replyFormVisible = ref(false)
const repostFormVisible = ref(false)
const editorVisible = ref(false)
const isProcessingRepost = ref(false)
const isProcessingLike = ref(false)
let menuVisible = $ref(false)
let selectedPaymentOption = $ref<PaymentOption | null>(null)

const author = $computed(() => new ProfileWrapper(props.post.account))

function getProfileLocation(profile: Mention | Profile): string | RouteLocationRaw {
  if (currentUser === null) {
    // Viewing as guest
    return profile.url
  }
  return getActorLocation("profile", profile)
}

function getPostLocation(post: Post): string | RouteLocationRaw {
  if (currentUser === null) {
    // Viewing as guest
    return post.uri
  }
  return { name: "post", params: { postId: post.id } }
}

function highlight(postId: string | null) {
  emit("highlight", postId)
}

function scrollTo(postId: string) {
  emit("navigate-to", postId)
}

function getVisibilityDisplay(): string {
  return VISIBILITY_MAP[props.post.visibility]
}

function getReplyMentions(): Mention[] {
  if (props.post.in_reply_to_id === null) {
    return []
  }
  if (
    props.post.in_reply_to_account_id === props.post.account.id &&
    props.post.mentions.every((mention) => mention.id !== props.post.account.id)
  ) {
    // Self-reply
    return [props.post.account, ...props.post.mentions]
  } else {
    return props.post.mentions
  }
}

function canReply(): boolean {
  if (currentUser === null) {
    return false
  }
  return currentUser.role.permissions.includes(Permissions.CreatePost)
}

function toggleReplyForm() {
  replyFormVisible.value = !replyFormVisible.value
}

function onReplyCreated(post: Post) {
  replyFormVisible.value = false
  repostFormVisible.value = false
  emit("comment-created", post)
}

function canRepostWithComment(): boolean {
  return props.inThread && canReply()
}

async function onRepostWithComment() {
  replyFormVisible.value = false
  repostFormVisible.value = true
}

function onRepostCreated() {
  repostFormVisible.value = false
  router.push({ name: "home" })
}

function canRepost(): boolean {
  if (currentUser === null) {
    return false
  }
  return (
    props.post.visibility === "public" &&
    currentUser.role.permissions.includes(Permissions.CreatePost)
  )
}

async function toggleRepost() {
  if (!currentUser) {
    return
  }
  const authToken = ensureAuthToken()
  isProcessingRepost.value = true
  let updatedPost
  try {
    if (props.post.reblogged) {
      updatedPost = await deleteRepost(authToken, props.post.id)
    } else {
      updatedPost = await createRepost(authToken, props.post.id)
    }
  } catch (error) {
    isProcessingRepost.value = false
    console.log(error)
    return
  }
  isProcessingRepost.value = false
  props.post.reblogs_count = updatedPost.reblogs_count
  props.post.reblogged = updatedPost.reblogged
}

function canLike(): boolean {
  return currentUser !== null
}

async function toggleLike() {
  if (!currentUser) {
    return
  }
  const authToken = ensureAuthToken()
  isProcessingLike.value = true
  let updatedPost
  try {
    if (props.post.favourited) {
      updatedPost = await unfavourite(authToken, props.post.id)
    } else {
      updatedPost = await favourite(authToken, props.post.id)
    }
  } catch (error) {
    isProcessingLike.value = false
    console.log(error)
    return
  }
  isProcessingLike.value = false
  props.post.favourites_count = updatedPost.favourites_count
  props.post.favourited = updatedPost.favourited
}

function toggleMenu() {
  menuVisible = !menuVisible
}

function hideMenu() {
  menuVisible = false
}

function copyPostUri(): void {
  navigator.clipboard.writeText(props.post.uri)
}

function canPin(): boolean {
  return (
    props.post.account.id === currentUser?.id &&
    props.post.visibility === Visibility.Public &&
    !props.post.pinned
  )
}

async function onPin() {
  const authToken = ensureAuthToken()
  const { pinned } = await pinPost(authToken, props.post.id)
  props.post.pinned = pinned
}

function canUnpin(): boolean {
  return (
    props.post.account.id === currentUser?.id &&
    props.post.visibility === Visibility.Public &&
    props.post.pinned
  )
}

async function onUnpin() {
  const authToken = ensureAuthToken()
  const { pinned } = await unpinPost(authToken, props.post.id)
  props.post.pinned = pinned
}

function getIpfsUrl(): string | null {
  const gatewayUrl = instance?.ipfs_gateway_url
  if (
    !gatewayUrl ||
    props.post.ipfs_cid === null
  ) {
    return null
  }
  return `${gatewayUrl}/ipfs/${props.post.ipfs_cid}`
}

function canSaveToIpfs(): boolean {
  return (
    Boolean(instance?.ipfs_gateway_url) &&
    props.post.account.id === currentUser?.id &&
    props.post.visibility === "public" &&
    props.post.ipfs_cid === null
  )
}

async function saveToIpfs() {
  if (!currentUser || !instance || !instance.ipfs_gateway_url) {
    return
  }
  const authToken = ensureAuthToken()
  const { ipfs_cid } = await makePermanent(authToken, props.post.id)
  props.post.ipfs_cid = ipfs_cid
}

function canEditPost(): boolean {
  return props.post.account.id === currentUser?.id
}

async function onEditPost() {
  const authToken = ensureAuthToken()
  const contentSource = await getPostSource(authToken, props.post.id)
  props.post.contentSource = contentSource
  editorVisible.value = true
}

function onPostUpdated(updatedPost: Post) {
  Object.assign(props.post, updatedPost)
  editorVisible.value = false
}

function canDeletePost(): boolean {
  return props.post.account.id === currentUser?.id
}

async function onDeletePost() {
  if (confirm("Are you sure you want to delete this post?")) {
    const authToken = ensureAuthToken()
    await deletePost(authToken, props.post.id)
    emit("post-deleted")
  }
}

function canMute(): boolean {
  return (
    props.post.account.id !== currentUser?.id &&
    // Don't show menu item if post.relationship property is not set
    !!props.post.relationship &&
    !props.post.relationship.muting
  )
}

async function onMute() {
  const authToken = ensureAuthToken()
  props.post.relationship = await mute(authToken, props.post.account.id)
}

function canUnmute(): boolean {
  return (
    props.post.account.id !== currentUser?.id &&
    // Don't show menu item if post.relationship property is not set
    !!props.post.relationship &&
    props.post.relationship.muting
  )
}

async function onUnmute() {
  const authToken = ensureAuthToken()
  props.post.relationship = await unmute(authToken, props.post.account.id)
}

function getPaymentOptions(): PaymentOption[] {
  const options: PaymentOption[] = []
  for (const field of props.post.account.fields) {
    const currency = getCurrencyByLabel(field.name)
    if (!currency) {
      continue
    }
    const address = field.value.trim()
    options.push({
      code: currency.code,
      name: currency.name,
      address,
      subscription: null,
    })
  }
  const subscriptionLink = getSubscriptionLink(props.post.account)
  if (subscriptionLink) {
    // TODO: use CAIP-2 ID -> symbol mapping
    let currency: Currency
    if (subscriptionLink.type === "ethereum") {
      currency = ETHEREUM
    } else if (subscriptionLink.type === "monero") {
      currency = MONERO
    } else {
      throw new Error("invalid subscription type")
    }
    const option = options.find((option) => {
      return option.code === currency.code
    })
    if (option) {
      // Add subscription link if option already exists
      option.subscription = subscriptionLink.location
    } else {
      options.push({
        code: currency.code,
        name: currency.name,
        address: null,
        subscription: subscriptionLink.location,
      })
    }
  }
  return options
}

function togglePaymentAddress(option: PaymentOption) {
  selectedPaymentOption = selectedPaymentOption?.code === option.code ? null : option
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/theme";
@import "../styles/mixins";

.post {
  background-color: var(--block-background-color);
  border-radius: $block-border-radius;
  text-align: left;

  &.highlighted {
    outline: 1px solid var(--highlight-color);
  }
}

.post-header {
  @include post-icon;

  align-items: center;
  color: var(--secondary-text-color);
  display: flex;
  flex-direction: row;
  gap: calc($block-inner-padding / 2);
  padding: $block-inner-padding $block-inner-padding 0;

  .floating-avatar {
    @include floating-avatar;

    @media screen and (min-width: $screen-breakpoint-medium + 1) {
      margin-right: calc(0px - $block-inner-padding / 2);
    }
  }

  .display-name-link {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;

    .display-name {
      color: var(--text-color);
      font-weight: bold;
    }
  }

  .actor-address {
    flex-basis: 25%;
    flex-grow: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    user-select: all;
  }

  .timestamp {
    color: var(--secondary-text-color);
    text-align: right;
    white-space: nowrap;

    &:hover {
      color: var(--secondary-text-hover-color);
    }
  }
}

.post-subheader {
  color: var(--secondary-text-color);
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: $whitespace;
  padding: calc($block-inner-padding / 4) $block-inner-padding 0;

  a {
    @include block-link;

    overflow: hidden;
    word-wrap: break-word;
  }
}

.post-content {
  margin: $block-inner-padding 0;
  padding: 0 $block-inner-padding;
}

.post-attachments {
  margin: $block-inner-padding 0;
  padding: 0 $block-inner-padding;
}

.post-quote {
  color: inherit;
  display: block;
  margin: 0 $block-inner-padding $block-inner-padding;

  &:hover {
    color: inherit;
  }
}

.post-footer {
  @include post-icon;

  align-items: center;
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: calc($block-inner-padding / 2);
  padding: 0 $block-inner-padding $block-inner-padding;
}

@keyframes spin {
  100% {
    transform: rotate(360deg);
  }
}

.dropdown-menu-wrapper {
  @include block-dropdown-menu;

  a.icon,
  button.icon {
    gap: calc($block-inner-padding / 2);
  }
}

.crypto-widget {
  display: flex;
  flex-grow: 1;
  gap: calc($block-inner-padding / 2);
  justify-content: right;

  .crypto-address {
    max-width: 200px;
    width: 100%;
  }

  .subscribe-btn {
    background-color: var(--widget-background-color);
    border-radius: calc($icon-size / 2);
    font-family: monospace;
    font-size: 12px;
    line-height: $icon-size;
    padding: 0 7px;
  }

  .icon svg {
    /* Make filled icons lighter to match line icons */
    opacity: 0.75;
  }
}

.comment-form {
  border-top: 1px solid var(--separator-color);
}
</style>
