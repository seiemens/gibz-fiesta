<script>
    import {fly} from 'svelte/transition';
    import {onMount} from "svelte";

    let links = [
        {label: "Home", href: "/"}
    ]

    let visible = false;
    let isSignedIn = false;
    let isAdmin = true;

    function menutToggle() {
        visible = !visible;
    }

    function checkSignIn() {
        return [true, true];
    }

    onMount(() => {
        //TODO: Check for logged in user

        let res = checkSignIn();
        isSignedIn = res[0];
        isAdmin = res[1];
        if (isSignedIn) {
            //order is important, else it looks ugly and makes no sense from ergonomics perspective
            links = [...links, {label: "Skills", href: "/skills"}];
            if (isAdmin) {
                links = [...links, {label: "Admin Panel", href: "/admin"}];
            }
            links = [...links, {label: "Logout", href: "/logout"}];
        } else {
            links = [...links, {label: "Login", href: "/login"}];
        }
    });

</script>

<header>
    <img src="pepe.webp" alt="reading pepe">
    <h1 class="text-4xl">GIBZ FIEŚTA</h1>
    <ul class="nav-big">
        {#each links as link}
            <li><a href="{link.href}">{link.label}</a></li>
        {/each}
    </ul>
    <div class="nav-small">
        <a on:click={menutToggle} id="hamburger" href="#"><i class="material-icons">menu</i></a>
        {#if visible}
            <ul transition:fly={{x:200}}>
                {#each links as link}
                    <li><a href="{link.href}" on:click={menutToggle}>{link.label}</a></li>
                {/each}
            </ul>
        {/if}
    </div>
</header>
<style>
    header, .nav-big {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 20px;
        background-color: #535bf2;
    }

    /*for error page :)*/
    .nav-big li, .nav-small li, #hamburger {
        z-index: 11;
    }

    header {
        color: #f9f9f9;
        height: 60px;
        padding-left: 20px;
        width: min-content;
        min-width: calc(100vw - 20px);
    }

    ul {
        padding: 0;
        text-align: left;
        list-style: none;
        margin-right: 20px;
        /*move to the left*/
        margin-left: auto;
    }

    li {
        width: max-content;
    }

    a {
        cursor: pointer;
        text-decoration: none;
        color: #f9f9f9;
    }

    img {
        height: 50px;
    }

    h1 {
        width: max-content;
    }

    .nav-small {
        display: none;
        flex-direction: row;
        align-items: center;
        gap: 20px;
        background-color: #535bf2;
        margin: 0 20px 0 auto;
    }

    .nav-small ul {
        position: absolute;
        top: 60px;
        right: 0;
        background-color: #3d45b8;
        width: 70vw;
        margin: 0;
        z-index: 11;
    }

    @media (max-width: 620px) {
        .nav-big {
            display: none;
        }

        .nav-small {
            display: flex;
        }
    }
</style>