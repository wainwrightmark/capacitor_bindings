import { registerPlugin } from './core.js';

const Toast = registerPlugin('Toast', {
    web: () => import('../common/web-90727218.js').then(m => new m.ToastWeb()),
});

export { Toast };
