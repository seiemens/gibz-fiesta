<script>
    import {onMount} from "svelte";
    import {goto} from "$app/navigation";
    import {isAdmin, isLoggedIn} from "../../lib/stores.js";

    onMount(()=>{
        localStorage.clear();
        isLoggedIn.update(()=>false);
        isAdmin.update(()=>false);
        goto("/");
    })
</script>