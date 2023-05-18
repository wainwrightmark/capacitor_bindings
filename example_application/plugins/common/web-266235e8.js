import { WebPlugin } from '../@capacitor/core.js';

class ActionSheetWeb extends WebPlugin {
    async showActions(options) {
        return new Promise((resolve, _reject) => {
            let actionSheet = document.querySelector('pwa-action-sheet');
            if (!actionSheet) {
                actionSheet = document.createElement('pwa-action-sheet');
                document.body.appendChild(actionSheet);
            }
            actionSheet.header = options.title;
            actionSheet.cancelable = false;
            actionSheet.options = options.options;
            actionSheet.addEventListener('onSelection', async (e) => {
                const selection = e.detail;
                resolve({
                    index: selection,
                });
            });
        });
    }
}

export { ActionSheetWeb };
