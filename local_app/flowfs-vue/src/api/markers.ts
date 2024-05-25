import { BACKEND_URL } from "@/constants"

import { handleResponse, http } from "./common"

export interface Marker {
  last_read_id: string;
  version: number;
  updated_at: string;
}

export async function getNotificationMarker(
  authToken: string,
): Promise<Marker | null> {
  const url = `${BACKEND_URL}/api/v1/markers`
  const response = await http(url, {
    method: "GET",
    queryParams: { "timeline[]": "notifications" },
    authToken,
  })
  const data = await handleResponse(response)
  return data.notifications
}

export async function updateNotificationMarker(
  authToken: string,
  lastReadId: string,
): Promise<Marker> {
  const url = `${BACKEND_URL}/api/v1/markers`
  const response = await http(url, {
    method: "POST",
    json: { "notifications[last_read_id]": lastReadId },
    authToken,
  })
  const data = await handleResponse(response)
  return data.notifications
}
