import { registerPlugin } from './core.js';

const Dialog = registerPlugin('Dialog', {
    web: () => import('../common/web-fe308abf.js').then(m => new m.DialogWeb()),
});

export { Dialog };
