import { registerPlugin } from './core.js';

const Share = registerPlugin('Share', {
    web: () => import('../common/web-73f790ea.js').then(m => new m.ShareWeb()),
});

export { Share };
