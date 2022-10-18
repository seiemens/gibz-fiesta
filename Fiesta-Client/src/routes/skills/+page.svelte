<script>
    import {Accordion, AccordionItem} from "flowbite-svelte";
</script>
<h1 class="mb-5 text-gray-500 dark:text-gray-400 text-4xl mr-3 h-6">Skills</h1>
<Accordion
        activeClasses="bg-blue-100 dark:bg-gray-700 text-blue-600 dark:text-white"
        inactiveClasses="text-gray-500 dark:text-gray-400 hover:bg-blue-100 dark:hover:bg-gray-700">
    <AccordionItem open>
        <span slot="header">Header 2-1</span>
        <p class="mb-2 text-gray-500 dark:text-gray-400">Lorem ipsum dolor sit amet, consectetur adipisicing elit.
            Illo
            ab necessitatibus sint explicabo ...</p>
    </AccordionItem>
    <AccordionItem>
        <span slot="header">Header 2-2</span>
        <p class="mb-2 text-gray-500 dark:text-gray-400">Lorem ipsum dolor sit amet, consectetur adipisicing elit.
            Illo
            ab necessitatibus sint explicabo ...</p>
    </AccordionItem>
</Accordion>