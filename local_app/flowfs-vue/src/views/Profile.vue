<template>
  <sidebar-layout>
    <template #content>
      <div class="not-found" v-if="!profile && !isLoading">
        Profile not found
      </div>
      <div
        class="profile-block"
        v-if="profile"
        :data-profile-id="profile.id"
      >
        <div class="profile-header">
          <img v-if="profile.header" :src="profile.header">
        </div>
        <div class="profile-info-group">
          <div class="avatar-menu-group">
            <div class="avatar-group">
              <avatar :profile="profile"></avatar>
              <div class="badges">
                <div class="badge" v-if="isAdminProfile()">Admin</div>
                <div class="badge" v-if="profile.bot">Automated</div>
                <div
                  class="badge"
                  v-if="aliases.length > 0"
                  :title="aliases.map(profile => profile.url).join(', ')"
                >
                  Alias
                </div>
                <div class="badge" v-if="isFollowedBy()">Follows you</div>
                <div class="badge" v-if="isSubscriptionValid()">Subscription</div>
                <div class="badge" v-if="isSubscriber()">Subscriber</div>
                <div class="badge" v-if="isMuted()">Muted</div>
              </div>
            </div>
            <div
              class="dropdown-menu-wrapper"
              v-click-away="hideProfileMenu"
            >
              <button title="More" @click="toggleProfileMenu()">
                <icon-more></icon-more>
              </button>
              <menu v-if="profileMenuVisible" class="dropdown-menu">
                <li v-if="!isLocalUser()">
                  <a
                    title="Open profile page"
                    :href="profile.url"
                    target="_blank"
                    rel="noreferrer"
                    @click="hideProfileMenu()"
                  >
                    Open profile page
                  </a>
                </li>
                <li v-if="isLocalUser()">
                  <a
                    :href="feedUrl"
                    target="_blank"
                  >
                    Atom feed
                  </a>
                </li>
                <li v-if="isCurrentUser()">
                  <router-link
                    title="View follow requests"
                    :to="{ name: 'follow-request-list' }"
                  >
                    View follow requests
                  </router-link>
                </li>
                <li v-if="canVerifyEthereumAddress()">
                  <button
                    title="Link ethereum address"
                    @click="hideProfileMenu(); onVerifyEthereumAddress()"
                  >
                    Link ethereum address
                  </button>
                </li>
                <li v-if="isCurrentUser()">
                  <router-link
                    title="Link minisign key"
                    :to="{ name: 'identity-proof' }"
                  >
                    Link minisign key
                  </router-link>
                </li>
                <li v-if="canVerifyEthereumAddress()">
                  <button
                    title="Send signed update"
                    @click="hideProfileMenu(); onSignActivity()"
                  >
                    Send signed update
                  </button>
                </li>
                <li v-if="canViewSubscriber()">
                  <router-link
                    :to="{ name: 'subscriber', params: { profileId: profile.id } }"
                  >
                    Subscriber details
                  </router-link>
                </li>
                <li v-if="canHideReposts()">
                  <button @click="onFollow(false, undefined)">Hide reposts</button>
                </li>
                <li v-if="canShowReposts()">
                  <button @click="onFollow(true, undefined)">Show reposts</button>
                </li>
                <li v-if="canHideReplies()">
                  <button @click="onFollow(undefined, false)">Hide replies</button>
                </li>
                <li v-if="canShowReplies()">
                  <button @click="onFollow(undefined, true)">Show replies</button>
                </li>
                <li v-if="isFollowedBy()">
                  <button @click="onRemoveFollower()">Remove from followers</button>
                </li>
                <li v-if="canMute()">
                  <button @click="onMute()">Mute</button>
                </li>
                <li v-if="canUnmute()">
                  <button @click="onUnmute()">Unmute</button>
                </li>
                <li v-if="isAdmin()">
                  <button
                    title="Copy profile ID"
                    @click="hideProfileMenu(); copyProfileId()"
                  >
                    Copy profile ID
                  </button>
                </li>
                <li v-if="isAdmin()">
                  <button
                    title="Copy actor ID"
                    @click="hideProfileMenu(); copyActorId()"
                  >
                    Copy actor ID
                  </button>
                </li>
                <li v-if="canLoadLatestPosts()">
                  <button @click="hideProfileMenu(); onLoadLatestPosts()">
                    Load latest posts
                  </button>
                </li>
              </menu>
            </div>
          </div>
          <div class="name-buttons-group">
            <div class="name-group">
              <profile-display-name :profile="profile">
              </profile-display-name>
              <div class="actor-address">{{ actorHandle }}</div>
            </div>
            <div class="buttons">
              <router-link
                v-if="isCurrentUser()"
                class="edit-profile btn"
                :to="{ name: 'settings-profile' }"
              >
                Edit profile
              </router-link>
              <button v-if="canAcceptFollowRequest()" class="btn" @click="onAcceptFollowRequest()">
                Accept follow request
              </button>
              <button v-if="canAcceptFollowRequest()" class="btn" @click="onRejectFollowRequest()">
                Reject follow request
              </button>
              <button
                v-if="canFollow()"
                class="btn follow-btn"
                :title="profile.locked ? 'Manually approves followers' : undefined"
                :disabled="isProcessingFollow"
                @click="onFollow()"
              >
                <span>Follow</span>
                <icon-lock v-if="profile.locked"></icon-lock>
              </button>
              <button
                v-if="canUnfollow()"
                class="btn unfollow-btn"
                :disabled="isProcessingUnfollow"
                @click="onUnfollow()"
              >
                <template v-if="isFollowRequestPending()">Cancel follow request</template>
                <template v-else>Unfollow</template>
              </button>
              <universal-link
                v-if="subscriptionPageLocation && canSubscribe()"
                :to="subscriptionPageLocation"
                title="Become a subscriber"
                class="btn"
              >
                <template #link-content>Subscribe</template>
              </universal-link>
            </div>
          </div>
          <div class="bio" v-html="profile.note"></div>
          <div class="extra-fields" v-if="fields.length > 0">
            <div
              v-for="field in fields"
              class="field"
              :class="{ verified: field.verified_at, legacy: field.is_legacy_proof && isCurrentUser() }"
              :key="field.name"
            >
              <div class="name" :title="field.name">{{ field.name }}</div>
              <div class="value" v-html="field.value"></div>
              <template v-if="field.verified_at">
                <a
                  class="verified-icon"
                  v-if="field.is_legacy_proof && isCurrentUser()"
                  title="Re-verify"
                  @click="updateIdentityProof(field.name)"
                >
                  <icon-refresh></icon-refresh>
                </a>
                <div
                  class="verified-icon"
                  v-else
                  title="Verified"
                >
                  <icon-check></icon-check>
                </div>
              </template>
            </div>
          </div>
          <div v-if="isLocalUser()" class="stats">
            <component
              class="stats-item"
              :is="isCurrentUser() ? 'a' : 'span'"
              @click="isCurrentUser() && switchTab('posts')"
            >
              <span class="value">{{ profile.statuses_count }}</span>
              <span class="label">posts</span>
            </component>
            <component
              class="stats-item"
              :is="isCurrentUser() ? 'a' : 'span'"
              @click="isCurrentUser() && switchTab('followers')"
            >
              <span class="value">{{ profile.followers_count }}</span>
              <span class="label">followers</span>
            </component>
            <component
              class="stats-item"
              :is="isCurrentUser() ? 'a' : 'span'"
              @click="isCurrentUser() && switchTab('following')"
            >
              <span class="value">{{ profile.following_count }}</span>
              <span class="label">following</span>
            </component>
            <component
              class="stats-item"
              v-if="isSubscriptionsFeatureEnabled()"
              :is="isCurrentUser() ? 'a' : 'span'"
              @click="isCurrentUser() && switchTab('subscribers')"
            >
              <span class="value">{{ profile.subscribers_count }}</span>
              <span class="label">subscribers</span>
            </component>
          </div>
        </div>
      </div>
      <div class="tab-bar" v-if="profile">
        <template v-if="tabName === 'posts' || tabName === 'posts-with-replies' || tabName === 'posts-featured'">
          <a
            class="tab"
            :class="{ active: tabName === 'posts' }"
            @click="switchTab('posts')"
          >
            Posts
          </a>
          <a
            class="tab"
            :class="{ active: tabName === 'posts-with-replies' }"
            @click="switchTab('posts-with-replies')"
          >
            Posts and replies
          </a>
          <a
            class="tab"
            :class="{ active: tabName === 'posts-featured' }"
            @click="switchTab('posts-featured')"
          >
            Featured
          </a>
          <router-link class="tab" :to="getActorLocation('profile-gallery', profile)">
            Gallery
          </router-link>
        </template>
        <span v-else-if="tabName === 'followers'" class="tab active">
          Followers
        </span>
        <span v-else-if="tabName === 'following'" class="tab active">
          Following
        </span>
        <span v-else-if="tabName === 'subscribers'" class="tab active">
          Subscribers
        </span>
      </div>
      <loader v-if="isLoading"></loader>
      <div
        v-if="profile"
        :style="{ visibility: isLoading ? 'hidden' : 'visible' }"
      >
        <template v-if="tabName === 'posts' || tabName === 'posts-with-replies' || tabName === 'posts-featured'">
          <div v-if="posts.length === 0" class="empty-list">
            No posts found
          </div>
          <post-list
            ref="postListRef"
            :posts="posts"
            @load-next-page="loadNextPage"
          ></post-list>
        </template>
        <template v-else-if="tabName === 'followers' || tabName === 'following'">
          <router-link
            class="profile-list-item"
            v-for="profile in followList"
            :key="profile.id"
            :to="getActorLocation('profile', profile)"
          >
            <profile-list-item :profile="profile"></profile-list-item>
          </router-link>
          <button
            v-if="followListNextPageUrl"
            class="btn secondary next-btn"
            @click="loadFollowListNextPage()"
          >
            Show more profiles
          </button>
        </template>
        <template v-else-if="tabName === 'subscribers'">
          <router-link
            class="profile-list-item"
            v-for="subscription in subscriptions"
            :key="subscription.id"
            :to="{ name: 'subscriber', params: { profileId: subscription.sender.id } }"
          >
            <profile-list-item :profile="subscription.sender">
              <template #profile-footer>
                <div class="subscription-info">
                  Subscription expires {{ formatDate(subscription.expires_at) }}
                </div>
              </template>
            </profile-list-item>
          </router-link>
        </template>
      </div>
    </template>
  </sidebar-layout>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue"
