import { z } from "zod";

const ZeroTo59 = z.number().int().min(0).max(59);
const ZeroTo23 = z.number().int().min(0).max(23);

export interface TimeDisplay {
    readonly hours: number;
    readonly minutes: number;
    readonly seconds: number;
}

export class Duration {
    _timeInSec: number = $state(0)

    constructor(
        hours: number,
        minutes: number,
        seconds: number
    ) {
        ZeroTo23.parse(hours)
        ZeroTo59.parse(minutes);
        ZeroTo59.parse(seconds);
        this._timeInSec = hours * 3600 + minutes * 60 + seconds;
    }

    get hours(): number {
        return Math.floor(this._timeInSec / 3600);
    }

    get minutes(): number {
        const secWithoutHours = this._timeInSec - (this.hours * 3600);
        return Math.floor(secWithoutHours / 60);
    }

    get seconds(): number {
        return this._timeInSec % 60;
    }

    clone(): Duration {
        return new Duration(this.hours, this.minutes, this.seconds)
    }

    isZero(): boolean {
        return (this._timeInSec === 0);
    }

    subtract1Sec(): boolean {
        if (this.isZero()) {
            return false;
        }
        else {
            this._timeInSec -= 1;
            return true;
        }
    }

    setZero() {
        this._timeInSec = 0;
    }

    getTimeDisplay(): TimeDisplay {
        const hours = this.hours;
        const minutes = this.minutes;
        const seconds = this.seconds;
        return { hours, minutes, seconds };
    }
}