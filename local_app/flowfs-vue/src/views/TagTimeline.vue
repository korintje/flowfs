<template>
  <sidebar-layout>
    <template #content>
      <div class="tag-name">
        #{{ route.params.tagName }}
      </div>
      <post-list
        :posts="posts"
        @load-next-page="loadNextPage"
      ></post-list>
      <loader v-if="isLoading"></loader>
    </template>
  </sidebar-layout>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue"
import { useRoute } from "vue-router"

import { addRelationships, getTagTimeline, Post } from "@/api/posts"
import Loader from "@/components/Loader.vue"
import PostList from "@/components/PostList.vue"
import SidebarLayout from "@/components/SidebarLayout.vue"
import { useCurrentUser } from "@/composables/user"

const route = useRoute()
const { authToken } = useCurrentUser()
const posts = ref<Post[]>([])
const isLoading = ref(false)

async function loadTimelinePage(
  authToken: string | null,
  maxId?: string,
): Promise<Post[]> {
  const page = await getTagTimeline(
    authToken,
    route.params.tagName as string,
    maxId,
  )
  if (authToken !== null) {
    await addRelationships(authToken, page)
  }
  return page
}

onMounted(async () => {
  isLoading.value = true
  posts.value = await loadTimelinePage(authToken.value)
  isLoading.value = false
})

async function loadNextPage(maxId: string) {
  const nextPage = await loadTimelinePage(authToken.value, maxId)
  posts.value = [...posts.value, ...nextPage]
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";
@import "../styles/theme";

.tag-name {
  @include content-message;

  margin-bottom: $block-outer-padding;
}

.loader {
  margin: $block-outer-padding auto;
}
</style>