import { $, $ref } from "vue/macros"
import { useRoute, useRouter } from "vue-router"

import { Post, getProfileTimeline } from "@/api/posts"
import {
  acceptFollowRequest,
  rejectFollowRequest,
  follow,
  unfollow,
  mute,
  unmute,
  Relationship,
  getRelationship,
  getFollowers,
  getFollowing,
  removeFollower,
} from "@/api/relationships"
import { getReceivedSubscriptions, Subscription } from "@/api/subscriptions-common"
import {
  getAliases,
  getProfile,
  loadLatestPosts,
  lookupProfile,
  Profile,
  ProfileField,
  ProfileWrapper,
  EXTRA_FIELD_COUNT_MAX,
} from "@/api/users"
import IconMore from "@/assets/feather/more-vertical.svg?component"
import IconCheck from "@/assets/forkawesome/check.svg?component"
import IconLock from "@/assets/forkawesome/lock.svg?component"
import IconRefresh from "@/assets/forkawesome/refresh.svg?component"
import Avatar from "@/components/Avatar.vue"
import Loader from "@/components/Loader.vue"
import PostList from "@/components/PostList.vue"
import ProfileDisplayName from "@/components/ProfileDisplayName.vue"
import ProfileListItem from "@/components/ProfileListItem.vue"
import SidebarLayout from "@/components/SidebarLayout.vue"
import UniversalLink from "@/components/UniversalLink.vue"
import { useEthereumAddressVerification } from "@/composables/ethereum-address-verification"
import { useActorHandle } from "@/composables/handle"
import { useInstanceInfo } from "@/composables/instance"
import { useSignedActivity } from "@/composables/signed-activity"
import { useSubscribe } from "@/composables/subscribe"
import { useCurrentUser } from "@/composables/user"
import { BACKEND_URL } from "@/constants"
import { formatDate } from "@/utils/dates"
import { hasEthereumWallet } from "@/utils/ethereum"

