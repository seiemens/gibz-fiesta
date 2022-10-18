<script>
    import Header from "../components/Header.svelte";
    import "../app.postcss";
    import {DarkMode} from "flowbite-svelte";
</script>

<Header></Header>
<DarkMode/>
<slot></slot>