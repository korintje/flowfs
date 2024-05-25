const DATA_URL_REGEXP = /^data:(\w+\/[-+.\w]+);base64,([a-zA-Z0-9+/]+={0,2})$/

export async function fileToDataUrl(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.readAsDataURL(file)
    reader.onload = () => {
      const result = reader.result as string
      resolve(result)
    }
    reader.onerror = error => reject(error)
  })
}

export function dataUrlToBase64(dataUrl: string): { mediaType: string, data: string } {
  const match = dataUrl.trim().match(DATA_URL_REGEXP)
  if (!match) {
    throw new Error(`invalid data URL ${dataUrl}`)
  }
  return { mediaType: match[1], data: match[2] }
}
