class Posts {
    constructor() {
        
    }
}

class Collection {
    constructor(inner) {
        inner && Object.assign(this, inner);
    }

    posts() {
        
    }
}

class Nuxtlify {
    constructor(config) {
        const {collections, ...config_items} = config;
        this.collections = collections.map(c => new Collection(c));
        this.config = config_items;
        console.log(this.collections);
    }
}

module.exports = {Nuxtlify, Collection}
