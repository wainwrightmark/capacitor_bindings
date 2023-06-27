import { WebPlugin } from '../@capacitor/core.js';

class SplashScreenWeb extends WebPlugin {
    async show(_options) {
        return undefined;
    }
    async hide(_options) {
        return undefined;
    }
}

export { SplashScreenWeb };
