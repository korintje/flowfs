<template>
  <form class="post-form">
    <router-link
      v-if="author"
      class="floating-avatar"
      :to="getActorLocation('profile', author)"
    >
      <avatar :profile="author"></avatar>
    </router-link>
    <div class="textarea-group">
      <textarea
        id="content"
        ref="contentInputElement"
        v-show="preview === null"
        :value="content"
        @input="onContentInput"
        rows="1"
        required
        :placeholder="inReplyTo ? 'Your reply' : (repostOf ? 'Your comment' : 'What\'s on your mind?')"
        @paste="onPaste($event)"
        @keyup.ctrl.enter="onCtrlEnter()"
      ></textarea>
      <div
        class="mention-suggestions"
        v-if="mentionSuggestionList.length > 0 && preview === null"
      >
        <button
          v-for="profile in mentionSuggestionList"
          :key="profile.id"
          @click.prevent="autocompleteMention(profile)"
        >
          @{{ profile.acct }}
        </button>
      </div>
      <post-content
        v-if="preview"
        class="preview"
        :post="preview"
        @click.prevent=""
      ></post-content>
      <div v-if="attachments.length > 0" class="attachments">
        <post-editor-attachment
          v-for="(attachment, index) in attachments"
          :attachment="attachment"
          :key="attachment.id"
          @attachment-updated="onAttachmentUpdated(index, $event)"
          @attachment-removed="onAttachmentRemoved(index)"
        ></post-editor-attachment>
      </div>
      <div class="toolbar">
        <button
          type="button"
          class="icon"
          title="Attach file"
          :disabled="!canAttachFile()"
          @click="selectAttachment()"
        >
          <icon-attach v-if="!isAttachmentLoading"></icon-attach>
          <loader v-else></loader>
          <input
            type="file"
            ref="attachmentUploadInput"
            :accept="getAcceptedMediaTypes()"
            style="display: none;"
            @change="onAttachmentUpload($event)"
          >
        </button>
        <div
          class="dropdown-menu-wrapper"
          v-click-away="hideVisibilityMenu"
        >
          <button
            v-if="canChangeVisibility()"
            type="button"
            class="icon"
            title="Change visibility"
            @click="toggleVisibilityMenu()"
          >
            <visibility-icon :visibility="visibility"></visibility-icon>
          </button>
          <span v-else class="icon">
            <visibility-icon :visibility="visibility"></visibility-icon>
          </span>
          <menu v-if="visibilityMenuVisible" class="dropdown-menu">
            <li v-for="[value, display] in visibilityMap" :key="value">
              <button
                class="icon"
                :title="display"
                @click="hideVisibilityMenu(); visibility = value as Visibility"
              >
                <visibility-icon :visibility="value"></visibility-icon>
                <span>{{ display }}</span>
              </button>
            </li>
          </menu>
        </div>
        <button
          type="button"
          class="icon"
          :class="{ warning: isSensitive }"
          :title="isSensitive ? 'Remove content warning' : 'Add content warning'"
          @click="isSensitive = !isSensitive"
        >
          <icon-alert></icon-alert>
        </button>
        <div
          class="dropdown-menu-wrapper"
          v-click-away="hideEmojiPicker"
        >
          <button
            type="button"
            class="icon"
            title="Insert emoji"
            @click="toggleEmojiPicker"
          >
            <icon-smile></icon-smile>
          </button>
          <emoji-picker
            v-if="emojiPickerVisible"
            @emoji-picked="insertEmoji($event)"
          ></emoji-picker>
        </div>
        <div class="toolbar-space"></div>
        <div
          v-if="isCharacterCounterVisible()"
          class="character-counter"
          title="Characters left"
        >
          {{ getCharacterCount() }}
        </div>
        <button
          v-if="canPreview()"
          type="button"
          class="icon btn-small"
          title="Toggle preview"
          @click="togglePreview()"
        >
          <icon-show v-if="preview === null"></icon-show>
          <icon-hide v-else></icon-hide>
        </button>
        <button
          v-if="isEditorEmbedded"
          class="icon btn-small"
          @click.prevent="cancel()"
        >
          Cancel
        </button>
        <button
          type="submit"
          v-if="isEditorEmbedded"
          class="icon btn-small"
          :disabled="!canPublish()"
          @click.prevent="publish()"
        >
          <template v-if="repostOf">Repost</template>
          <template v-else>Publish</template>
        </button>
      </div>
    </div>
    <div v-if="!isEditorEmbedded" class="submit-btn-wrapper">
      <div class="error-message" v-if="errorMessage">{{ errorMessage }}</div>
      <button
        v-if="post"
        class="btn secondary"
        @click.prevent="cancel()"
      >
        Cancel
      </button>
      <button
        class="btn"
        type="submit"
        :disabled="!canPublish()"
        @click.prevent="publish()"
      >
        <template v-if="post">Update</template>
        <template v-else>Publish</template>
      </button>
    </div>
  </form>
