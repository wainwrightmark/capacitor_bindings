import { registerPlugin } from './core.js';

const Device = registerPlugin('Device', {
    web: () => import('../common/web-47224e2e.js').then(m => new m.DeviceWeb()),
});

export { Device };
