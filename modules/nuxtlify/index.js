const nuxt = require('nuxt');
const YAML = require('yamljs');
const path = require('path');
const axios = require('axios');
const api = require('./api');

module.exports = async function NetlifyCMS(moduleOptions) {
    const config_file = path.resolve(
        this.options.srcDir,
        'static/admin/config.yml'
    )
    const config = YAML.load(config_file)

    this.addServerMiddleware({
        path: "ncms",
        handler: api(config),
    })

    //let response = await axios.get(':3000/ncms/collections');
    //console.log(response);

    //this.options.generate.routes
}
