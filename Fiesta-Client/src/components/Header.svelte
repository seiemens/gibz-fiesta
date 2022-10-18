<script>
    import {onMount} from "svelte";
    import {Navbar, NavBrand, NavLi, NavUl, NavHamburger} from 'flowbite-svelte'

    let links = [
        {label: "Home", href: "/"}
    ]

    let isSignedIn = false;
    let isAdmin = true;

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

<Navbar let:hidden let:toggle rounded color="form">
    <NavBrand href="/" style="z-index: 100">
        <img src="pepe.webp" class="mr-3 h-6 sm:h-9" alt="Reading Pepe"/>
        <span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">GIBZ FIEŚTA</span>
    </NavBrand>
    <NavHamburger on:click={toggle}/>
    <NavUl {hidden}>
        {#each links as link}
            <NavLi href="{link.href}">{link.label}</NavLi>
        {/each}
    </NavUl>
</Navbar>

