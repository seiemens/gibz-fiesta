<script>
    import {onMount} from "svelte";
    import {goto} from "$app/navigation";
    import {isAdmin, isLoggedIn} from "../../lib/stores.js";
    import {logout} from "../../lib/apiCalls.js";

    //Delete Cookies and Reset all Stores.
    onMount(async ()=>{
        await logout();
        localStorage.clear();
        isLoggedIn.update(()=>false);
        isAdmin.update(()=>false);
        await goto("/");
    })
</script>