<template>
  <div
    class="qr-wrapper"
    v-if="qrSvg"
    v-html="qrSvg"
    :title="url"
  ></div>
</template>

<script setup lang="ts">
import { onMounted } from "vue"
import { $ref } from "vue/macros"
import QRCode from "qrcode"

const props = defineProps<{
  url: string,
}>()

let qrSvg = $ref<string | null>(null)

onMounted(async () => {
  qrSvg = await QRCode.toString(props.url, { type: "svg" })
})
</script>

<style scoped lang="scss">
:deep(svg) {
  width: 100%;
}
</style>
