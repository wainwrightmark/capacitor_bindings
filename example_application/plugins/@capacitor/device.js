import { registerPlugin } from './core.js';

const Device = registerPlugin('Device', {
    web: () => import('../common/web-67daad1f.js').then(m => new m.DeviceWeb()),
});

export { Device };
