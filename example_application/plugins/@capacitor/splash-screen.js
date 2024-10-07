import { registerPlugin } from './core.js';

const SplashScreen = registerPlugin('SplashScreen', {
    web: () => import('../common/web-171ab330.js').then(m => new m.SplashScreenWeb()),
});

export { SplashScreen };
