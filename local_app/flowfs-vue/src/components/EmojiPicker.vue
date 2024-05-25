<template>
  <menu class="emoji-picker">
    <li v-if="isLoading"><loader></loader></li>
    <template v-else>
      <li class="search">
        <input
          type="search"
          placeholder="Search..."
          v-model="searchQuery"
          @keydown.enter.prevent
        >
      </li>
      <li v-if="getEmojjList().length === 0">No emojis found</li>
      <li v-else class="custom-emojis">
        <div class="emoji-grid">
          <button
            class="emoji"
            v-for="emoji in getEmojjList()"
            :key="emoji.shortcode"
            :title="getEmojiShortcode(emoji.shortcode)"
            @click.prevent="pick(emoji)"
          >
            <img loading="lazy" :src="emoji.url">
          </button>
        </div>
      </li>
    </template>
  </menu>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue"

import { getCustomEmojis, getEmojiShortcode, CustomEmoji } from "@/api/emojis"
import Loader from "@/components/Loader.vue"

/* eslint-disable-next-line func-call-spacing */
const emit = defineEmits<{
  (event: "emoji-picked", name: string): void,
}>()

const customEmojis = ref<CustomEmoji[]>([])
const searchQuery = ref<string>("")
const isLoading = ref(false)

function getEmojjList() {
  if (searchQuery.value === "") {
    return customEmojis.value
  }
  return customEmojis.value
    .filter(emoji => emoji.shortcode.includes(searchQuery.value))
}

function pick(emoji: CustomEmoji) {
  emit("emoji-picked", emoji.shortcode)
}

onMounted(async () => {
  isLoading.value = true
  customEmojis.value = await getCustomEmojis()
  isLoading.value = false
})
</script>

<style scoped lang="scss">
@import "../styles/layout";

.emoji-picker {
  min-width: 120px;

  .custom-emojis {
    max-height: 200px;
    overflow-y: scroll;
  }

  .emoji-grid {
    display: grid;
    gap: calc($block-inner-padding / 2);
    grid-template-columns: repeat(auto-fit, minmax(min-content, $emoji-size));
    margin-right: calc($block-inner-padding / 2); /* extra space for scrollbar */
    max-width: ($emoji-size + calc($block-inner-padding / 2)) * 4;

    .emoji {
      display: flex;
      height: $emoji-size;
      width: $emoji-size;
    }
  }
}

.search {
  input {
    border: 1px solid var(--separator-color);
    border-radius: $btn-border-radius;
  }
}

.loader {
  --loader-size: #{$icon-size};
  --loader-width: 2px;
}
</style>
