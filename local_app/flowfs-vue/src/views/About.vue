<template>
  <sidebar-layout v-if="currentUser && instance">
    <template #content>
      <h1>{{ instance.title }}</h1>
      <div class="description static-text" v-html="instance.description"></div>
      <template v-if="instance.contact_account">
        <h2 class="staff-header">Administered by</h2>
        <router-link :to="getActorLocation('profile', instance.contact_account)">
          <profile-list-item :profile="instance.contact_account"></profile-list-item>
        </router-link>
      </template>
      <details class="technical-info static-text">
        <summary>Technical Info</summary>
        mitra version: {{ getMitraVersion(instance.version) }}
        <br>
        mitra-web version: {{ APP_VERSION }}
      </details>
    </template>
  </sidebar-layout>
  <static-page v-else-if="currentUser === null && instance" class="wide">
    <template #heading>{{ instance.title }}</template>
    <template #text>
      <div class="description" v-html="instance.description"></div>
      <template v-if="instance.contact_account">
        <h2 class="staff-header">Administered by</h2>
        <router-link :to="getActorLocation('profile', instance.contact_account)">
          <profile-list-item :profile="instance.contact_account"></profile-list-item>
        </router-link>
      </template>
    </template>
  </static-page>
</template>

<script setup lang="ts">
import { APP_VERSION } from "@/constants"
import ProfileListItem from "@/components/ProfileListItem.vue"
import SidebarLayout from "@/components/SidebarLayout.vue"
import StaticPage from "@/components/StaticPage.vue"
import { useActorHandle } from "@/composables/handle"
import { useInstanceInfo } from "@/composables/instance"
import { useCurrentUser } from "@/composables/user"

const { getActorLocation } = useActorHandle()
const { currentUser } = useCurrentUser()
const { instance } = useInstanceInfo()

function getMitraVersion(apiVersion: string): string {
  const match = apiVersion.match(/.+Mitra ([\d.]+)/)
  if (match) {
    return match[1]
  } else {
    return "unknown"
  }
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/theme";

.description {
  word-wrap: break-word;
}

/* Internal page */
.content {
  h1 {
    font-size: 18px * 2.5;
    margin-bottom: calc($block-outer-padding / 2);
  }

  .description {
    font-size: 18px;
  }

  .staff-header {
    margin-top: $block-outer-padding;
  }

  .technical-info {
    font-size: 18px;
    margin-top: $block-outer-padding;

    summary {
      font-weight: bold;
    }
  }
}

/* Public page */
.page-content {
  .profile {
    line-height: normal;

    :deep(.avatar) {
      height: 3em;
      width: 3em;
    }
  }
}

:deep(.page-content) {
  @if $about-background-color != transparent {
    background-color: $about-background-color;
    border-radius: $block-border-radius;
    margin: -$block-inner-padding;
    padding: $block-inner-padding;
  }
}
</style>