const route = useRoute()
const router = useRouter()
const {
  authToken,
  currentUser,
  ensureAuthToken,
  isAdmin,
} = $(useCurrentUser())
const { verifyEthereumAddress } = useEthereumAddressVerification()
const { getActorHandle, getActorLocation } = useActorHandle()
const { getBlockchainInfo } = useInstanceInfo()
const { getSubscriptionLink, getSubscriptionOption } = useSubscribe()

const postListRef = $ref<InstanceType<typeof PostList> | null>(null)

let profile = $ref<ProfileWrapper | null>(null)
let relationship = $ref<Relationship | null>(null)
let aliases = $ref<Profile[]>([])

let profileMenuVisible = $ref(false)
const isProcessingFollow = ref(false)
const isProcessingUnfollow = ref(false)

let tabName = $ref("posts")
let isLoading = $ref(false)
let posts = $ref<Post[]>([])
let followList = $ref<Profile[]>([])
let followListNextPageUrl = $ref<string | null>(null)
let subscriptions = $ref<Subscription[]>([])

onMounted(async () => {
  isLoading = true
  try {
    let _profile
    if (route.params.acct) {
      _profile = await lookupProfile(
        authToken,
        route.params.acct as string,
      )
    } else {
      _profile = await getProfile(
        authToken,
        route.params.profileId as string,
      )
    }
    profile = new ProfileWrapper(_profile)
  } catch (error: any) {
    if (error.message === "profile not found") {
      // Show "not found" text
      isLoading = false
      return
    }
    throw error
  }
  if (currentUser && !isCurrentUser()) {
    relationship = await getRelationship(
      ensureAuthToken(),
      profile.id,
    )
  }
  if (profile.identity_proofs.length > 0) {
    const { verified } = await getAliases(profile.id)
    aliases = verified
  }
  await switchTab("posts")
  isLoading = false
})

