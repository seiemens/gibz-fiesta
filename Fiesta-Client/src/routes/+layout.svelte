<script>
    import Header from "../components/Header.svelte";
    import "../app.postcss";
</script>

<Header></Header>
<div class="container mx-auto ">
    <slot></slot>
</div>