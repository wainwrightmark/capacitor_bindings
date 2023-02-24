import { registerPlugin } from './core.js';

var ActionSheetButtonStyle;
(function (ActionSheetButtonStyle) {
    /**
     * Default style of the option.
     *
     * @since 1.0.0
     */
    ActionSheetButtonStyle["Default"] = "DEFAULT";
    /**
     * Style to use on destructive options.
     *
     * @since 1.0.0
     */
    ActionSheetButtonStyle["Destructive"] = "DESTRUCTIVE";
    /**
     * Style to use on the option that cancels the Action Sheet.
     * If used, should be on the latest availabe option.
     *
     * @since 1.0.0
     */
    ActionSheetButtonStyle["Cancel"] = "CANCEL";
})(ActionSheetButtonStyle || (ActionSheetButtonStyle = {}));
/**
 * @deprecated Use `ActionSheetButtonStyle`.
 * @since 1.0.0
 */
const ActionSheetOptionStyle = ActionSheetButtonStyle;

const ActionSheet = registerPlugin('ActionSheet', {
    web: () => import('../common/web-31f5983c.js').then(m => new m.ActionSheetWeb()),
});

export { ActionSheet, ActionSheetButtonStyle, ActionSheetOptionStyle };
