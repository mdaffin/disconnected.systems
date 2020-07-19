const path = require('path')
const fs = require('fs-extra')

async function netlifyRedirects({options}) {
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

async function rssFeed({options}) {
  var RSS = require('rss')
  var feed = new RSS({
    title: options.siteData.title,
    description: options.siteData.description,
    feed_url: "https://disconnected.systems/rss.xml",
    site_url: "https://disconnected.systems",
    copyright: "2018 Michael Daffin",
    language: 'en',
  })

  options.siteData.pages
    .filter(page => page.path.startsWith('/blog/'))
    .map(page => ({...page, date: new Date(page.frontmatter.date)}))
    .sort((a, b) => b.date - a.date)
    .map(page => ({
      title: page.title,
      description: page.frontmatter.description,
      url: `https://disconnected.systems${page.path}`,
      //guid: "",
      //categories: "",
      date: page.date,
    }))
    .forEach(page => feed.item(page))

  await fs.writeFile(
    path.resolve(options.outDir, 'rss.xml'),
    feed.xml()
  )
}

module.exports = async function (args) {
  await netlifyRedirects(args)
  await rssFeed(args)
}
