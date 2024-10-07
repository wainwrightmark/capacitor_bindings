import { registerPlugin } from '../@capacitor/core.js';

const InAppReview = registerPlugin('InAppReview', {
    web: () => import('../common/web-5343d4a6.js').then(m => new m.InAppReviewWeb()),
});

export { InAppReview };
