import { ref } from "vue"

import { getNotificationMarker, updateNotificationMarker } from "@/api/markers"
import { Notification, getNotifications } from "@/api/notifications"

const notifications = ref<Notification[]>([])
const lastReadId = ref<string | null>(null)

export function useNotifications() {

  async function loadNotifications(authToken: string): Promise<void> {
    const items = await getNotifications(authToken)
    const marker = await getNotificationMarker(authToken)
    // Don't update reactive object until marker is loaded
    notifications.value = items
    if (marker) {
      lastReadId.value = marker.last_read_id
    }
  }

  function getUnreadNotificationCount(): number {
    let unreadCount = 0
    if (lastReadId.value) {
      for (const notification of notifications.value) {
        if (parseInt(notification.id) <= parseInt(lastReadId.value)) {
          break
        }
        unreadCount += 1
      }
    } else {
      unreadCount = notifications.value.length
    }
    return unreadCount
  }

  async function updateUnreadNotificationCount(authToken: string) {
    const firstNotification = notifications.value[0]
    if (
      firstNotification &&
      firstNotification.id !== lastReadId.value
    ) {
      await updateNotificationMarker(
        authToken,
        firstNotification.id,
      )
      lastReadId.value = firstNotification.id
    }
  }

  return {
    notifications,
    loadNotifications,
    getUnreadNotificationCount,
    updateUnreadNotificationCount,
  }
}
