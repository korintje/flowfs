<template>
  <sidebar-layout>
    <template #content>
      <h1>Identities</h1>
      <section v-if="aliases.declared_all.length > 0">
        <h2>Declared aliases</h2>
        <div class="profile-group" v-for="alias in aliases.declared_all" :key="alias.id">
          <router-link
            v-if="alias.account !== null"
            :to="getActorLocation('profile', alias.account)"
          >
            <profile-list-item :profile="alias.account"></profile-list-item>
          </router-link>
          <profile-list-item
            v-else
            :profile="defaultProfile({ display_name: 'Unknown', url: alias.id })"
          ></profile-list-item>
          <button
            class="remove-alias icon"
            title="Remove alias"
            @click="onRemoveAlias(alias.id)"
          >
            <icon-delete></icon-delete>
          </button>
        </div>
      </section>
      <section v-if="aliases.verified.length > 0">
        <h2>Verified aliases</h2>
        <router-link
          v-for="profile in aliases.verified"
          :key="profile.id"
          :to="getActorLocation('profile', profile)"
        >
          <profile-list-item :profile="profile"></profile-list-item>
        </router-link>
      </section>
      <section>
        <h2>Add alias</h2>
        <form @submit.prevent="onAddAlias()">
          <div class="input-group">
            <input
              id="alias"
              type="text"
              v-model.trim="newAlias"
              placeholder="Fediverse address"
              @input="newAliasSuggestions = []; newAliasError = null"
            >
            <div class="suggestions" v-if="newAliasSuggestions.length > 0">
              <button
                class="suggestion"
                v-for="profile in newAliasSuggestions"
                :key="profile.id"
                @click="newAlias = profile.acct; newAliasSuggestions = []"
              >
                {{ profile.acct }}
              </button>
            </div>
          </div>
          <button
            type="submit"
            class="btn"
            :disabled="!canAddAlias()"
          >
            Add
          </button>
          <div class="error-message" v-if="newAliasError">
            {{ newAliasError }}
          </div>
        </form>
      </section>
      <loader v-if="isLoading"></loader>
    </template>
  </sidebar-layout>
</template>

<script setup lang="ts">
import { onMounted } from "vue"
import { $, $ref } from "vue/macros"

import { searchProfilesByAcct } from "@/api/search"
import { addAlias, removeAlias } from "@/api/settings"
import { defaultProfile, getAliases, Aliases, Profile } from "@/api/users"
import IconDelete from "@/assets/feather/x.svg?component"
import SidebarLayout from "@/components/SidebarLayout.vue"
import Loader from "@/components/Loader.vue"
import ProfileListItem from "@/components/ProfileListItem.vue"
import { useActorHandle } from "@/composables/handle"
import { useCurrentUser } from "@/composables/user"

const { getActorLocation } = useActorHandle()
const { ensureCurrentUser, ensureAuthToken } = $(useCurrentUser())

let aliases = $ref<Aliases>({ declared: [], declared_all: [], verified: [] })
let isLoading = $ref(false)
let newAlias = $ref<string>("")
let newAliasSuggestions = $ref<Profile[]>([])
let newAliasError = $ref<string | null>(null)

onMounted(async () => {
  isLoading = true
  aliases = await getAliases(ensureCurrentUser().id)
  isLoading = false
})

function canAddAlias(): boolean {
  return newAlias.length > 0 && newAliasError === null
}

async function onAddAlias() {
  isLoading = true
  const profiles = await searchProfilesByAcct(
    ensureAuthToken(),
    newAlias,
    true,
    5,
  )
  if (profiles.length === 0) {
    newAliasError = "profile not found"
    isLoading = false
    return
  }
  if (profiles.length === 1 && profiles[0].acct === newAlias) {
    try {
      aliases = await addAlias(ensureAuthToken(), newAlias)
    } catch (error: any) {
      newAliasError = error.message
      isLoading = false
      return
    }
    newAlias = ""
  } else {
    newAliasSuggestions = profiles
  }
  isLoading = false
}

async function onRemoveAlias(actorId: string) {
  isLoading = true
  aliases = await removeAlias(ensureAuthToken(), actorId)
  isLoading = false
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";
@import "../styles/theme";

section {
  margin-bottom: $block-outer-padding;
}

.profile {
  margin-bottom: $block-outer-padding;
}

.profile-group {
  margin-bottom: $block-outer-padding;
  position: relative;

  .profile {
    margin-bottom: 0;
  }

  .remove-alias {
    align-items: center;
    display: flex;
    padding: calc($block-inner-padding / 2);
    position: absolute;
    right: 0;
    top: 0;

    svg {
      height: $icon-size;
      min-width: $icon-size;
      stroke: var(--secondary-text-color);
      width: $icon-size;
    }

    &:hover {
      svg {
        stroke: var(--secondary-text-hover-color);
      }
    }
  }
}

form {
  @include content-form;

  .suggestions {
    background-color: var(--block-background-color);
    border-radius: $btn-border-radius;
    display: flex;
    flex-direction: column;
    margin-top: 1px;

    .suggestion {
      padding: calc($input-padding / 2) $input-padding;
      text-align: left;
      word-wrap: break-word;

      &:first-child {
        padding-top: $input-padding;
      }

      &:last-child {
        padding-bottom: $input-padding;
      }
    }
  }
}

.loader {
  margin: $block-outer-padding auto;
}
</style>
