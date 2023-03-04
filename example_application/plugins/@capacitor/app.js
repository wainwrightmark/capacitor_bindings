import { registerPlugin } from './core.js';

const App = registerPlugin('App', {
    web: () => import('../common/web-b1d9d3d2.js').then(m => new m.AppWeb()),
});

export { App };
