import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router"

import { Permissions } from "@/api/users"
import { useInstanceInfo } from "@/composables/instance"
import { useCurrentUser } from "@/composables/user"

import AboutPage from "@/views/About.vue"
import EthereumPage from "@/views/Ethereum.vue"
import FollowRequestList from "@/views/FollowRequestList.vue"
import Gallery from "@/views/Gallery.vue"
import HomeTimeline from "@/views/HomeTimeline.vue"
import IdentityProof from "@/views/IdentityProof.vue"
import ImportFollowers from "@/views/ImportFollowers.vue"
import ImportFollows from "@/views/ImportFollows.vue"
import LandingPage from "@/views/LandingPage.vue"
import NotificationList from "@/views/NotificationList.vue"
import ProfileAliases from "@/views/ProfileAliases.vue"
import ProfileDirectory from "@/views/ProfileDirectory.vue"
import ProfileView from "@/views/Profile.vue"
import ProfileForm from "@/views/ProfileForm.vue"
import PostDetail from "@/views/PostDetail.vue"
import PublicTimeline from "@/views/PublicTimeline.vue"
import SettingsPage from "@/views/Settings.vue"
import TagTimeline from "@/views/TagTimeline.vue"
import SearchResultList from "@/views/SearchResultList.vue"
import SubscriberView from "@/views/Subscriber.vue"
import SubscriptionPage from "@/views/SubscriptionPage.vue"
import SubscriptionsSettings from "@/views/SubscriptionsSettings.vue"

async function authGuard(to: any) {
  const { isAuthenticated } = useCurrentUser()
  const isUserAuthenticated = await isAuthenticated()
  const onlyGuest = to.matched.some((record: RouteRecordRaw) => {
    return record.meta?.onlyGuest
  })
  const onlyAuthenticated = to.matched.some((record: RouteRecordRaw) => {
    return record.meta?.onlyAuthenticated
  })
  if (onlyGuest && isUserAuthenticated) {
    return { name: "home" }
  } else if (onlyAuthenticated && !isUserAuthenticated) {
    return { name: "landing-page" }
  }
}

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "landing-page",
    component: LandingPage,
    meta: { onlyGuest: true },
  },
  {
    path: "/about",
    name: "about",
    component: AboutPage,
    meta: { },
  },
  {
    path: "/ethereum",
    name: "ethereum",
    component: EthereumPage,
    meta: { onlyGuest: true },
  },
  {
    path: "/home",
    name: "home",
    component: HomeTimeline,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/local",
    name: "local",
    component: PublicTimeline,
    meta: { },
    beforeEnter: () => {
      const { currentUser } = useCurrentUser()
      const { instance } = useInstanceInfo()
      if (
        currentUser.value !== null ||
        instance.value?.allow_unauthenticated.timeline_local
      ) {
        return true
      }
      return { name: "landing-page" }
    },
  },
  {
    path: "/federated",
    name: "known-network",
    component: PublicTimeline,
    meta: { onlyAuthenticated: true },
    beforeEnter: () => {
      const { isAdmin } = useCurrentUser()
      const { instance } = useInstanceInfo()
      const federatedTimelineRestricted = instance.value?.federated_timeline_restricted ?? true
      if (!federatedTimelineRestricted || isAdmin()) {
        return true
      }
      return { name: "home" }
    },
  },
  {
    path: "/post/:postId",
    name: "post",
    component: PostDetail,
    meta: { },
  },
  {
    path: "/tag/:tagName",
    name: "tag",
    component: TagTimeline,
    meta: { },
  },
  {
    path: "/notifications",
    name: "notifications",
    component: NotificationList,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/profile/:profileId",
    name: "profile",
    component: ProfileView,
    meta: { },
  },
  {
    path: "/@:acct(.*)",
    name: "profile-by-acct",
    component: ProfileView,
    meta: { },
  },
  {
    path: "/profile/:profileId/subscription",
    name: "profile-subscription",
    component: SubscriptionPage,
    meta: { },
  },
  {
    path: "/@:acct(.*)/subscription",
    name: "profile-subscription-by-acct",
    component: SubscriptionPage,
    meta: { },
  },
  {
    path: "/profile/:profileId/gallery",
    name: "profile-gallery",
    component: Gallery,
    meta: { },
  },
  {
    path: "/@:acct(.*)/gallery",
    name: "profile-gallery-by-acct",
    component: Gallery,
    meta: { },
  },
  {
    path: "/subscriber/:profileId",
    name: "subscriber",
    component: SubscriberView,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/profile-directory",
    name: "profile-directory",
    component: ProfileDirectory,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/search",
    name: "search",
    component: SearchResultList,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/follow-requests",
    name: "follow-request-list",
    component: FollowRequestList,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/settings",
    name: "settings",
    component: SettingsPage,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/settings/profile",
    name: "settings-profile",
    component: ProfileForm,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/settings/aliases",
    name: "settings-aliases",
    component: ProfileAliases,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/settings/identity-proof",
    name: "identity-proof",
    component: IdentityProof,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/settings/import-followers",
    name: "import-followers",
    component: ImportFollowers,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/settings/import-follows",
    name: "import-follows",
    component: ImportFollows,
    meta: { onlyAuthenticated: true },
  },
  {
    path: "/subscriptions/settings",
    name: "subscriptions-settings",
    component: SubscriptionsSettings,
    meta: { onlyAuthenticated: true },
    beforeEnter: () => {
      const { ensureCurrentUser } = useCurrentUser()
      return ensureCurrentUser()
        .role.permissions
        .includes(Permissions.ManageSubscriptionOptions)
    },
  },
  {
    path: "/:pathMatch(.*)*",
    redirect: { name: "home" },
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach(authGuard)

export default router