async function switchTab(name: string) {
  if (!profile) {
    return
  }
  isLoading = true
  tabName = name
  if (postListRef !== null) {
    postListRef.resetPagination()
  }
  if (tabName === "posts") {
    posts = await getProfileTimeline(
      authToken,
      profile.id,
      true,
      false,
      false,
    )
  } else if (tabName === "posts-with-replies") {
    posts = await getProfileTimeline(
      authToken,
      profile.id,
      false,
      false,
      false,
    )
  } else if (tabName === "posts-featured") {
    posts = await getProfileTimeline(
      authToken,
      profile.id,
      false,
      true,
      false,
    )
  } else if (tabName === "followers" && isCurrentUser()) {
    const page = await getFollowers(
      ensureAuthToken(),
      profile.id,
    )
    followList = page.profiles
    followListNextPageUrl = page.nextPageUrl
  } else if (tabName === "following" && isCurrentUser()) {
    const page = await getFollowing(
      ensureAuthToken(),
      profile.id,
    )
    followList = page.profiles
    followListNextPageUrl = page.nextPageUrl
  } else if (tabName === "subscribers" && isCurrentUser()) {
    subscriptions = await getReceivedSubscriptions(
      ensureAuthToken(),
      profile.id,
      false,
    )
  }
  isLoading = false
}

const actorHandle = computed<string>(() => {
  if (!profile) {
    return ""
  }
  return getActorHandle(profile)
})

const fields = computed<ProfileField[]>(() => {
  if (!profile) {
    return []
  }
  return profile.identity_proofs
    .concat(profile.fields)
    .slice(0, EXTRA_FIELD_COUNT_MAX)
})

function isCurrentUser(): boolean {
  if (!currentUser || !profile) {
    return false
  }
  return currentUser.id === profile.id
}

function isAdminProfile(): boolean {
  return isCurrentUser() && isAdmin()
}

function isFollowedBy(): boolean {
  if (!relationship) {
    return false
  }
  return relationship.followed_by
}

function isSubscriptionValid(): boolean {
  if (!relationship) {
    return false
  }
  return relationship.subscription_to
}

function isSubscriber(): boolean {
  if (!relationship) {
    return false
  }
  return relationship.subscription_from
}

function isMuted(): boolean {
  if (!relationship) {
    return false
  }
  return relationship.muting
}

