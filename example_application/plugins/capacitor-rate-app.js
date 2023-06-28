import { registerPlugin } from './@capacitor/core.js';

const RateApp = registerPlugin('RateApp', {
    web: () => import('./common/web-d47d0e1c.js').then(m => new m.RateAppWeb()),
});

export { RateApp };
