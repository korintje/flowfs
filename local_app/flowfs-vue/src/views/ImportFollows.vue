<template>
  <sidebar-layout>
    <template #content>
      <h1>Import follows</h1>
      <form class="import-follows">
        <div class="input-group">
          <textarea
            id="follows"
            placeholder="Follows (CSV)"
            v-model="followsCsv"
          >
          </textarea>
        </div>
        <button
          type="submit"
          class="btn"
          :disabled="!canImport() || isLoading"
          @click.prevent="submit()"
        >
          Import
        </button>
        <div class="error-message" v-if="errorMessage">{{ errorMessage }}</div>
      </form>
    </template>
  </sidebar-layout>
</template>

<script setup lang="ts">
import { $, $ref } from "vue/macros"
import { useRouter } from "vue-router"

import { importFollows } from "@/api/settings"
import SidebarLayout from "@/components/SidebarLayout.vue"
import { useActorHandle } from "@/composables/handle"
import { useCurrentUser } from "@/composables/user"

const router = useRouter()
const { getActorLocation } = useActorHandle()
const { currentUser, ensureAuthToken } = $(useCurrentUser())

const followsCsv = $ref("")
let isLoading = $ref(false)
let errorMessage = $ref<string | null>(null)

function canImport(): boolean {
  return followsCsv.length > 0
}

async function submit() {
  if (currentUser === null) {
    return
  }
  isLoading = true
  try {
    await importFollows(
      ensureAuthToken(),
      followsCsv,
    )
  } catch (error: any) {
    isLoading = false
    errorMessage = error.message
    return
  }
  isLoading = false
  errorMessage = null
  router.push(getActorLocation("profile", currentUser))
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";
@import "../styles/theme";

form {
  @include content-form;
}
</style>
