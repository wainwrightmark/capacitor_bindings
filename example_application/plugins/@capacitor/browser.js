import { registerPlugin } from './core.js';

const Browser = registerPlugin('Browser', {
    web: () => import('../common/web-926d467e.js').then(m => new m.BrowserWeb()),
});

export { Browser };
