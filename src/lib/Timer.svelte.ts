import { Duration, type TimeDisplay } from "./Duration.svelte";
import { invoke } from "@tauri-apps/api/core";

function durationFromSeconds(durationInSecs: number) {
    const hours = Math.floor(durationInSecs / 3600);
    const minutes = Math.floor((durationInSecs % 3600) / 60);
    const seconds = durationInSecs % 60;

    return new Duration(hours, minutes, seconds);
}

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
            invoke("start_timer", { durationInSecs:  this._timeLeft.timeInSeconds })
                .then(() => {
                    this._timerState = TimerState.Running;
                     this._intervalId = setInterval(() => {
                        invoke<number>("get_time_left")
                            .then((time_left) => {
                                this._timeLeft = durationFromSeconds(time_left);
                                console.log("new time left");
                            })
                    }, 100)
                })
                .catch(() => {});
            
           
        }
    }

    pause() {
        if (this._timerState === TimerState.Running) {
            invoke<number>("stop_timer")
                .then((time_left) => {
                    this._timeLeft = durationFromSeconds(time_left);
                    this._timerState = TimerState.Paused;
                    this.clearFetchTimeLeft();
                })
                .catch((_) => {});
        }
    }

    skip() {
        if (this._timerState !== TimerState.Finished) {
            invoke<number>("stop_timer")
                .then((_) => {
                    this.setDone()
                })
                .catch((_) => {});
        }
    }

    reset() {
        if (this._timerState === TimerState.Running) {
            invoke<number>("stop_timer").catch((_) => {
                return;
            });
            this.clearFetchTimeLeft();
        }
        this._timerState = TimerState.Paused;
        this._timeLeft = this._duration.clone();
    }

    setDone() {
        this.clearFetchTimeLeft();
        this._timerState = TimerState.Finished;
        this._timeLeft.setZero();
    }

    clearFetchTimeLeft() {
        clearInterval(this._intervalId);
    }
}