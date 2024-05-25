export function resizeTextArea(textarea: HTMLTextAreaElement) {
  textarea.style.height = "0px"
  textarea.style.height = `${textarea.scrollHeight}px`
}

export function setupAutoResize(textarea: HTMLTextAreaElement) {
  textarea.style.minHeight = `${textarea.offsetHeight}px`
  textarea.style.overflowY = "hidden"
  textarea.addEventListener("input", () => {
    resizeTextArea(textarea)
  }, false)
}
