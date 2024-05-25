<template>
  <div class="post-content" ref="postContentRef" v-html="getContent()"></div>
</template>

<script setup lang="ts">
import { onMounted } from "vue"
import { $, $ref } from "vue/macros"
import { useRouter } from "vue-router"

import { replaceShortcodes } from "@/api/emojis"
import { Post } from "@/api/posts"
import { useActorHandle } from "@/composables/handle"
import { useCurrentUser } from "@/composables/user"
import { addGreentext } from "@/utils/greentext"

const router = useRouter()
const { getActorLocation } = useActorHandle()
const { currentUser } = $(useCurrentUser())

const props = defineProps<{
  post: Post,
}>()

const postContentRef = $ref<HTMLElement | null>(null)

onMounted(() => {
  if (currentUser !== null) {
    configureInlineLinks()
  }
})

function configureInlineLinks() {
  if (postContentRef === null) {
    return
  }
  const mentions = postContentRef.getElementsByClassName("mention")
  for (const mentionElement of Array.from(mentions)) {
    if (!(mentionElement instanceof HTMLElement)) {
      continue
    }
    const mention = props.post.mentions
      .find((mention) => mentionElement.getAttribute("href") === mention.url)
    if (mention) {
      mentionElement.addEventListener("click", (event: Event) => {
        event.preventDefault()
        router.push(getActorLocation("profile", mention))
      })
      mentionElement.dataset.internalLink = "true"
    }
  }
  const hashtags = postContentRef.querySelectorAll('.hashtag, [rel~="tag"]')
  for (const hashtagElement of Array.from(hashtags)) {
    if (!(hashtagElement instanceof HTMLElement)) {
      continue
    }
    const hashtag = props.post.tags
      .find((tag) => {
        const innerText = (hashtagElement as HTMLElement).innerText
        return innerText.toLowerCase() === `#${tag.name}`
      })
    if (hashtag) {
      hashtagElement.addEventListener("click", (event: Event) => {
        event.preventDefault()
        router.push({ name: "tag", params: { tagName: hashtag.name } })
      })
      hashtagElement.dataset.internalLink = "true"
    }
  }
  for (const linkedPost of props.post.links) {
    const links = postContentRef.querySelectorAll("a")
    for (const linkElement of Array.from(links)) {
      if (linkedPost.uri === linkElement.getAttribute("href")) {
        linkElement.addEventListener("click", (event: Event) => {
          event.preventDefault()
          router.push({ name: "post", params: { postId: linkedPost.id } })
        })
        linkElement.dataset.internalLink = "true"
      }
    }
  }
}

function getContent(): string {
  let content = addGreentext(props.post.content)
  // Replace emoji shortcodes
  content = replaceShortcodes(content, props.post.emojis)
  return content
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/theme";
@import "../styles/mixins";

.post-content {
  color: var(--text-color);
  line-height: 1.5;
  word-wrap: break-word;

  :deep(p) {
    white-space: pre-wrap;
  }

  :deep(h1),
  :deep(h2),
  :deep(h3),
  :deep(h4),
  :deep(h5),
  :deep(h6),
  :deep(p),
  :deep(blockquote),
  :deep(ul),
  :deep(ol),
  :deep(hr),
  :deep(pre) {
    &:not(:last-child) {
      margin-bottom: 1rem;
    }
  }

  :deep(h1) {
    font-size: 1.6rem;
  }

  :deep(h2) {
    font-size: 1.4rem;
  }

  :deep(h3) {
    font-size: 1.2em;
  }

  :deep(h4),
  :deep(h5),
  :deep(h6) {
    font-size: 1rem;
  }

  :deep(a) {
    @include block-external-link;

    /* data-object-link is set by configureInlineLinks() */
    &[data-internal-link="true"] {
      @include block-link;
    }
  }

  :deep(pre),
  :deep(code) {
    overflow-x: auto;
  }

  :deep(ul),
  :deep(ol),
  :deep(li) {
    list-style-position: inside;
  }

  :deep(ul) {
    list-style-type: disc;
  }

  :deep(li li) {
    padding-left: 20px;
  }

  :deep(.greentext) {
    color: $greentext-color;
  }

  :deep(blockquote) {
    color: $greentext-color;

    &::before {
      content: ">";
      float: left;
    }
  }

  :deep(.emoji) {
    display: inline-block;
    height: $emoji-size;
    position: relative;
    vertical-align: text-bottom;
    width: $emoji-size;

    img {
      height: inherit;
      left: 0;
      max-width: none;
      object-fit: contain;
      position: absolute;
      top: 0;
      width: inherit;
      z-index: 0;

      &:hover {
        height: $emoji-size * 2;
        left: -50%;
        top: -50%;
        transition: 100ms linear;
        transition-delay: 0.5s;
        width: $emoji-size * 2;
        z-index: 1;
      }
    }
  }
}
</style>
