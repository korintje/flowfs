<template>
  <sidebar-layout>
    <template #content>
      <h1>Edit profile</h1>
      <form class="profile-form" @submit.prevent="save()">
        <div class="input-group">
          <label for="display-name">Display name</label>
          <input id="display-name" type="text" v-model.trim="form.display_name">
        </div>
        <div class="input-group">
          <label for="bio">Bio</label>
          <textarea
            id="bio"
            ref="bioInputRef"
            :value="form.note || ''"
            @input="onBioUpdate($event)"
          ></textarea>
        </div>
        <div class="image-upload-group">
          <profile-card :profile="profilePreview" :compact="true"></profile-card>
          <div class="image-upload-inputs">
            <div class="input-group">
              <label for="avatar">Avatar</label>
              <input
                type="file"
                id="avatar"
                ref="avatarInputRef"
                :accept="getAcceptedMediaTypes()"
                @change="onFilePicked('avatar', $event)"
              >
              <button
                v-if="images.avatar !== null"
                @click.prevent="onFileRemoved('avatar')"
              >
                Remove
              </button>
            </div>
            <div class="input-group">
              <label for="banner">Banner</label>
              <input
                type="file"
                id="banner"
                ref="bannerInputRef"
                :accept="getAcceptedMediaTypes()"
                @change="onFilePicked('header', $event)"
              >
              <button
                v-if="images.header !== null"
                ref="headerInputRef"
                @click.prevent="onFileRemoved('header')"
              >
                Remove
              </button>
            </div>
          </div>
        </div>
        <div class="input-group">
          <input
            type="checkbox"
            id="locked"
            v-model="form.locked"
          >
          <label for="locked">
            Lock account
            <div class="sub-label">Requires you to manually approve followers</div>
          </label>
        </div>
        <div class="input-group">
          <label for="mention_policy">
            Accept mentions from
            <div class="sub-label">Applies to direct messages and public posts</div>
          </label>
          <select
            id="mention_policy"
            v-model="form.mention_policy"
          >
            <option
              v-for="policy in MENTION_POLICIES"
              :value="policy.value"
              :key="policy.value"
            >
              {{ policy.name }}
            </option>
          </select>
        </div>
        <div class="extra-fields input-group">
          <label>
            Additional info
            <div class="sub-label">You can have up to {{ EXTRA_FIELD_COUNT_MAX }} items displayed as a table on your profile</div>
          </label>
          <div
            v-for="(field, index) in form.fields_attributes"
            :key="index"
            class="extra-field"
            :class="{'error': !isValidExtraField(index)}"
          >
            <input
              type="text"
              v-model.trim="field.name"
              placeholder="Label"
            >
            <input
              type="text"
              v-model.trim="field.value"
              placeholder="Content"
            >
            <a
              class="remove-extra-field"
              title="Remove item"
              @click="removeExtraField(index)"
            >
              <div class="remove-icon">
                <icon-remove></icon-remove>
              </div>
            </a>
          </div>
          <button
            v-if="canAddExtraField()"
            type="button"
            class="add-extra-field"
            @click="addExtraField()"
          >
            <icon-add></icon-add>
            Add new item
          </button>
        </div>
        <button
          type="submit"
          class="btn"
          :disabled="!isFormValid() || isLoading"
        >
          Save
        </button>
        <div class="error-message" v-if="errorMessage">{{ errorMessage }}</div>
      </form>
    </template>
  </sidebar-layout>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue"
import { $computed, $ref } from "vue/macros"
import { useRouter } from "vue-router"

import {
  Profile,
  ProfileUpdateData,
  updateProfile,
  EXTRA_FIELD_COUNT_MAX,
} from "@/api/users"
import IconAdd from "@/assets/feather/plus-circle.svg?component"
import IconRemove from "@/assets/feather/x-circle.svg?component"
import ProfileCard from "@/components/ProfileCard.vue"
import SidebarLayout from "@/components/SidebarLayout.vue"
import { useActorHandle } from "@/composables/handle"
import { useInstanceInfo } from "@/composables/instance"
import { useCurrentUser } from "@/composables/user"
import { setupAutoResize } from "@/utils/autoresize"
import { fileToDataUrl, dataUrlToBase64 } from "@/utils/upload"

const MENTION_POLICIES = [
  { name: "Everybody", value: "none" },
  { name: "Everybody except new accounts", value: "only_known" },
  { name: "People I follow and my followers", value: "only_contacts" },
]

const router = useRouter()
const { getActorLocation } = useActorHandle()
const { instance } = useInstanceInfo()
const { ensureCurrentUser, setCurrentUser, ensureAuthToken } = useCurrentUser()

const profile = ensureCurrentUser()
let isLoading = $ref(false)
let errorMessage = $ref<string | null>(null)

function getFieldsAttributes() {
  const fields_attributes = []
  for (let index = 0; index < profile.fields.length; index++) {
    const field_attributes = {
      name: profile.source.fields[index].name,
      value: profile.source.fields[index].value,
    }
    fields_attributes.push(field_attributes)
  }
  return fields_attributes
}

