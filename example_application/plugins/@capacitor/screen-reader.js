import { registerPlugin } from './core.js';

const ScreenReader = registerPlugin('ScreenReader', {
    web: () => import('../common/web-9c42efba.js').then(m => new m.ScreenReaderWeb()),
});

export { ScreenReader };
