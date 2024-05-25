<template>
  <sidebar-layout>
    <template #content>
      <h1 class="content-header">Follow requests</h1>
      <div v-if="!isLoading && profiles.length === 0" class="content-message">
        No follow requests found
      </div>
      <div v-if="!isLoading" class="follow-request-list">
        <router-link
          v-for="profile in profiles"
          :key="profile.id"
          :to="getActorLocation('profile', profile)"
        >
          <profile-list-item :profile="profile"></profile-list-item>
        </router-link>
        <button
          v-if="nextPageUrl"
          class="btn secondary next-btn"
          @click="loadNextPage()"
        >
          Show more requests
        </button>
      </div>
      <loader v-if="isLoading"></loader>
    </template>
  </sidebar-layout>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue"

import { getFollowRequests } from "@/api/relationships"
import { Profile } from "@/api/users"
import Loader from "@/components/Loader.vue"
import ProfileListItem from "@/components/ProfileListItem.vue"
import SidebarLayout from "@/components/SidebarLayout.vue"
import { useActorHandle } from "@/composables/handle"
import { useCurrentUser } from "@/composables/user"

const { getActorLocation } = useActorHandle()
const { ensureAuthToken } = useCurrentUser()

const profiles = ref<Profile[]>([])
const nextPageUrl = ref<string | null>(null)
const isLoading = ref(false)

onMounted(async () => {
  isLoading.value = true
  const page = await getFollowRequests(ensureAuthToken())
  profiles.value = page.profiles
  nextPageUrl.value = page.nextPageUrl
  isLoading.value = false
})

async function loadNextPage() {
  if (nextPageUrl.value === null) {
    return
  }
  const page = await getFollowRequests(
    ensureAuthToken(),
    nextPageUrl.value,
  )
  profiles.value.push(...page.profiles)
  nextPageUrl.value = page.nextPageUrl
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";

.content-header {
  @include content-list-header;

  font-size: inherit;
}

.content-message {
  @include content-message;

  margin-bottom: $block-outer-padding;
}

.follow-request-list {
  display: flex;
  flex-direction: column;
  gap: $block-outer-padding;

  .next-btn {
    align-self: flex-start;
  }
}

.loader {
  margin: $block-outer-padding auto;
}
</style>
