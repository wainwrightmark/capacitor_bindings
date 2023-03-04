import { registerPlugin } from './core.js';

const Network = registerPlugin('Network', {
    web: () => import('../common/web-08e24156.js').then(m => new m.NetworkWeb()),
});

export { Network };
