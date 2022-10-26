<script>
    import {onMount} from "svelte";
    import {DarkMode, Navbar, NavBrand, NavHamburger, NavLi, NavUl} from 'flowbite-svelte'
    import {isAdmin, isLoggedIn, secretCounter} from "../lib/stores.js";
    import {navigating} from "$app/stores";

    let btnClass = "text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 rounded-lg text-sm 5 z-50";

    let links = []

    function checkSignIn() {
        //TODO: Check for logged in user
        return [$isLoggedIn, $isAdmin];
    }

    function generateNavLinks() {
        let res = checkSignIn();
        isLoggedIn.update(() => res[0]);
        isAdmin.update(() => res[1]);

        links = []
        links = [...links, {label: "Home", href: "/"}];

        if ($isLoggedIn) {
            //order is important, else it looks ugly and makes no sense from ergonomics perspective
            links = [...links, {label: "Skills", href: "/skills"}];
            if ($isAdmin) {
                links = [...links, {label: "Admin Panel", href: "/admin"}];
            }
            links = [...links, {label: "Logout", href: "/logout"}];
        } else {
            links = [...links, {label: "Login", href: "/login"}];
        }
    }

    $: if ($navigating) generateNavLinks();

    secretCounter.subscribe(value => {
        if (value === 11) {
            let i = 0;
            let cl = document.querySelectorAll("body")[0].classList;
            let interval = setInterval(() => {
                cl.remove(...cl);
                if (i === 0) {
                    cl.add("bg-blue-200")
                    cl.add("dark:bg-blue-700")
                    i++;
                } else if (i === 1) {
                    cl.add("bg-red-200")
                    cl.add("dark:bg-red-700")
                    i++;
                } else if (i === 2) {
                    cl.add("bg-green-200")
                    cl.add("dark:bg-green-700")
                    i++;
                } else if (i === 3) {
                    cl.add("bg-yellow-200")
                    cl.add("dark:bg-yellow-700")
                    i++;
                } else if (i === 4) {
                    cl.add("bg-purple-200")
                    cl.add("dark:bg-purple-700")
                    i++;
                } else if (i === 5) {
                    cl.add("bg-gray-200")
                    cl.add("dark:bg-gray-700")
                    i++;
                } else if (i === 6) {
                    cl.add("bg-rose-200")
                    cl.add("dark:bg-rose-700")
                    i++;
                } else if (i === 7) {
                    cl.add("bg-fuchsia-200")
                    cl.add("dark:bg-fuchsia-700")
                    i++;
                } else if (i === 8) {
                    cl.add("bg-orange-200")
                    cl.add("dark:bg-orange-700")
                    i++;
                } else if (i === 9) {
                    cl.add("bg-emerald-200")
                    cl.add("dark:bg-emerald-700")
                    i = 0;
                }
            }, 100)

        }
    })

    onMount(() => {
        generateNavLinks();
    });

</script>

<Navbar color="form" let:hidden let:toggle rounded style="transition: 0.2s">
    <NavBrand href="/" style="z-index: 100">
        <img alt="Reading Pepe" class="mr-6 h-6 sm:h-9 scale-150 ml-3" src="pepe.webp"/>
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