</template>

<script setup lang="ts">
/* eslint-disable vue/no-setup-props-destructure */
import { computed, nextTick, onMounted, ref } from "vue"
import { $ref } from "vue/macros"

import { getEmojiShortcode } from "@/api/emojis"
import {
  createPost,
  previewPost,
  updatePost,
  uploadAttachment,
  Attachment,
  Mention,
  Post,
  Visibility,
  VISIBILITY_MAP,
} from "@/api/posts"
import { searchProfilesByAcct } from "@/api/search"
import { Profile, User } from "@/api/users"
import IconAlert from "@/assets/feather/alert-triangle.svg?component"
import IconShow from "@/assets/feather/eye.svg?component"
import IconHide from "@/assets/feather/eye-off.svg?component"
import IconAttach from "@/assets/feather/paperclip.svg?component"
import IconSmile from "@/assets/feather/smile.svg?component"
import Avatar from "@/components/Avatar.vue"
import EmojiPicker from "@/components/EmojiPicker.vue"
import Loader from "@/components/Loader.vue"
import PostContent from "@/components/PostContent.vue"
import PostEditorAttachment from "@/components/PostEditorAttachment.vue"
import VisibilityIcon from "@/components/VisibilityIcon.vue"
import { useClientConfig } from "@/composables/client-config"
import { useActorHandle } from "@/composables/handle"
import { useInstanceInfo } from "@/composables/instance"
import { useCurrentUser } from "@/composables/user"
import { resizeTextArea, setupAutoResize } from "@/utils/autoresize"
import { debounce } from "@/utils/debounce"
import { fileToDataUrl, dataUrlToBase64 } from "@/utils/upload"

const visibilityMap = Object.entries(VISIBILITY_MAP)
const POST_CONTENT_STORAGE_KEY = "post_content"

const { ctrlEnterEnabled } = useClientConfig()
const { getActorHandle, getActorLocation } = useActorHandle()
const { currentUser, ensureAuthToken } = useCurrentUser()
const { instance } = useInstanceInfo()

const props = defineProps<{
  post: Post | null,
  inReplyTo: Post | null,
  repostOf: Post | null,
}>()

/* eslint-disable-next-line func-call-spacing */
const emit = defineEmits<{
  (event: "post-saved", post: Post): void,
  (event: "post-editor-closed"): void,
}>()

const contentInputElement = ref<HTMLTextAreaElement | null>(null)
const attachmentUploadInput = ref<HTMLInputElement | null>(null)

const content = ref(loadLocalDraft())
let attachments = $ref<Attachment[]>([])
let visibility = $ref(Visibility.Public)
let isSensitive = $ref(false)

const mentionSuggestionList = ref<Profile[]>([])
const mentionPosition = ref<[number, number] | null>(null)
let visibilityMenuVisible = $ref(false)
const emojiPickerVisible = ref(false)
let preview = $ref<Post | null>(null)
let isLoading = $ref(false)
let isAttachmentLoading = $ref(false)
let errorMessage = $ref<string | null>(null)

const author = computed<User | null>(() => currentUser.value)
const isEditorEmbedded = computed(() => {
  return props.inReplyTo !== null || props.repostOf !== null
})

if (props.post) {
  content.value = props.post.contentSource || ""
  attachments = [...props.post.media_attachments]
  visibility = props.post.visibility
  isSensitive = props.post.sensitive
}

if (props.inReplyTo && content.value.length === 0) {
  const mentions: Mention[] = [
    props.inReplyTo.account,
    ...props.inReplyTo.mentions,
  ]
  content.value = mentions
    .filter(mention => mention.id !== currentUser.value?.id)
    .map(mention => getActorHandle(mention))
    // Remove non-webfinger handles
    .filter(mention => mention.startsWith("@"))
    // Remove duplicates
    .filter((mention, index, mentions) => mentions.indexOf(mention) === index)
    .join(" ")
}

if (props.inReplyTo && props.inReplyTo.visibility !== Visibility.Public) {
  visibility = Visibility.Direct
}

onMounted(() => {
  if (contentInputElement.value) {
    setupAutoResize(contentInputElement.value)
    resizeTextArea(contentInputElement.value)
  }
})

function getLocalDraftKey(): string {
  const postId = props.inReplyTo?.id || "new"
  return `${POST_CONTENT_STORAGE_KEY}_${postId}`
}

function loadLocalDraft(): string {
  return localStorage.getItem(getLocalDraftKey()) || ""
}

function saveLocalDraft() {
  localStorage.setItem(getLocalDraftKey(), content.value)
}

