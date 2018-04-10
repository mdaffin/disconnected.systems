import test from 'ava';

import {Nuxtlify, Collection} from './src/lib.js'

test('config and collections gets included in the object', t => {
    const config = {
        backend: {
          name: "git-gateway",
          branch: "master",
          folder: "some/dir",
        },
    };
    const collections = [{name: "blog", label: "Blog"}];

    const nuxtlify = new Nuxtlify({
        ...config,
        collections,
    });

    t.deepEqual(nuxtlify.config, config);
    t.deepEqual(nuxtlify.collections, collections.map(c => new Collection(c)));
});
