<script>
    import {onMount} from "svelte";

    let timer = 7;

    onMount(async () => {
        let i = timer * 2;
        let interval = setInterval(() => {
            i--;
            if (i === 0) {
                clearInterval(interval);
                document.getElementById("bomb").style.display = "block";
            }
            if (i % 2 === 0) {
                timer--;
                document.getElementById("alarm").classList.add("warning");
            } else {
                document.getElementById("alarm").classList.remove("warning");
            }
        }, 500)

    })

</script>
<div id="alarm" class="warning stretch"></div>
<img src="https://media.tenor.com/AFceZ8FyeV0AAAAC/explosion-boom.gif" alt="bomb exploding" class="stretch" id="bomb">
<div class="center">
    <h1 class="text-6xl text-center mb-8 text-gray-700 dark:text-gray-300">Error 404</h1>
    <h2 class="text-3xl text-center text-gray-700 dark:text-gray-300">Page Not Found</h2>
    <h3 class="text-3xl text-center text-gray-700 dark:text-gray-300">Self Destruction Initiated <span class="font-bold">{timer}s</span></h3>
</div>
<style>
    * {
        animation: 5s explode;
    }
    :global(body){
        overflow: hidden;
    }

    .center {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
    }

    .stretch {
        z-index: 10;
        pointer-events: none; /* make click-thru */
        position: absolute;
        display: block;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
    }

    .warning {
        background-color: rgba(255, 0, 0, 0.2);
    }

    #bomb {
        display: none;
    }
</style>