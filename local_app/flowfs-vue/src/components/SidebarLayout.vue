<template>
  <header v-if="currentUser !== null">
    <div id="header">
      <div id="nav">
        <router-link
          class="home-btn"
          :to="{ name: 'home' }"
          @click.prevent="showHomeTimeline()"
        >
          <icon-home></icon-home>
          <span>Home</span>
        </router-link>
        <search />
      </div>
      <div id="profile">
        <router-link
          class="profile-link"
          :to="getActorLocation('profile', currentUser)"
        >
          <avatar :profile="currentUser"></avatar>
          <div class="profile-name">@{{ currentUser.username }}</div>
        </router-link>
      </div>
    </div>
  </header>
  <div v-else id="header-public" class="wide">
    <instance-info></instance-info>
  </div>
  <div id="main" :class="{ wide: currentUser === null }">
    <div class="content">
      <slot name="content"></slot>
    </div>
    <sidebar v-if="currentUser !== null"></sidebar>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue"
import { $ } from "vue/macros"
import { useRoute, useRouter } from "vue-router"

import IconHome from "@/assets/feather/home.svg?component"
import Avatar from "@/components/Avatar.vue"
import InstanceInfo from "@/components/InstanceInfo.vue"
import Search from "@/components/Search.vue"
import Sidebar from "@/components/Sidebar.vue"
import { useActorHandle } from "@/composables/handle"
import { useNotifications } from "@/composables/notifications"
import { useTheme } from "@/composables/theme"
import { useCurrentUser } from "@/composables/user"

const route = useRoute()
const router = useRouter()
const { getActorLocation } = useActorHandle()
const { currentUser, ensureAuthToken } = $(useCurrentUser())
const { loadNotifications } = $(useNotifications())
const { loadTheme } = useTheme()

const emit = defineEmits<{(event: "reload-home"): void}>()

onMounted(() => {
  loadTheme()
})

function showHomeTimeline() {
  if (route.name === "home") {
    loadNotifications(ensureAuthToken())
    emit("reload-home")
  } else {
    router.push({ name: "home" })
  }
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";

header {
  @include main-background;

  box-sizing: border-box;
  height: $header-height;
  margin-bottom: $block-outer-padding;
  padding: $body-padding;
  position: sticky;
  top: 0;
  z-index: $header-z-index;
}

#header {
  display: flex;
  flex-direction: row;
  gap: $content-gap;
  height: 100%;
  margin: 0 auto;
  max-width: $content-width + $content-gap + $sidebar-width;
}

#nav {
  align-items: center;
  display: flex;
  flex-direction: row;
  gap: $body-padding;
  min-width: $content-min-width;
  width: $content-width;

  .home-btn {
    align-items: center;
    background-color: var(--block-background-color);
    border-radius: $btn-border-radius;
    box-shadow: $menu-shadow-size var(--shadow-color);
    box-sizing: border-box;
    color: var(--text-color);
    display: flex;
    flex-direction: row;
    flex-shrink: 0;
    height: 100%;
    padding: 7px $body-padding;

    svg {
      height: 1.2em;
      margin-right: 5px;
      stroke: var(--text-color);
      width: 1.2em;
    }

    span {
      padding-top: 1px;
    }

    &:hover {
      background-color: var(--btn-background-color);
      color: var(--btn-text-color);

      svg {
        stroke: var(--btn-text-color);
      }
    }
  }

  .search {
    background-color: var(--block-background-color);
    box-shadow: $menu-shadow-size var(--shadow-color);
    height: 100%;
    margin: 0 0 0 auto;
    width: 250px;
  }
}

#profile {
  flex-shrink: 0;
  width: $sidebar-width;

  .profile-link {
    align-items: center;
    display: flex;
    flex-direction: row;
    height: 100%;
  }

  .profile-name {
    margin-left: 10px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .avatar {
    height: $avatar-size;
    width: $avatar-size;
  }
}

#header-public {
  @include main-background;

  box-sizing: border-box;
  display: flex;
  justify-content: center;
  position: sticky;
  top: 0;
  z-index: $header-z-index;
}

.instance-info {
  max-width: $wide-content-width;
  min-width: $content-min-width;
  width: $wide-content-width;
}

#main {
  align-items: flex-start;
  display: flex;
  flex-direction: row;
  gap: $content-gap;
  margin: 0 auto;
  max-width: $content-width + $content-gap + $sidebar-width;
}

#main:not(.wide) {
  padding: 0 $body-padding;
}

.content {
  box-sizing: border-box;
  max-width: $content-width;
  min-width: $content-min-width;
  width: $content-width;
}

#main.wide {
  /* main element should not have top padding to make scrollTo impl simpler */
  margin-top: 1px;
  max-width: $wide-content-width;
  padding-top: 0;

  .content {
    max-width: $wide-content-width;
    min-width: $content-min-width;
    width: $wide-content-width;
  }
}

@media screen and (max-width: $screen-breakpoint-medium) {
  #header,
  #main {
    /* Equal to header's bottom padding + margin */
    gap: $block-outer-padding + $body-padding;
  }
}

@media screen and (max-width: $screen-breakpoint-small) {
  header {
    margin-bottom: 0;
  }

  #header {
    gap: $body-padding;
  }

  #nav {
    min-width: auto;
    width: 100%;

    .search {
      width: auto;
    }
  }

  #profile {
    width: auto;

    .profile-name {
      display: none;
    }
  }

  #header-public {
    width: auto;
  }

  #main {
    flex-direction: column-reverse;
    gap: 0;
  }

  #main.wide {
    max-width: none;
  }

  #main .content,
  #main.wide .content,
  .instance-info {
    max-width: none;
    min-width: auto;
    width: 100%;
  }
}

@media screen and (max-width: $screen-breakpoint-x-small) {
  #nav .home-btn {
    svg {
      margin-right: 0;
    }

    span {
      display: none;
    }
  }
}
</style>
