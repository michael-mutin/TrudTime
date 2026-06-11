<script lang="ts">
  import Clock from "$lib/components/Clock.svelte";
  import TimeInput from "$lib/components/TimeInput.svelte";
  import { Duration } from "$lib/Duration.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { Timer, TimerState } from "$lib/Timer.svelte";
  import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification'

  let timer = $state(new Timer(new Duration(0, 0, 5)));
  let timeDisplay = $derived(timer.getTimeDisplay());

  let dialogSetTimer: HTMLDialogElement;

  let permissionGranted = false;

  $effect(() => {
    async function getPermission() {
      permissionGranted = await isPermissionGranted();
      if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
      }
    }
    
    getPermission();
  })


  function changeTimer() {
    if (timer.timerState === TimerState.Running) {
      timer.pause()
    }
    dialogSetTimer.showModal();
  }

  function setNewTimer(hours: number, minutes: number, seconds: number) {
    const duration = new Duration(hours, minutes, seconds);
    timer = new Timer(duration);
  }

  listen("over", () => {
    timer.setDone();
    invoke('play_sound').catch(() => {})
    if (permissionGranted) {
      sendNotification({ title: '⌛', body: 'Time\'s up!' });
    }
  })
  // TODO: After timer finished or skipped the user shall be asked how much time should be captured
</script>

<main class="container">
  <h1>TrudTime ⚒️</h1>
  <Clock {...timeDisplay} />
  <div>
    {#if timer.timerState === TimerState.Paused}
      <button class="timer-function" onclick={() => timer.run()}>▶</button>
    {/if}
    {#if timer.timerState === TimerState.Running}
      <button class="timer-function" onclick={() => timer.pause()}>⏸️</button>
    {/if}
    {#if timer.timerState !== TimerState.Finished}
      <button class="timer-function" onclick={() => timer.skip()}>⏩</button>
    {/if}
    <button class="timer-function" onclick={() => timer.reset()}>↩️</button>
    <button class="timer-function" onclick={changeTimer}>✏️</button>
  </div>
  <dialog bind:this={dialogSetTimer}>
    <TimeInput
      submit={(hours: number, minutes: number, seconds: number) => {
        setNewTimer(hours, minutes, seconds);
        dialogSetTimer.close();
      }}
      cancel={() => dialogSetTimer.close()}
    />
  </dialog>
</main>

<style>

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  align-items: center;
}

.timer-function {
  display: inline-block;
}

h1 {
  text-align: center;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}

</style>
