<script>
    import Header from "../components/Header.svelte";
    import "../app.postcss";
</script>

<Header></Header>
<div class="container mx-auto px-10">
    <slot></slot>
</div>