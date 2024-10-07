import { registerPlugin } from './core.js';

const Toast = registerPlugin('Toast', {
    web: () => import('../common/web-a1996e6b.js').then(m => new m.ToastWeb()),
});

export { Toast };
