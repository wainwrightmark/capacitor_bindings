import { registerPlugin } from './core.js';

const Share = registerPlugin('Share', {
    web: () => import('../common/web-153d7710.js').then(m => new m.ShareWeb()),
});

export { Share };
