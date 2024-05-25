<template>
  <div class="instance-info" v-if="instance">
    <h1 class="instance-title">{{ instance.title }}</h1>
    <div class="instance-description">{{ instance.short_description }}</div>
    <router-link class="btn" :to="{ name: 'about' }">Learn more</router-link>
  </div>
</template>

<script setup lang="ts">
import { $ } from "vue/macros"

import { useInstanceInfo } from "@/composables/instance"

const { instance } = $(useInstanceInfo())
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";

.instance-info {
  @include block-btn;

  background-color: var(--block-background-color);
  border-radius: $block-border-radius;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  flex-shrink: 1;
  gap: $block-inner-padding;
  padding: $block-inner-padding;

  h1 {
    font-size: 32px;
    font-weight: bold;
    margin-bottom: 0;
  }

  .btn {
    width: min-content;
  }
}
</style>
