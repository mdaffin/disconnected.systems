const path = require('path')
const fs = require('fs-extra')

module.exports = async function ({options}) {
  const redirects = options.siteData.pages
    .filter(page => page.frontmatter.aliases)
    .map(page => page.frontmatter.aliases.map(alias => ({
        dest: page.path,
        alias: alias
      })))
    .reduce((acc, aliases) => acc.concat(aliases), [])
    .map(({dest, alias}) => `${alias}\t${dest}`)

  if (redirects.length > 0) {
    await fs.writeFile(
        path.resolve(options.outDir, '_redirects'),
        redirects.join('\n')
    )
  }
}
