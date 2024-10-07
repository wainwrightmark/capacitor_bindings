import { registerPlugin } from './core.js';

const Preferences = registerPlugin('Preferences', {
    web: () => import('../common/web-9807141f.js').then(m => new m.PreferencesWeb()),
});

export { Preferences };
