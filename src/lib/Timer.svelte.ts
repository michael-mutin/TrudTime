import { Duration, type TimeDisplay } from "./Duration.svelte";
import sound from '$lib/assets/universfield-new-notification-050-494248.mp3';

export enum TimerState {
    Running,
    Paused,
    Finished,
}

export class Timer {
    _timeLeft = $state(new Duration(0, 0, 0));
    _intervalId: number | undefined;
    _timerState = $state(TimerState.Paused);

    constructor(private _duration: Duration) {
        this._timeLeft = _duration.clone();
        this._intervalId = undefined;
    }

    get timerState(): TimerState {
        return this._timerState;
    }

    getTimeDisplay(): TimeDisplay {
        return this._timeLeft.getTimeDisplay();
    }

    run() {
        if (this._timerState === TimerState.Paused) {
            this._timerState = TimerState.Running;
            this._intervalId = setInterval(() => {
                if (this._timeLeft.isZero()) {
                    this._timerState = TimerState.Finished;
                    clearInterval(this._intervalId as number);
                    // TODO: notification
                    const audio = new Audio(sound);
                    audio.play()
                } else {
                    this._timeLeft.subtract1Sec();
                }
            }, 1000)
        }
    }

    pause() {
        if (this._timerState === TimerState.Running) {
            this._timerState = TimerState.Paused;
            clearInterval(this._intervalId as number);
        }
    }

    skip() {
        if (this._timerState !== TimerState.Finished) {
            this._timerState = TimerState.Finished;
            this._timeLeft.setZero();
        }
    }

    reset() {
        if (this._timerState === TimerState.Running) {
            clearInterval(this._intervalId as number);
        }
        this._timerState = TimerState.Paused;
        this._timeLeft = this._duration.clone();
    }
}