function canAcceptFollowRequest(): boolean {
  if (!relationship) {
    return false
  }
  return relationship.requested_by
}

async function onAcceptFollowRequest() {
  if (!profile) {
    return
  }
  relationship = await acceptFollowRequest(
    ensureAuthToken(),
    profile.id,
  )
}

async function onRejectFollowRequest() {
  if (!profile) {
    return
  }
  relationship = await rejectFollowRequest(
    ensureAuthToken(),
    profile.id,
  )
}

function canFollow(): boolean {
  if (currentUser === null) {
    // Show 'Follow' button to guests
    return true
  }
  if (!relationship) {
    return false
  }
  return !relationship.following && !relationship.requested
}

function canUnfollow(): boolean {
  if (!relationship) {
    return false
  }
  return (relationship.following || relationship.requested)
}

function isFollowRequestPending(): boolean {
  if (!relationship) {
    return false
  }
  return relationship.requested
}

function canHideReposts(): boolean {
  if (!relationship) {
    return false
  }
  return (relationship.following || relationship.requested) && relationship.showing_reblogs
}

function canShowReposts(): boolean {
  if (!relationship) {
    return false
  }
  return (relationship.following || relationship.requested) && !relationship.showing_reblogs
}

function canHideReplies(): boolean {
  if (!relationship) {
    return false
  }
  return (relationship.following || relationship.requested) && relationship.showing_replies
}

function canShowReplies(): boolean {
  if (!relationship) {
    return false
  }
  return (relationship.following || relationship.requested) && !relationship.showing_replies
}

async function onFollow(showReposts?: boolean, showReplies?: boolean) {
  if (!currentUser) {
    // Viewing as guest
    alert(`You can follow this account from your Fediverse server: ${actorHandle.value}`)
    return
  }
  if (!profile || !relationship) {
    return
  }
  isProcessingFollow.value = true
  relationship = await follow(
    ensureAuthToken(),
    profile.id,
    showReposts ?? relationship.showing_reblogs,
    showReplies ?? relationship.showing_replies,
  )
  isProcessingFollow.value = false
  if (
    showReposts === undefined &&
    showReplies === undefined &&
    !relationship.following
  ) {
    // Update follower status after 5 secs
    let count = 0
    const intervalId = setInterval(async () => {
      if (profile && relationship && !relationship.following && count < 5) {
        relationship = await getRelationship(
          ensureAuthToken(),
          profile.id,
        )
        count += 1
      } else {
        clearInterval(intervalId)
      }
    }, 5000)
  }
}

async function onUnfollow() {
  if (!currentUser || !profile) {
    return
  }
  isProcessingUnfollow.value = true
  relationship = await unfollow(
    ensureAuthToken(),
    profile.id,
  )
  isProcessingUnfollow.value = false
}

async function onRemoveFollower() {
  if (!currentUser || !profile) {
    return
  }
  if (confirm(`Are you sure you want to remove ${profile.getDisplayName()} from followers?`)) {
    relationship = await removeFollower(
      ensureAuthToken(),
      profile.id,
    )
  }
}

function canMute(): boolean {
  if (!relationship) {
    return false
  }
  return !relationship.muting
}

function canUnmute(): boolean {
  if (!relationship) {
    return false
  }
  return relationship.muting
}

async function onMute() {
  if (!currentUser || !profile) {
    return
  }
  relationship = await mute(
    ensureAuthToken(),
    profile.id,
  )
}

async function onUnmute() {
  if (!currentUser || !profile) {
    return
  }
  relationship = await unmute(
    ensureAuthToken(),
    profile.id,
  )
}

function toggleProfileMenu() {
  profileMenuVisible = !profileMenuVisible
}

function hideProfileMenu() {
  profileMenuVisible = false
}

function isLocalUser(): boolean {
  if (!profile) {
    return false
  }
  return profile.username === profile.acct
}

const feedUrl = computed<string>(() => {
  if (!profile || !isLocalUser()) {
    return ""
  }
  return `${BACKEND_URL}/feeds/users/${profile.username}`
})

function canVerifyEthereumAddress(): boolean {
  return isCurrentUser() && hasEthereumWallet()
}

