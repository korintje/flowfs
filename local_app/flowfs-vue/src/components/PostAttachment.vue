<template>
  <div
    v-if="attachment.type === 'image'"
    class="image"
    :class="{ sensitive: contentWarningEnabled }"
  >
    <button
      v-if="contentWarningEnabled"
      class="show-image"
      @click="showImage()"
    >
      Sensitive content
    </button>
    <button
      v-else-if="isSensitive"
      class="hide-image"
      title="Hide image"
      @click="hideImage()"
    >
      <icon-hide></icon-hide>
    </button>
    <img
      :src="attachment.url"
      :alt="attachment.description || undefined"
      :title="attachment.description || undefined"
      @click="onImageClick()"
    >
    <div
      v-if="lightboxOpen"
      class="lightbox"
      @click="closeLightbox()"
    >
      <button title="Close">
        <icon-close></icon-close>
      </button>
      <img
        :src="attachment.url"
        :alt="attachment.description || undefined"
        :title="attachment.description || undefined"
      >
    </div>
  </div>
  <video v-else-if="attachment.type === 'video'" :src="attachment.url" controls></video>
  <audio v-else-if="attachment.type === 'audio'" :src="attachment.url" controls></audio>
  <table v-else class="document">
    <tr>
      <td><icon-file></icon-file></td>
      <td><a :href="attachment.url">{{ attachment.url }}</a></td>
    </tr>
  </table>
</template>

<script setup lang="ts">
import { ref } from "vue"

import { Attachment } from "@/api/posts"
import IconHide from "@/assets/feather/eye-off.svg?component"
import IconFile from "@/assets/feather/file.svg?component"
import IconClose from "@/assets/feather/x.svg?component"
import { useClientConfig } from "@/composables/client-config"

const { contentWarningsEnabled } = useClientConfig()

const props = defineProps<{
  attachment: Attachment,
  isSensitive: boolean,
}>()

const contentWarningEnabled = ref(props.isSensitive && contentWarningsEnabled.value)
const lightboxOpen = ref(false)

function showImage() {
  contentWarningEnabled.value = false
}

function hideImage() {
  contentWarningEnabled.value = true
}

function openLightbox() {
  lightboxOpen.value = true
}

function closeLightbox() {
  lightboxOpen.value = false
}

function onImageClick() {
  if (props.isSensitive && contentWarningEnabled.value === true) {
    // If post is marked as sensitive, hide content warning
    showImage()
  } else {
    openLightbox()
  }
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";

button {
  @include media-btn;
}

.image {
  overflow: hidden;
  position: relative;

  .show-image {
    left: 50%;
    position: absolute;
    top: 50%;
    transform: translate(-50%, -50%);
    z-index: 1;
  }

  .hide-image {
    left: $body-padding;
    padding: $input-padding;
    position: absolute;
    top: $body-padding;
    z-index: 1;
  }

  > img {
    cursor: zoom-in;
    display: block;
    height: 100%;
    object-fit: cover;
    width: 100%;
  }

  &.sensitive > img {
    cursor: initial;
    filter: blur(50px);
  }
}

.lightbox {
  background-color: rgb(0 0 0 / 75%);
  bottom: 0;
  box-sizing: border-box;
  display: flex;
  justify-content: center;
  left: 0;
  padding: $body-padding;
  position: fixed;
  right: 0;
  top: 0;
  width: 100%;
  z-index: $header-z-index + 2; /* image must be above sidebar */

  button {
    position: absolute;
    right: $body-padding;
    top: $body-padding;
  }

  > img {
    background-color: var(--block-background-color);
    object-fit: contain;
  }
}

audio,
video {
  width: 100%;
}

.document {
  table-layout: fixed;
  width: 100%;

  td {
    border: 1px solid var(--separator-color);
    padding: $input-padding;
  }

  td:first-child {
    width: $icon-size * 2;

    svg {
      height: 100%;
      stroke: var(--secondary-text-color);
      vertical-align: middle;
      width: 100%;
    }
  }

  a {
    @include block-link;

    word-wrap: break-word;
  }
}
</style>
