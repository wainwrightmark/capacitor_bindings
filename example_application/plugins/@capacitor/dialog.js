import { registerPlugin } from './core.js';

const Dialog = registerPlugin('Dialog', {
    web: () => import('../common/web-53a6aa72.js').then(m => new m.DialogWeb()),
});

export { Dialog };
