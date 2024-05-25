// Taken from https://github.com/jpkleemans/vite-svg-loader
// Removed svgo dependency
import { readFile } from "fs/promises"
import { compileTemplate } from "vue/compiler-sfc"

export default function svgLoader() {
  const svgRegex = /\.svg\?component$/

  return {
    name: "svg-loader",
    enforce: "pre",

    async load(id: string) {
      if (!id.match(svgRegex)) {
        return
      }

      const [path] = id.split("?")

      let svg = await readFile(path, "utf-8")

      // To prevent compileTemplate from removing the style tag
      svg = svg.replace(/<style/g, '<component is="style"').replace(/<\/style/g, "</component")

      const { code } = compileTemplate({
        id: JSON.stringify(id),
        source: svg,
        filename: path,
        transformAssetUrls: false,
      })

      return `${code}\nexport default { render: render }`
    },
  }
}
