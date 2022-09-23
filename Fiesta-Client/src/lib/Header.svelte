<script>
    import { fly } from 'svelte/transition';

    let links =[
        {label: "sus 1", href: "#"},
        {label: "sus 2", href: "#"},
        {label: "sus 3", href: "#"},
        {label: "sus 4", href: "#"},
        {label: "sus 5", href: "#"},
    ]

    let visible = true;
    function menutToggle() {
        visible = !visible;
    }
</script>

<header>
    <img src="https://cdn.discordapp.com/emojis/873878094333747251.webp" alt="reading pepe">
    <h1>GIBZ FIEŚTA</h1>
    <ul class="nav-big">
        {#each links as link}
            <li><a href="{link.href}">{link.label}</a></li>
        {/each}
    </ul>
    <div class="nav-small">
        <a on:click={menutToggle}><i class="material-icons">menu</i></a>
        {#if visible}
            <ul transition:fly="{{ x: 200, duration: 200 }}">
                {#each links as link}
                    <li><a href="{link.href}">{link.label}</a></li>
                {/each}
            </ul>
        {/if}
    </div>
</header>
<style>
    header,.nav-big {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 20px;
        background-color: #535bf2;
    }
    header{
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
    h1{
        width: max-content;
    }
    .nav-small{
        display: none;
        flex-direction: row;
        align-items: center;
        gap: 20px;
        background-color: #535bf2;
        margin: 0 20px 0 auto;
    }
    .menu-btn{
        z-index: 10;
    }
    .nav-small ul{
        position: absolute;
        top:60px;
        right:0;
        background-color: #3d45b8;
        width: 70vw;
        margin: 0;
        z-index: 9;
    }

    @media(max-width: 620px){
        .nav-big{
            display: none;
        }
        .nav-small{
            display: flex;
        }
    }
</style>