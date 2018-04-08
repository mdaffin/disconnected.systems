const nuxt = require('nuxt');
const YAML = require('yamljs');
const path = require('path');
const api = require('./api');

module.exports = function NetlifyCMS(moduleOptions) {
    const config_file = path.resolve(
        this.options.srcDir,
        'static/admin/config.yml'
    )
    const config = YAML.load(config_file)

    this.addServerMiddleware({
        path: "netlify-cms",
        handler: api(config),
    })
}
