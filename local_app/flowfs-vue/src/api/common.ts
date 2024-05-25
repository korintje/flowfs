import { ENV } from "@/constants"

export const PAGE_SIZE = 20

// Wrapped in object for easy stubbing in tests
export const fetcher = {
  async fetch(url: string, params: RequestInit): Promise<Response> {
    return await window.fetch(url, params)
  },
}

interface RequestInfo extends RequestInit {
  authToken?: string | null;
  json?: any;
  queryParams?: { [name: string]: string | number | boolean | undefined };
}

export async function http(
  url: string | URL,
  requestInfo?: RequestInfo,
): Promise<Response> {
  const defaults: RequestInit = {}
  if (ENV === "development") {
    // Development mode
    defaults.credentials = "include"
  } else {
    defaults.credentials = "same-origin"
  }

  let params: RequestInit
  if (!requestInfo) {
    params = { ...defaults }
  } else {
    const { authToken, json, queryParams, ...requestParams } = { ...requestInfo }
    if (authToken) {
      requestParams.headers = {
        ...requestParams.headers,
        Authorization: `Bearer ${authToken}`,
      }
    }
    if (json) {
      requestParams.body = JSON.stringify(json)
      requestParams.headers = {
        ...requestParams.headers,
        "Content-Type": "application/json",
      }
    }
    if (queryParams) {
      if (!(url instanceof URL)) {
        // Convert URL string to URL object
        url = new URL(url, window.location.origin)
      }
      // Serialize query params
      const serialized = Object.keys(queryParams).reduce((res: { [name: string]: string }, key) => {
        const value = queryParams[key]
        if (value !== undefined) {
          res[key] = value.toString()
        }
        return res
      }, {})
      url.search = new URLSearchParams(serialized).toString()
    }
    params = { ...defaults, ...requestParams }
  }

  const response = await fetcher.fetch(url as string, params)
  return response
}

export async function handleResponse(
  response: Response,
  expectedStatus: number = 200,
): Promise<any> {
  if (response.status === expectedStatus) {
    if (expectedStatus === 204) {
      // No data
      return null
    } else {
      const data = await response.json()
      return data
    }
  } else {
    let errorDescription
    if (response.headers.get("Content-Type") === "application/json") {
      const data = await response.json()
      errorDescription = data.error_description
    } else {
      // Unexpected response
      errorDescription = response.statusText
    }
    throw new Error(errorDescription)
  }
}
