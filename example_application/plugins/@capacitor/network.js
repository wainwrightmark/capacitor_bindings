import { registerPlugin } from './core.js';

const Network = registerPlugin('Network', {
    web: () => import('../common/web-42b8cd91.js').then(m => new m.NetworkWeb()),
});

export { Network };
