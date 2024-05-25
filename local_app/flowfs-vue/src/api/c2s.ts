import { BACKEND_URL } from "@/constants"
import { handleResponse, http } from "./common"

export async function sendSignedActivity(
  username: string,
  activityValue: { [key: string]: any },
): Promise<void> {
  const url = `${BACKEND_URL}/users/${username}/outbox`
  const response = await http(url, {
    method: "POST",
    json: activityValue,
  })
  await handleResponse(response, 202)
}
