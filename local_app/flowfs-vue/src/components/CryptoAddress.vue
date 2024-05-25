<template>
  <div class="crypto-address">
    <input type="text" :value="address" readonly>
    <button class="copy-btn" title="Copy address" @click="copyAddress()">
      <icon-copy></icon-copy>
    </button>
  </div>
</template>

<script setup lang="ts">
import IconCopy from "@/assets/forkawesome/files-o.svg?component"

const props = defineProps<{
  address: string,
}>()

function copyAddress() {
  navigator.clipboard.writeText(props.address)
}
</script>

<style scoped lang="scss">
@import "../styles/layout";

.crypto-address {
  background-color: var(--widget-background-color);
  border-radius: calc($icon-size / 2);
  display: flex;
  line-height: $icon-size;

  input {
    background-color: inherit;
    border: none;
    border-radius: 10px 0 0 10px;
    color: var(--text-color);
    font-family: monospace;
    font-size: 12px;
    max-width: 200px;
    padding: 0 7px;
    width: 100%;
  }

  .copy-btn {
    border-radius: 0 10px 10px 0;
    height: $icon-size;
    min-width: $icon-size;
    padding: 3px 7px 3px 0;
    width: $icon-size;

    svg {
      fill: var(--link-color);
      height: 100%;
      object-fit: contain;
      width: 100%;
    }

    &:hover svg {
      fill: var(--link-hover-color);
    }
  }
}
</style>
