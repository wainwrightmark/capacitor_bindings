import { registerPlugin } from './core.js';

const App = registerPlugin('App', {
    web: () => import('../common/web-207a4df5.js').then(m => new m.AppWeb()),
});

export { App };