function removeLocalDraft() {
  localStorage.removeItem(getLocalDraftKey())
}

async function suggestMentions() {
  if (contentInputElement.value === null) {
    return
  }
  const currentPosition = contentInputElement.value.selectionStart
  const contentBefore = content.value.substring(0, currentPosition)
  // "d" flag requires FF 88+
  // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/hasIndices
  const mentionRegexp = /(^|\s)@(?<name>\S+)$/d
  const match = mentionRegexp.exec(contentBefore)
  const mentionText = match?.groups?.name
  if (mentionText && mentionText.length >= 2) {
    const indices = (match as any).indices.groups.name
    const results = await searchProfilesByAcct(
      ensureAuthToken(),
      mentionText,
      false,
      4,
    )
    if (results.length !== 1 || results[0].acct !== mentionText) {
      mentionSuggestionList.value = results
      mentionPosition.value = indices
      return
    }
  }
  mentionSuggestionList.value = []
}

const suggestMentionsDebounced = debounce(suggestMentions, 500)

async function insertText(start: number, stop: number, text: string) {
  if (contentInputElement.value === null) {
    throw new Error("editor doesn't exist")
  }
  content.value =
    content.value.substring(0, start) +
    text +
    content.value.substring(stop)
  await nextTick()
  const newPosition = start + text.length
  contentInputElement.value.focus()
  contentInputElement.value.selectionEnd = newPosition
}

async function autocompleteMention(profile: Profile) {
  if (contentInputElement.value !== null && mentionPosition.value !== null) {
    const [start, stop] = mentionPosition.value
    // Suggested profile is expected to have webfinger address
    await insertText(start, stop, profile.acct)
    mentionSuggestionList.value = []
  }
}

function onContentInput(event: Event) {
  content.value = (event.target as HTMLTextAreaElement).value
  saveLocalDraft()
  suggestMentionsDebounced()
}

async function onPaste(event: ClipboardEvent) {
  const files = event.clipboardData?.files || []
  if (files.length > 0) {
    event.preventDefault()
    // NOTE: files property gets emptied after event propagation
    await addAttachment(files[0])
  }
}

function canAttachFile(): boolean {
  if (!instance.value) {
    return false
  }
  return (
    attachments.length < instance.value.configuration.statuses.max_media_attachments &&
    !isAttachmentLoading
  )
}

function getAcceptedMediaTypes(): string {
  if (!instance.value) {
    return ""
  }
  const types = [...instance.value.configuration.media_attachments.supported_mime_types]
  if (types.includes("video/x-m4v")) {
    // Some OSes don't associate .m4v files with video/x-m4v media type
    types.push(".m4v")
  }
  return types.join(",")
}

function selectAttachment() {
  if (attachmentUploadInput.value) {
    attachmentUploadInput.value.click()
  }
}

async function onAttachmentUpload(event: Event) {
  const files = (event.target as HTMLInputElement).files
  if (!files) {
    return
  }
  await addAttachment(files[0])
}

async function addAttachment(file: File) {
  isAttachmentLoading = true
  const imageDataUrl = await fileToDataUrl(file)
  const imageData = dataUrlToBase64(imageDataUrl)
  let attachment
  try {
    attachment = await uploadAttachment(
      ensureAuthToken(),
      imageData.data,
      imageData.mediaType,
    )
  } catch (error: any) {
    isAttachmentLoading = false
    alert(error.message)
    return
  }
  attachments.push(attachment)
  isAttachmentLoading = false
}

function onAttachmentUpdated(index: number, attachment: Attachment) {
  Object.assign(attachments[index], attachment)
}

function onAttachmentRemoved(index: number) {
  attachments.splice(index, 1)
}

function canChangeVisibility(): boolean {
  return (
    props.post === null && (
      props.inReplyTo === null ||
      props.inReplyTo.visibility === Visibility.Public
    )
  )
}

function toggleVisibilityMenu() {
  visibilityMenuVisible = !visibilityMenuVisible
}

function hideVisibilityMenu() {
  visibilityMenuVisible = false
}

function toggleEmojiPicker() {
  emojiPickerVisible.value = !emojiPickerVisible.value
}

function hideEmojiPicker() {
  emojiPickerVisible.value = false
}

async function insertEmoji(name: string) {
  if (contentInputElement.value === null) {
    throw new Error("editor doesn't exist")
  }
  const position = contentInputElement.value.selectionStart
  // Add whitespace before and after shortcode
  let text = `${getEmojiShortcode(name)} `
  if (position !== 0 && !/\s/.test(content.value.charAt(position - 1))) {
    text = " " + text
  }
  await insertText(position, position, text)
  hideEmojiPicker()
}

function getCharacterCount(): number {
  if (!instance.value) {
    return 0
  }
  return (instance.value.configuration.statuses.max_characters - content.value.length)
}

