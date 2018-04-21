#!/usr/bin/env node
const path = require('path')
const fs = require('fs-extra')

async function writeRedirects(options) {
    const redirects = options.siteData.pages
        .filter(page => page.frontmatter.aliases)
        .map(page => page.frontmatter.aliases.map(alias => ({
                dest: page.path,
                alias: alias
            })))
        .reduce((acc, aliases) => acc.concat(aliases), [])
        .map(({dest, alias}) => `${alias}\t${dest}`)
        .join('\n')
        
    await fs.writeFile(path.resolve(options.outDir, '_redirects'), redirects)
};

async function main() {
    const prepare = require('../node_modules/vuepress/lib/prepare')
    const options = await prepare('site')
    
    await writeRedirects(options)
}

main()
