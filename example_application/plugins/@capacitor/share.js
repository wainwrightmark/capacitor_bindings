import { registerPlugin } from './core.js';

const Share = registerPlugin('Share', {
    web: () => import('../common/web-bb419041.js').then(m => new m.ShareWeb()),
});

export { Share };