function isCharacterCounterVisible(): boolean {
  return getCharacterCount() < 100
}

function canPreview(): boolean {
  return content.value.length > 0
}

async function togglePreview() {
  if (preview === null) {
    preview = await previewPost(ensureAuthToken(), content.value)
  } else {
    preview = null
  }
}

function cancel() {
  emit("post-editor-closed")
}

function canPublish(): boolean {
  return getCharacterCount() >= 0 && !isLoading && !isAttachmentLoading
}

function getObjectLink(post: Post): string {
  let markup = `\n\n RE: [[${post.uri}]]`
  if (!post.account.acct.includes("://")) {
    // Insert mention only if acct is a webfinger address
    markup += `\n\n@${post.account.acct}`
  }
  return markup
}

async function publish() {
  const postData = {
    content: content.value,
    inReplyToId: props.inReplyTo ? props.inReplyTo.id : null,
    visibility: visibility,
    isSensitive: isSensitive,
    attachments: attachments,
  }
  if (props.repostOf) {
    // Append object link markup
    postData.content = postData.content + getObjectLink(props.repostOf)
  }
  isLoading = true
  let post
  try {
    if (props.post !== null) {
      post = await updatePost(
        ensureAuthToken(),
        props.post.id,
        content.value,
        attachments,
        isSensitive,
      )
    } else {
      post = await createPost(
        ensureAuthToken(),
        postData,
      )
    }
  } catch (error: any) {
    errorMessage = error.message
    isLoading = false
    if (isEditorEmbedded.value === true) {
      // Show alert if there's no errorbox
      alert(errorMessage)
    }
    return
  }
  // Refresh editor
  errorMessage = null
  isLoading = false
  removeLocalDraft()
  content.value = ""
  isSensitive = false
  attachments = []
  preview = null
  if (contentInputElement.value) {
    await nextTick()
    resizeTextArea(contentInputElement.value)
  }
  emit("post-saved", post)
}

async function onCtrlEnter() {
  if (ctrlEnterEnabled.value) {
    await publish()
  }
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";
@import "../styles/theme";

$line-height: 1.5;

.post-form {
  position: relative;

  .floating-avatar {
    @include floating-avatar;

    left: $block-inner-padding;
    margin-top: calc($line-height * 1em / 2);
    position: absolute;
    top: $block-inner-padding;

    @media screen and (max-width: $screen-breakpoint-medium) {
      display: none;
    }
  }
}

.textarea-group {
  background-color: var(--block-background-color);
  border-radius: $block-border-radius;
}

#content {
  border-radius: $block-border-radius $block-border-radius 0 0;
  height: 100px;
  line-height: $line-height;
  padding: $block-inner-padding;
  width: 100%;
}

.mention-suggestions {
  display: flex;
  flex-wrap: wrap;
  gap: $input-padding;
  padding: calc($block-inner-padding / 1.5) $block-inner-padding;

  button {
    background-color: var(--background-color);
    border-radius: $btn-border-radius;
    overflow: hidden;
    padding: calc($input-padding / 2);
    text-overflow: ellipsis;
  }
}

.preview {
  padding: $block-inner-padding;
}

.attachments {
  padding: calc($block-inner-padding / 1.5) $block-inner-padding;
}

.toolbar {
  @include post-icon;

  align-items: center;
  border-radius: 0 0 $block-border-radius $block-border-radius;
  border-top: 1px solid var(--separator-color);
  color: var(--secondary-text-color);
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: calc($block-inner-padding / 2);
  justify-content: right;
  padding: calc($block-inner-padding / 1.5) $block-inner-padding;

  .toolbar-space {
    flex-grow: 1;
  }

  .loader {
    --loader-size: #{$icon-size};
    --loader-width: 2px;
  }

  .icon.warning {
    color: $warning-color;

    svg {
      stroke: $warning-color;
    }
  }

  .btn-small {
    font-weight: bold;
    margin-left: calc($block-inner-padding / 2);

    &[type="submit"] {
      color: var(--link-color);

      &:hover {
        color: var(--link-hover-color);
      }
    }

    &[disabled] {
      color: var(--btn-disabled-text-color);
      cursor: initial;
    }

    @media screen and (max-width: $screen-breakpoint-x-small) {
      margin-left: 0;
    }
  }
}

.dropdown-menu-wrapper {
  @include block-dropdown-menu;

  button.icon {
    gap: calc($block-inner-padding / 2);
  }
}

.submit-btn-wrapper {
  align-items: center;
  display: flex;
  flex-direction: row;
  gap: $block-outer-padding;
  justify-content: flex-end;
  margin-top: calc($block-inner-padding / 1.5);

  .error-message {
    color: $error-color;
    margin-right: $block-inner-padding;
  }
}
</style>
