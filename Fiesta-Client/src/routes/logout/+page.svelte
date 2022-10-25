<script>
    import {onMount} from "svelte";
    import {goto} from "$app/navigation";
    import {isAdmin, isLoggedIn} from "../../lib/stores.js";

    onMount(()=>{
        //TODO: Delete jwt or whatever we decide to do from backend to invalidate it
        localStorage.clear();
        isLoggedIn.update(()=>false);
        isAdmin.update(()=>false);
        goto("/");
    })
</script>