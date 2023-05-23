import { registerPlugin } from './core.js';

const Motion = registerPlugin('Motion', {
    android: () => import('../common/web-5a0a728c.js').then(m => new m.MotionWeb()),
    ios: () => import('../common/web-5a0a728c.js').then(m => new m.MotionWeb()),
    web: () => import('../common/web-5a0a728c.js').then(m => new m.MotionWeb()),
});

export { Motion };
