import { registerPlugin } from './core.js';

const Preferences = registerPlugin('Preferences', {
    web: () => import('../common/web-7dc896a8.js').then(m => new m.PreferencesWeb()),
});

export { Preferences };
