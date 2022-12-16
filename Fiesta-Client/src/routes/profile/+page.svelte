<script>
    import {goto} from "$app/navigation";
    import {onMount} from "svelte";

    //Redirect to Root because there is nothing to see here.
    onMount(()=>{
        goto("/")
    })
</script>