import { registerPlugin } from './core.js';

const Preferences = registerPlugin('Preferences', {
    web: () => import('../common/web-672d63c0.js').then(m => new m.PreferencesWeb()),
});

export { Preferences };
