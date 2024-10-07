import { WebPlugin } from '../@capacitor/core.js';

class InAppReviewWeb extends WebPlugin {
    async requestReview() {
        throw this.unimplemented('Not implemented on web.');
    }
}

export { InAppReviewWeb };
