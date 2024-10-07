import { registerPlugin } from './core.js';

const Device = registerPlugin('Device', {
    web: () => import('../common/web-8e565224.js').then(m => new m.DeviceWeb()),
});

export { Device };
