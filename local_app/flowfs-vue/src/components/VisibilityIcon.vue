<template>
  <icon-globe v-if="visibility === 'public'"></icon-globe>
  <icon-users v-else-if="visibility === 'private'"></icon-users>
  <icon-payment v-else-if="visibility === 'subscribers'"></icon-payment>
  <icon-envelope v-else-if="visibility === 'direct'"></icon-envelope>
</template>

<script setup lang="ts">
import IconGlobe from "@/assets/feather/globe.svg?component"
import IconEnvelope from "@/assets/forkawesome/envelope-o.svg?component"
import IconPayment from "@/assets/tabler/coin.svg?component"
import IconUsers from "@/assets/tabler/users.svg?component"

defineProps<{
  visibility: string,
}>()
</script>
