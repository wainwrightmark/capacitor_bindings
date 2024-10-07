import { registerPlugin } from './core.js';

const Browser = registerPlugin('Browser', {
    web: () => import('../common/web-e2ee183c.js').then(m => new m.BrowserWeb()),
});

export { Browser };
