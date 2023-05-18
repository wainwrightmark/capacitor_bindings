import { registerPlugin } from './core.js';

const App = registerPlugin('App', {
    web: () => import('../common/web-d56acca6.js').then(m => new m.AppWeb()),
});

export { App };
