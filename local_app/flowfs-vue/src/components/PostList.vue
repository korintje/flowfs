<template>
  <post-or-repost
    v-for="post in posts"
    :post="post"
    :key="post.id"
    @post-deleted="onPostDeleted($event)"
  ></post-or-repost>
  <button
    v-if="isPageFull()"
    class="btn secondary next-btn"
    :disabled="isNextPageLoading"
    @click="loadNextPage()"
  >
    Show more posts
  </button>
</template>

<script setup lang="ts">
import { watch } from "vue"
import { $ref } from "vue/macros"

import { PAGE_SIZE } from "@/api/common"
import { Post as PostObject } from "@/api/posts"
import PostOrRepost from "@/components/PostOrRepost.vue"

const props = defineProps<{
  posts: PostObject[],
}>()
/* eslint-disable-next-line func-call-spacing */
const emit = defineEmits<{
  (event: "load-next-page", maxId: string): void,
}>()

defineExpose({ resetPagination })

let initialPostCount: number | null = null
let isNextPageLoading = $ref(false)

watch(() => props.posts, (posts) => {
  if (initialPostCount === null) {
    initialPostCount = posts.length
  }
  isNextPageLoading = false
})

function onPostDeleted(postId: string) {
  const posts = props.posts
  const postIndex = posts.findIndex((post) => post.id === postId)
  posts.splice(postIndex, 1)
}

function isPageFull(): boolean {
  return initialPostCount === null ? false : initialPostCount >= PAGE_SIZE
}

function resetPagination() {
  initialPostCount = null
}

function loadNextPage() {
  if (props.posts.length > 0) {
    const maxId = props.posts[props.posts.length - 1].id
    isNextPageLoading = true
    emit("load-next-page", maxId)
  }
}
</script>

<style scoped lang="scss">
@import "../styles/layout";

.next-btn {
  margin-bottom: $block-outer-padding;
}
</style>
