<template>
  <span
    class="display-name"
    v-html="getDisplayNameHtml()"
  >
  </span>
</template>

<script setup lang="ts">
import { replaceShortcodes } from "@/api/emojis"
import { ProfileWrapper } from "@/api/users"

const props = defineProps<{
  profile: ProfileWrapper,
}>()

function getDisplayNameHtml(): string {
  const profile = props.profile
  const escaped = new Option(profile.getDisplayName()).innerHTML
  return replaceShortcodes(escaped, profile.emojis)
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";
@import "../styles/theme";

.display-name {
  @include emoji-inline;
}
</style>
