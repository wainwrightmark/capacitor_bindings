import { registerPlugin } from './core.js';

const Dialog = registerPlugin('Dialog', {
    web: () => import('../common/web-a2cac75a.js').then(m => new m.DialogWeb()),
});

export { Dialog };
