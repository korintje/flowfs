import { APP_NAME, BACKEND_URL } from "@/constants"
import { handleResponse, http } from "./common"
import { Aliases, ClientConfigValue, User } from "./users"

export async function updateClientConfig(
  authToken: string,
  clientConfig: { [property: string]: ClientConfigValue },
): Promise<User> {
  const url = `${BACKEND_URL}/api/v1/settings/client_config`
  const response = await http(url, {
    method: "POST",
    json: { [APP_NAME]: clientConfig },
    authToken,
  })
  const data = await handleResponse(response)
  return data
}

export async function changePassword(
  authToken: string,
  newPassword: string,
): Promise<User> {
  const url = `${BACKEND_URL}/api/v1/settings/change_password`
  const response = await http(url, {
    method: "POST",
    json: { new_password: newPassword },
    authToken,
  })
  const data = await handleResponse(response)
  return data
}

export async function addAlias(
  authToken: string,
  acct: string,
): Promise<Aliases> {
  const url = `${BACKEND_URL}/api/v1/settings/aliases`
  const response = await http(url, {
    method: "POST",
    json: { acct: acct },
    authToken,
  })
  const data = await handleResponse(response)
  return data
}

export async function removeAlias(
  authToken: string,
  actorId: string,
): Promise<Aliases> {
  const url = `${BACKEND_URL}/api/v1/settings/aliases/remove`
  const response = await http(url, {
    method: "POST",
    json: { actor_id: actorId },
    authToken,
  })
  const data = await handleResponse(response)
  return data
}

async function downloadBlob(blob: Blob, name: string) {
  const fileUrl = window.URL.createObjectURL(blob)
  const hiddenLink = document.createElement("a")
  hiddenLink.setAttribute("href", fileUrl)
  hiddenLink.setAttribute("download", name)
  hiddenLink.click()
  window.URL.revokeObjectURL(fileUrl)
}

export async function exportFollowers(
  authToken: string,
): Promise<void> {
  const url = `${BACKEND_URL}/api/v1/settings/export_followers`
  const response = await http(url, {
    method: "GET",
    authToken,
  })
  const blob = await response.blob()
  downloadBlob(blob, "followers.csv")
}

export async function exportFollows(
  authToken: string,
): Promise<void> {
  const url = `${BACKEND_URL}/api/v1/settings/export_follows`
  const response = await http(url, {
    method: "GET",
    authToken,
  })
  const blob = await response.blob()
  downloadBlob(blob, "follows.csv")
}

export async function importFollows(
  authToken: string,
  followsCsv: string,
): Promise<void> {
  const url = `${BACKEND_URL}/api/v1/settings/import_follows`
  const response = await http(url, {
    method: "POST",
    authToken,
    json: { follows_csv: followsCsv },
  })
  await handleResponse(response, 204)
}

export async function importFollowers(
  authToken: string,
  fromActorId: string,
  followersCsv: string,
): Promise<User> {
  const url = `${BACKEND_URL}/api/v1/settings/import_followers`
  const response = await http(url, {
    method: "POST",
    authToken,
    json: { from_actor_id: fromActorId, followers_csv: followersCsv },
  })
  const data = await handleResponse(response)
  return data
}

export async function deleteAccount(
  authToken: string,
): Promise<void> {
  const url = `${BACKEND_URL}/api/v1/settings/delete_account`
  const response = await http(url, {
    method: "POST",
    authToken,
  })
  await handleResponse(response, 204)
}
