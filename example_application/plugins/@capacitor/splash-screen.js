import { registerPlugin } from './core.js';

const SplashScreen = registerPlugin('SplashScreen', {
    web: () => import('../common/web-1a6656a6.js').then(m => new m.SplashScreenWeb()),
});

export { SplashScreen };
