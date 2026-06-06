<script lang="ts">
    let { submit, cancel } = $props();

    let hours = $state(0);
    let minutes = $state(0);
    let seconds = $state(0);

    const minTime = 0;
    const maxHours = 23;
    const maxMinutes = 59;
    const maxSeconds = 59;


    function validateInput(): boolean {
        const hoursOk = Number.isInteger(hours) && minTime <= hours && hours <= maxHours;
        const minutesOk = Number.isInteger(minutes) && minTime <= minutes && minutes <= maxMinutes;
        const secondsOk = Number.isInteger(seconds) && minTime <= seconds && seconds <= maxSeconds;
        const notZero = hours !== 0 || minutes !== 0 || seconds !== 0;
        return hoursOk && minutesOk && secondsOk && notZero;
    }

    function onsubmit() { 
        if (validateInput()) {
            submit(hours, minutes, seconds);
        }
    }

    function cancelInput() {
        cancel();
    }
</script>

<form {onsubmit}>
    <input bind:value={hours} type="number" min="0" max="23" step="1" required>
    <input bind:value={minutes} type="number" min="0" max="59" step="1" required>
    <input bind:value={seconds} type="number" min="0" max="59" step="1" required>
    <br>
    <button onclick={cancelInput}>Cancel</button>
    <button type="submit">Submit</button>
</form>