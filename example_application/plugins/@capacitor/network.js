import { registerPlugin } from './core.js';

const Network = registerPlugin('Network', {
    web: () => import('../common/web-60057aea.js').then(m => new m.NetworkWeb()),
});

export { Network };
