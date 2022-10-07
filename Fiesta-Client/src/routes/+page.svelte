<script>
    export let data;

    let multi = 0;
    let subtract = 0;
    let total = "";

    async function klickk() {
        const response = await fetch('/add', {
            method: 'POST',
            headers: {
                'content-type': 'application/json'
            },
            body: JSON.stringify({multi, subtract}),

        });
        total = await response.json();
    }
</script>
<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<p>{data.body.number}</p>
<button on:click={klickk}>make request</button>
<p>Calc ultimate thing by multiplying with <input bind:value={multi} type="number">
    and subtracting <input bind:value={subtract} type="number">= {total}</p>