const form = $ref<ProfileUpdateData>({
  display_name: profile.display_name,
  note: profile.source.note,
  avatar: null,
  avatar_media_type: null,
  header: null,
  header_media_type: null,
  locked: profile.locked,
  mention_policy: profile.mention_policy,
  fields_attributes: getFieldsAttributes(),
})
const images = $ref({
  avatar: profile.avatar,
  header: profile.header,
})

const bioInputRef = $ref<HTMLTextAreaElement | null>(null)
const avatarInputRef = ref<HTMLInputElement | null>(null)
const bannerInputRef = ref<HTMLInputElement | null>(null)

onMounted(() => {
  if (bioInputRef) {
    setupAutoResize(bioInputRef)
  }
})

const profilePreview = $computed<Profile>(() => {
  return {
    ...profile,
    display_name: form.display_name,
    avatar: images.avatar,
    header: images.header,
  }
})

function onBioUpdate(event: Event) {
  const value = (event.target as HTMLTextAreaElement).value
  form.note = value || ""
}

function getAcceptedMediaTypes(): string {
  if (instance.value === null) {
    return ""
  }
  return instance.value
    .configuration.media_attachments.supported_mime_types
    .filter(mediaType => mediaType.startsWith("image/"))
    .join(",")
}

async function onFilePicked(fieldName: "avatar" | "header", event: Event) {
  const files = (event.target as HTMLInputElement).files
  if (!files) {
    return
  }
  const imageDataUrl = await fileToDataUrl(files[0])
  images[fieldName] = imageDataUrl
  const imageData = dataUrlToBase64(imageDataUrl)
  form[fieldName] = imageData.data
  form[`${fieldName}_media_type`] = imageData.mediaType
}

function onFileRemoved(fieldName: "avatar" | "header") {
  // Clear inputs
  if (fieldName === "avatar" && avatarInputRef.value !== null) {
    avatarInputRef.value.value = ""
  }
  if (fieldName === "header" && bannerInputRef.value !== null) {
    bannerInputRef.value.value = ""
  }
  // Remove preview
  images[fieldName] = null
  // Empty string removes the image from profile
  form[fieldName] = ""
  form[`${fieldName}_media_type`] = null
}

function isValidExtraField(index: number): boolean {
  const field = form.fields_attributes[index]
  for (let prevIndex = 0; prevIndex < index; prevIndex++) {
    const prevField = form.fields_attributes[prevIndex]
    if (field.name && field.name === prevField.name) {
      // Label is not unique
      return false
    }
  }
  return true
}

function removeExtraField(index: number) {
  form.fields_attributes.splice(index, 1)
}

function canAddExtraField(): boolean {
  return form.fields_attributes.length <= EXTRA_FIELD_COUNT_MAX
}

function addExtraField() {
  form.fields_attributes.push({ name: "", value: "" })
}

function isFormValid(): boolean {
  if (form.display_name && form.display_name.length > 75) {
    return false
  }
  return true
}

async function save() {
  const authToken = ensureAuthToken()
  isLoading = true
  errorMessage = null
  let user
  try {
    user = await updateProfile(authToken, form)
  } catch (error: any) {
    isLoading = false
    errorMessage = error.message
    return
  }
  isLoading = false
  setCurrentUser(user)
  router.push(getActorLocation("profile", user))
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";
@import "../styles/theme";

.profile-form {
  @include content-form;

  margin-bottom: $block-outer-padding;
}

.image-upload-group {
  display: grid;
  gap: $input-padding;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  width: 100%;

  .image-upload-inputs {
    display: flex;
    flex-direction: column;
    gap: $input-padding;
  }

  .input-group {
    label {
      margin-bottom: 0;
    }

    input {
      width: 100%;
    }
  }

  button {
    @include block-link;

    margin-top: calc($input-padding / 2);
    text-decoration: underline;
  }
}

.extra-field {
  display: flex;
  gap: $input-padding;
  margin-bottom: $input-padding;
  position: relative;

  input {
    flex-basis: 50%;
  }

  .remove-extra-field {
    $icon-size: 15px;

    align-items: center;
    display: none;
    height: $icon-size * 2;
    justify-content: center;
    position: absolute;
    right: -$icon-size;
    top: -$icon-size;
    width: $icon-size * 2;

    .remove-icon {
      background-color: var(--block-background-color);
      border-radius: 50%;
      height: $icon-size;
      width: $icon-size;

      /* stylelint-disable-next-line selector-max-compound-selectors */
      svg {
        height: inherit;
        stroke: var(--link-hover-color);
        width: inherit;
      }
    }
  }

  &:hover .remove-extra-field {
    display: flex;
  }

  &.error input {
    border: 1px solid $error-color;
  }
}

.add-extra-field {
  align-items: center;
  display: flex;

  svg {
    height: $icon-size;
    margin-right: 5px;
    stroke: var(--link-color);
    width: $icon-size;
  }

  &:hover svg {
    stroke: var(--link-hover-color);
  }
}

@media (hover: none) {
  .extra-field {
    .remove-extra-field {
      display: flex;
    }
  }
}
</style>
