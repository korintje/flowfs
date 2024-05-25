<template>
  <div class="post-preview">
    <div class="post-preview-header">
      <avatar :profile="author"></avatar>
      <profile-display-name :profile="author">
      </profile-display-name>
      <span
        class="actor-address"
        :title="getActorHandle(author)"
      >
        {{ getActorHandle(author) }}
      </span>
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
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"

import { Post } from "@/api/posts"
import { ProfileWrapper } from "@/api/users"
import Avatar from "@/components/Avatar.vue"
import PostAttachment from "@/components/PostAttachment.vue"
import PostContent from "@/components/PostContent.vue"
import ProfileDisplayName from "@/components/ProfileDisplayName.vue"
import { useActorHandle } from "@/composables/handle"

const props = defineProps<{
  post: Post,
}>()

const { getActorHandle } = useActorHandle()

const author = computed(() => new ProfileWrapper(props.post.account))
</script>

<style scoped lang="scss">
@import "../styles/layout";

.post-preview {
  border: 1px solid var(--separator-color);
  border-radius: $block-border-radius;

  &:hover {
    background-color: var(--widget-background-color);
  }
}

.post-preview-header {
  align-items: center;
  color: var(--secondary-text-color);
  display: flex;
  flex-direction: row;
  gap: calc($block-inner-padding / 2);
  padding: $block-inner-padding $block-inner-padding 0;

  .avatar {
    height: $icon-size;
    width: $icon-size;
  }

  .display-name {
    color: var(--text-color);
    font-weight: bold;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .actor-address {
    overflow: hidden;
    text-overflow: ellipsis;
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
</style>
