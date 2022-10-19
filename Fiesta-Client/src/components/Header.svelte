<script>
    import {onMount} from "svelte";
    import {Navbar, NavBrand, NavLi, NavUl, NavHamburger} from 'flowbite-svelte'
    import {DarkMode} from "flowbite-svelte";

    let btnClass = "text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 rounded-lg text-sm 5 z-50";

    let links = [
        {label: "Home", href: "/"}
    ]

    let isSignedIn = false;
    let isAdmin = true;

    function checkSignIn() {
        return [false, true];
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
        <img src="pepe.webp" class="mr-6 h-6 sm:h-9 scale-150 ml-3" alt="Reading Pepe"/>
        <span class="self-center whitespace-nowrap text-4xl font-semibold dark:text-white">GIBZ FIEŚTA</span>
    </NavBrand>
    <NavHamburger on:click={toggle}/>
    <NavUl {hidden}>
        {#each links as link}
            <NavLi href="{link.href}">{link.label}</NavLi>
        {/each}
        <NavLi>
            <DarkMode {btnClass}/>
        </NavLi>
    </NavUl>
</Navbar>

