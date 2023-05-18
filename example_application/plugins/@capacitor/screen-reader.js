import { registerPlugin } from './core.js';

const ScreenReader = registerPlugin('ScreenReader', {
    web: () => import('../common/web-3fe059b5.js').then(m => new m.ScreenReaderWeb()),
});

export { ScreenReader };
