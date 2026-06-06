<script lang="ts">
    import { type TimeDisplay } from "$lib/Duration.svelte";

    let { hours, minutes, seconds }: TimeDisplay = $props();
    let hoursPadded = $derived(hours.toString().padStart(2, "0"));
    let minutesPadded = $derived(minutes.toString().padStart(2, "0"));
    let secondsPadded = $derived(seconds.toString().padStart(2, "0"));
</script>

<div class="clock">
    {hoursPadded}:{minutesPadded}:{secondsPadded}
</div>


<style>

.clock {
    font-size: 50px;
    line-height: 1.2;
}

</style>