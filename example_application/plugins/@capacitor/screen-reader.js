import { registerPlugin } from './core.js';

const ScreenReader = registerPlugin('ScreenReader', {
    web: () => import('../common/web-c37ed32f.js').then(m => new m.ScreenReaderWeb()),
});

export { ScreenReader };
