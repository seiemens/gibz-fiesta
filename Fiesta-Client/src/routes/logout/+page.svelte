<script>
    import {onMount} from "svelte";
    import {goto} from "$app/navigation";
    import {isAdmin, isLoggedIn} from "../../lib/stores.js";
    import {logout} from "../../lib/apiCalls.js";

    onMount(async ()=>{
        await logout()
        localStorage.clear();
        document.cookie = ""
        isLoggedIn.update(()=>false);
        isAdmin.update(()=>false);
        await goto("/");
    })
</script>