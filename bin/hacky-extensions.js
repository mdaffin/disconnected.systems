#!/usr/bin/env node
async function main() {
  const path = require('path')
  const fs = require('fs-extra')
  const plugin = require('../site/.vuepress')

  const prepare = require('../node_modules/vuepress/lib/prepare')
  const options = await prepare('site')

  await plugin({options})
}

main()