async function onVerifyEthereumAddress() {
  if (!profile || !isCurrentUser()) {
    return
  }
  const user = await verifyEthereumAddress()
  if (user) {
    profile.identity_proofs = user.identity_proofs
  }
}

async function onSignActivity() {
  if (!profile || !isCurrentUser()) {
    return
  }
  const { signUpdateActivity } = useSignedActivity()
  await signUpdateActivity()
}

function isSubscriptionsFeatureEnabled(): boolean {
  const blockchain = getBlockchainInfo()
  return Boolean(blockchain?.features.subscriptions)
}

const subscriptionPageLocation = computed(() => {
  if (!profile) {
    return null
  }
  const link = getSubscriptionLink(profile)
  return link?.location || null
})

function canSubscribe(): boolean {
  return (
    subscriptionPageLocation.value !== null &&
    !isCurrentUser()
  )
}

function canViewSubscriber(): boolean {
  return (
    currentUser !== null &&
    !isCurrentUser() &&
    getSubscriptionOption(currentUser) !== null
  )
}

function copyProfileId(): void {
  if (!profile) {
    return
  }
  navigator.clipboard.writeText(profile.id)
}

function copyActorId(): void {
  if (!profile) {
    return
  }
  navigator.clipboard.writeText(profile.actor_id)
}

function canLoadLatestPosts(): boolean {
  return (
    profile !== null &&
    currentUser !== null &&
    !isLocalUser() &&
    isAdmin()
  )
}

async function onLoadLatestPosts() {
  if (!profile) {
    return
  }
  alert("Reload the page in a few minutes")
  await loadLatestPosts(
    ensureAuthToken(),
    profile.id,
  )
}

async function updateIdentityProof(fieldName: string) {
  if (fieldName === "$ETH") {
    if (!canVerifyEthereumAddress()) {
      alert("Ethereum wallet is not detected")
      return
    }
    await onVerifyEthereumAddress()
  } else if (fieldName === "Key") {
    router.push({ name: "identity-proof" })
  }
}

async function loadNextPage(maxId: string) {
  if (!profile) {
    return
  }
  const nextPage = await getProfileTimeline(
    authToken,
    profile.id,
    tabName !== "posts-with-replies",
    tabName === "posts-featured",
    false,
    maxId,
  )
  posts = [...posts, ...nextPage]
}

async function loadFollowListNextPage() {
  if (!profile || !followListNextPageUrl) {
    return
  }
  let loadFollowList
  if (tabName === "followers") {
    loadFollowList = getFollowers
  } else if (tabName === "following") {
    loadFollowList = getFollowing
  } else {
    throw new Error("wrong tab")
  }
  const page = await loadFollowList(
    ensureAuthToken(),
    profile.id,
    followListNextPageUrl,
  )
  followList.push(...page.profiles)
  followListNextPageUrl = page.nextPageUrl
}
</script>

<style scoped lang="scss">
@import "../styles/layout";
@import "../styles/mixins";
@import "../styles/theme";

$avatar-size: 170px;

.profile-block {
  @include block-btn;

  background-color: var(--block-background-color);
  border-radius: $block-border-radius;
  margin-bottom: $block-outer-padding;

  .profile-header {
    background-color: var(--btn-background-color);
    border-radius: $block-border-radius $block-border-radius 0 0;
    height: 200px;

    img {
      border-radius: inherit;
      height: 100%;
      object-fit: cover;
      width: 100%;
    }
  }
}

.profile-info-group {
  display: flex;
  flex-direction: column;
  gap: $block-inner-padding;
  padding: $block-inner-padding;
}

.avatar-menu-group {
  display: flex;
  flex-direction: row;
  gap: $block-inner-padding;

  .avatar-group {
    align-items: flex-start;
    display: flex;
    flex-direction: row;
    flex-grow: 1;
    flex-wrap: wrap;
    gap: calc($block-inner-padding / 2) $block-inner-padding;
  }

  .avatar {
    height: $avatar-size;
    margin-right: auto;
    margin-top: calc(-1 * ($avatar-size / 2 + $block-inner-padding));
    min-width: $avatar-size;
    padding: 7px;
    width: $avatar-size;
  }

  .badges {
    display: flex;
    flex-wrap: wrap;
    gap: calc($block-inner-padding / 2) $block-inner-padding;
  }

  .badge {
    border: 1px solid var(--btn-background-color);
    border-radius: $btn-border-radius;
    font-size: 14px;
    line-height: 30px;
    padding: 0 7px;
    white-space: nowrap;
  }

  .dropdown-menu-wrapper {
    @include block-dropdown-menu;

    /* stylelint-disable-next-line selector-max-compound-selectors */
    button svg {
      height: 32px;
      min-width: 20px;
      object-fit: none;
      stroke: var(--link-color);
      width: 20px;

      /* stylelint-disable-next-line selector-max-compound-selectors */
      &:hover {
        stroke: var(--link-hover-color);
      }
    }

    menu {
      right: 0;
    }
  }
}

