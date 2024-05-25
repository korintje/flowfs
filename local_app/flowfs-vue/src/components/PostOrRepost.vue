<template>
  <template v-if="post.reblog">
    <div class="action">
      <icon-repost></icon-repost>
      <router-link
        :to="getActorLocation('profile', post.account)"
        :title="getActorHandle(post.account)"
        class="display-name-link"
      >
        <profile-display-name :profile="author"></profile-display-name>
      </router-link>
      <span>reposted</span>
    </div>
    <post
      :post="post.reblog"
      :highlighted="false"
      :in-thread="false"
      @post-deleted="onPostDeleted((post.reblog as PostObject).id); onPostDeleted(post.id)"
    ></post>
  </template>
  <post
    v-else
    :post="post"
    :highlighted="false"
    :in-thread="false"
    @post-deleted="onPostDeleted(post.id)"
  ></post>
</template>

<script setup lang="ts">
import { computed } from "vue"

import type { Post as PostObject } from "@/api/posts"
import { ProfileWrapper } from "@/api/users"
import IconRepost from "@/assets/feather/repeat.svg?component"
import Post from "@/components/Post.vue"
import ProfileDisplayName from "@/components/ProfileDisplayName.vue"
import { useActorHandle } from "@/composables/handle"

const props = defineProps<{
  post: PostObject,
}>()

const emit = defineEmits<{(event: "post-deleted", postId: string): void}>()

const { getActorHandle, getActorLocation } = useActorHandle()

const author = computed(() => new ProfileWrapper(props.post.account))

function onPostDeleted(postId: string) {
  emit("post-deleted", postId)
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";
@import "../styles/theme";

.action {
  @include post-action;
}

.post,
.post-edit-form {
  margin-bottom: $block-outer-padding;
}
</style>
