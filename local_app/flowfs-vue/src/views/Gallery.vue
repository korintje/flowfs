<template>
  <div class="page wide" v-if="profile">
    <div class="page-content">
      <router-link
        class="back-btn"
        title="Back"
        :to="getActorLocation('profile', profile)"
      >
        <icon-arrow-left></icon-arrow-left>
      </router-link>
      <h1><profile-display-name :profile="profile"></profile-display-name> gallery</h1>
      <div
        v-if="posts.length > 0"
        class="post-grid"
      >
        <div
          v-for="post in posts"
          class="post"
          :key="post.id"
        >
          <post-attachment
            class="media"
            :attachment="post.media_attachments[0]"
            :is-sensitive="post.sensitive"
          >
          </post-attachment>
          <router-link
            class="post-link"
            title="View post"
            :to="{ name: 'post', params: { postId: post.id }}"
          >
            <icon-comment></icon-comment>
          </router-link>
        </div>
      </div>
      <h2 v-else-if="!isLoading" class="empty">
        No media found
      </h2>
      <loader v-if="isLoading"></loader>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue"
import { useRoute } from "vue-router"

import { getProfileTimeline, Post as PostObject } from "@/api/posts"
import { getProfile, lookupProfile, ProfileWrapper } from "@/api/users"
import IconArrowLeft from "@/assets/feather/arrow-left.svg?component"
import IconComment from "@/assets/forkawesome/comment-o.svg?component"
import Loader from "@/components/Loader.vue"
import PostAttachment from "@/components/PostAttachment.vue"
import ProfileDisplayName from "@/components/ProfileDisplayName.vue"
import { useActorHandle } from "@/composables/handle"
import { useTheme } from "@/composables/theme"
import { useCurrentUser } from "@/composables/user"

const route = useRoute()
const { getActorLocation } = useActorHandle()
const { authToken } = useCurrentUser()
const { loadTheme } = useTheme()

const profile = ref<ProfileWrapper | null>(null)
const posts = ref<PostObject[]>([])
const isLoading = ref(false)

onMounted(async () => {
  isLoading.value = true
  loadTheme()
  let _profile
  if (route.params.acct) {
    _profile = await lookupProfile(
      authToken.value,
      route.params.acct as string,
    )
  } else {
    _profile = await getProfile(
      authToken.value,
      route.params.profileId as string,
    )
  }
  profile.value = new ProfileWrapper(_profile)
  posts.value = await getProfileTimeline(
    authToken.value,
    profile.value.id,
    false,
    false,
    true,
  )
  isLoading.value = false
})
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";

$page-width: $wide-content-width + $content-gap + $wide-sidebar-width;

.page-content {
  margin: 0 auto;
  max-width: 100%;
  width: $page-width;

  .back-btn {
    display: block;
    position: absolute;
    top: $body-padding;

    svg {
      height: 40px;
      stroke: var(--text-color);
      width: 40px;
    }
  }

  h1 {
    margin-bottom: $block-outer-padding * 2;
    margin-top: $block-outer-padding * 2 - $body-padding;
    text-align: center;
  }
}

.post-grid {
  display: grid;
  gap: $block-outer-padding;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  padding-bottom: 5%;

  .post {
    height: 30vh;
    min-width: 0;
    position: relative;

    .media {
      height: 100%;
    }
  }
}

.post-link {
  @include media-btn;

  /* Same positioning is used PostAttachment.vue */
  position: absolute;
  right: $body-padding;
  top: $body-padding;
}

.empty {
  text-align: center;
}

.loader {
  margin: 0 auto;
}

@media screen and (max-width: $screen-breakpoint-x-small) {
  .post-grid {
    grid-template-columns: 1fr;
  }
}
</style>
