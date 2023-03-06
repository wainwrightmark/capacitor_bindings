import { WebPlugin } from '../@capacitor/core.js';

class ScreenReaderWeb extends WebPlugin {
    async isEnabled() {
        throw this.unavailable('This feature is not available in the browser.');
    }
    async speak(options) {
        if (!('speechSynthesis' in window)) {
            throw this.unavailable('Browser does not support the SpeechSynthesis API');
        }
        const utterance = new SpeechSynthesisUtterance(options.value);
        if (options.language) {
            utterance.lang = options.language;
        }
        speechSynthesis.speak(utterance);
    }
}

export { ScreenReaderWeb };