.name-buttons-group {
  align-items: center;
  display: flex;
  flex-wrap: wrap;
  gap: $block-inner-padding;

  .name-group {
    flex-grow: 1;
    font-size: 16px;
    line-height: 1.3;
    overflow-x: hidden;

    .display-name {
      font-weight: bold;
    }

    .actor-address {
      color: var(--secondary-text-color);
      overflow-x: hidden;
      text-overflow: ellipsis;
      user-select: all;
    }
  }

  .buttons {
    display: flex;
    flex-wrap: wrap;
    gap: $block-inner-padding;
    max-width: 100%;
  }
}

.follow-btn {
  align-items: center;
  display: flex;
  gap: $input-padding;

  svg {
    $icon-size: 1em;

    fill: var(--btn-text-color);
    height: $icon-size;
    min-width: $icon-size;
    width: $icon-size;
  }

  &:hover {
    svg {
      fill: var(--text-color);
    }
  }

  &[disabled] {
    svg {
      fill: var(--btn-disabled-text-color) !important;
    }
  }
}

.bio {
  white-space: pre-line;
  word-wrap: break-word;

  :deep(a) {
    @include block-external-link;
  }

  :deep(ul),
  :deep(ol) {
    list-style-position: inside;
  }
}

.extra-fields {
  border-bottom: 1px solid var(--separator-color);

  .field {
    border-top: 1px solid var(--separator-color);
    display: flex;
    gap: calc($block-inner-padding / 2);
    padding: calc($block-inner-padding / 2) 0;

    .name {
      font-weight: bold;
      min-width: 120px;
      overflow-x: hidden;
      text-overflow: ellipsis;
      width: 120px;
    }

    .value {
      flex-grow: 1;
      overflow-x: hidden;
      text-overflow: ellipsis;
    }

    &.verified {
      font-weight: bold;
    }

    /* stylelint-disable-next-line selector-max-compound-selectors */
    .verified-icon svg {
      fill: var(--text-color);
      height: 1em;
      min-width: 1em;
      width: 1em;
    }

    &.legacy {
      color: var(--secondary-text-color);

      /* stylelint-disable-next-line selector-max-compound-selectors */
      .verified-icon svg {
        fill: var(--secondary-text-color);
      }
    }
  }

  &:last-child {
    border-bottom: none;
  }
}

.stats {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  font-weight: bold;
  gap: $block-inner-padding 30px;
  text-align: center;

  .stats-item {
    align-items: baseline;
    display: flex;
    gap: 5px;

    .value {
      font-size: 18px;
    }

    .label {
      color: var(--secondary-text-color);
    }
  }
}

.tab-bar {
  align-items: center;
  display: flex;
  flex-wrap: wrap;
  margin-bottom: $block-outer-padding;

  .tab {
    /* same styles used in content-list-header mixin */

    border-radius: $block-border-radius;
    box-sizing: border-box;
    flex-grow: 1;
    padding: calc($block-inner-padding / 2);
    text-align: center;

    &.active {
      background-color: var(--block-background-color);
      font-weight: bold;
    }
  }
}

.profile-list-item {
  display: block;
  margin-bottom: $block-outer-padding;
}

.subscription-info {
  margin-top: $block-inner-padding;
}

.loader {
  margin: 0 auto;
}

.not-found {
  @include content-message;
}

.empty-list {
  @include content-message;

  background-color: transparent;
  text-align: center;
}

@media screen and (max-width: $screen-breakpoint-small) {
  .tab {
    flex-basis: 50%;
  }
}
</style>
