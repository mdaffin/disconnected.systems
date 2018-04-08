const { Router } = require('express')
const path = require('path')
const fs = require('fs')
const promisify = require('util').promisify;
const matter = require('gray-matter');
const toml = require('toml');

const readdir = promisify(fs.readdir);
const readFile = promisify(fs.readFile);
const access = promisify(fs.access);

function newCollectionsRouter(collections) {
    function load_collection(name) {
        return collections.find(x => x.name == name)
    }

    async function load_post(file_name) {
        try {
            const frontmatter = matter(
                await readFile(file_name, 'utf8'),
                {
                    language: 'toml',
                    delimiters: ['+++', '+++'],
                    engines: {
                        toml: toml.parse.bind(toml),
                    },
                }
            )
            const post =  {content: frontmatter.content, ...frontmatter.data}
            console.log(post)
            return {content: frontmatter.content, ...frontmatter.data}
        } catch (e) {
            throw `Error parsing frontmatter for '${file_name}' at ${e.line},${e.column}: ${e.message}`;
        }
    }

    function get_collections(req, res) {
        res.end(JSON.stringify(collections))
    }

    function get_collection(req, res, next) {
        const name = req.params.collection
        const c = collections.find(x => x.name == name)
        if (c !== undefined) {
            res.end(JSON.stringify(c))
        } else {
            next()
        }
    }

    async function get_posts(req, res, next) {
        const collection_name = req.params.collection;
        const collection_folder = path.resolve(load_collection(collection_name).folder);
        const files = fs.readdirSync(collection_folder).filter(f => f.endsWith(".md"));
        const posts = files.map(f => load_post(path.join(collection_folder, f)));
        res.end(JSON.stringify(await posts));
    }

    function get_post(req, res, next) {
        const collection_name = req.params.collection;
        const post_name = req.params.post;
        const post = get_post(path.join(post_name))
        res.end(JSON.stringify(post))
    }

    const router = Router()
    router.get('/', get_collections)
    router.get('/:collection', get_collection)
    router.get('/:collection/posts', get_posts)
    router.get('/:collection/posts/:post', get_post)
    return router
}

module.exports = function(config) {
    const router = Router()
    router.use('/collections', newCollectionsRouter(config.collections))
    return router
}
