import { registerPlugin } from './core.js';

const Toast = registerPlugin('Toast', {
    web: () => import('../common/web-c35afa81.js').then(m => new m.ToastWeb()),
});

export { Toast };
