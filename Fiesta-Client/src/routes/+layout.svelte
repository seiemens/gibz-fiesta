<script>
    import Header from "../components/Header.svelte";
    import "../app.postcss";
</script>
<!--Template for all the sites. Contains Header and a default Wrapper Div-->
<Header></Header>
<div class="container mx-auto ">
    <slot></slot>
</div>