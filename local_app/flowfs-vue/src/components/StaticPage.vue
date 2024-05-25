<template>
  <div class="page">
    <div class="page-content">
      <router-link class="back" to="/" title="Back">
        <icon-arrow-left></icon-arrow-left>
      </router-link>
      <h1 class="page-heading"><slot name="heading"></slot></h1>
      <div class="static-text"><slot name="text"></slot></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue"

import IconArrowLeft from "@/assets/feather/arrow-left.svg?component"
import { useTheme } from "@/composables/theme"

const { loadTheme } = useTheme()

onMounted(() => {
  loadTheme()
})
</script>

<style scoped lang="scss">
@import "../styles/layout";

.page {
  margin: 0 auto;
  max-width: $wide-content-width + $content-gap + $wide-sidebar-width;
  padding-top: 20vh;
}

.page-content {
  max-width: $wide-content-width;
  min-width: 0;
}

.back {
  position: absolute;
  top: $body-padding;

  svg {
    height: 40px;
    stroke: var(--text-color);
    width: 40px;
  }
}

h1.page-heading {
  font-size: 90px;
  font-weight: bold;
  margin: 0 0 20px;
  text-transform: uppercase;
  word-wrap: break-word;
}

.static-text {
  font-size: 24px;
}

@media screen and (max-width: $screen-breakpoint-small) {
  .page {
    padding-top: $content-gap;
  }
}
</style>
