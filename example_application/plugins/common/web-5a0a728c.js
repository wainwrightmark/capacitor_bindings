import { WebPlugin } from '../@capacitor/core.js';

class MotionWeb extends WebPlugin {
    constructor() {
        super();
        this.registerWindowListener('devicemotion', 'accel');
        this.registerWindowListener('deviceorientation', 'orientation');
    }
}

export { MotionWeb };
