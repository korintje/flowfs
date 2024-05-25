<template>
  <a
    v-if="typeof to === 'string'"
    :title="title"
    :href="to"
    target="_blank"
    rel="noreferrer"
  >
    <slot name="link-content"></slot>
  </a>
  <router-link
    v-else
    :title="title"
    :to="to"
  >
    <slot name="link-content"></slot>
  </router-link>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { RouteLocationRaw } from "vue-router"

const props = defineProps<{
  to: any,
  title?: string,
}>()

// Prop type check is broken (during navigation, not initial load)
// Invalid prop: type check failed for prop "to". Expected String | Null, got Object
const to = computed(() => props.to as string | RouteLocationRaw)
</script>
