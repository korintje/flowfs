<template>
  <div class="avatar">
    <img :src="avatarUrl">
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import makeBlockie from "ethereum-blockies-base64"

import { Profile } from "@/api/users"

const props = defineProps<{
  profile: Profile,
}>()

const UNNAMED = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGQAAABkCAYAAABw4pVUAAAC8ElEQVR4nO3Y3UtTcRzHcf+nPdwccQ4GmeBymZDXYUgP2BTbdOIfURmFMibOdJPZ00WRBq1wq3RBBlkIS106N92m9gd8vWrwW5B69fuccz4X78svfH/ntXMOO01/jivCcGrSvQAjCHQEAYsgYBEELIKARRCwCAIWQcAiCFgEAYsgYBEELIKARRCwCAIWQcAiCFgEAYsgYBEELIKARRCwCAIWQcAiCFgEAYsgYBEELIKARRCwCAKWKUGKu9sSn47J4GBQ/B0d4vG0iNvtFm9rqwQCnRK6OySJxBPZL+9o39XSIIe1skxMPJLm5mZxOByn5vG0yEx8SvvelgSpVvbk1s0bZ4JoLDISluOjA+1nsBRIZCSsXGSn0ynBYL8spJLyJfdJ1r+vyepKVubmZqS399o/KA/H72s/g2VA0u+WlIvr9XpleTn935mFVFJcLld9xjAM2drc0H4WS4A0Pqoyp2D8bfzBPWXODO8TeJC94rbyS+/vv33m2eLutrjd7vrscDik/TymB3mffqv8ypPJ2XPNX/L767N9fde1n8f0IEeH+1Io5OXH+jdZ+ZyR3Z2tc813dV0mCEq1akkMw6iDjI2Nat/J1iDz87PK4+75s5T2nWwLUijkxefz1THa2i5IrVrSvpctQbY2N6S7+4pydyy+eaV9L1uCZLMfpL39ooIxFYtq38uWIJOTj5X/HYZhyMsXT7XvZTuQw1pZQqEh5a4IBDpl7WtO+262BGn88DgwcEfKpd/a97IlyNLiawVjNDJsmk/tlgTp6bmqPKYqB0XtO9kW5Ff+p3J3xKdj2neyNUhmOa2A5FY/at/J1iBWjCBgEQQsU4NEoxPKOySVSmjfiSAEwYkgYBGEEcRuEQQsgoBFELAIAhZBwCIIWAQBiyBgEQQsgoBFELAIAhZBwCIIWAQBiyBgEQQsgoBFELAIAhZBwCIIWAQBiyBgEQQsgoBFELAIAhZBwCIIWAQBiyBgEQQsgoBFELAIAhZBwDoBOz/dnwXMOO8AAAAASUVORK5CYII="

const avatarUrl = computed<string>(() => {
  const profile = props.profile
  if (profile.avatar) {
    return profile.avatar
  } else if (profile.id === "") {
    return UNNAMED
  } else {
    return makeBlockie(profile.acct)
  }
})
</script>

<style scoped lang="scss">
.avatar {
  background-color: var(--block-background-color);
  border-radius: 50%;
  box-sizing: border-box;

  img {
    border-radius: inherit;
    height: 100%;
    object-fit: cover;
    width: 100%;
  }
}
</style>
