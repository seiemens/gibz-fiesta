<script>
    import {Accordion, AccordionItem, Button, ButtonGroup, Spinner} from "flowbite-svelte";
    import {onMount} from "svelte";

    async function loadSkills() {
        const response = await fetch('/testdata.json', {
            method: 'GET',
            headers: {
                'content-type': 'application/json'
            }
        });
        return await response.json();
    }

    let skills;
    let loading = true;

    onMount(async () => {
        skills = await loadSkills();
        loading = false;
    })

    async function changeComplete(skillId, levelIndex, status) {
        $: for (let i = 0; i < skills.skills.length; i++) {
            if (skills.skills[i].id === skillId) {
                for (let j = 0; j < skills.skills[i].levels.length; j++) {
                    if (skills.skills[i].levels[j].index === levelIndex) {
                        skills.skills[i].levels[j].completed = status;
                        //TODO: SYNC WITH DB
                        break;
                    }
                }

            }
        }
    }

    async function changeMark(skillId, status) {
        $: for (let i = 0; i < skills.skills.length; i++) {
            if (skills.skills[i].id === skillId) {
                skills.skills[i].marked = status;
                //TODO: SYNC WITH DB
                break;
            }
        }
    }

</script>
<div class="container my-24">
    <h1 class="text-4xl text-center mb-8 text-gray-700 dark:text-gray-300">Skills</h1>

    {#if loading}
        <div class="text-center">
            <Spinner size={10}/>
        </div>
    {:else}
        <Accordion
                activeClasses="bg-blue-100 dark:bg-gray-700 text-blue-600 dark:text-white"
                inactiveClasses="text-gray-500 dark:text-gray-400 hover:bg-blue-100 dark:hover:bg-gray-700">
            {#each skills.skills as skill}
                <AccordionItem open>
                    <div slot="header" class="flex items-center w-full">
                        <p>{skill.display_name}</p>
                        <div class="ml-auto">
                            {#if skill.marked}
                                <Button outline color="yellow" class="scale-75" on:click={()=>{changeMark(skill.id,false)}}>Unmark</Button>
                            {:else}
                                <Button outline color="yellow" class="scale-75" on:click={()=>{changeMark(skill.id,true)}}>Mark</Button>
                            {/if}
                        </div>
                    </div>
                    <Accordion
                            activeClasses="bg-blue-100 dark:bg-gray-700 text-blue-600 dark:text-white"
                            inactiveClasses="text-gray-500 dark:text-gray-400 hover:bg-blue-100 dark:hover:bg-gray-700">
                        {#each skill.levels as level}
                            <AccordionItem>
                                <div slot="header" class="flex items-center w-full">
                                    <p>{level.display_name}</p>
                                    <div class="ml-auto">
                                        {#if level.completed}
                                            <Button gradient shadow="green" color="green" class="scale-75" on:click={()=>{changeComplete(skill.id, level.index, false)}}>Uncomplete</Button>
                                        {:else}
                                            <Button gradient shadow="green" color="green" class="scale-75" on:click={()=>{changeComplete(skill.id, level.index, true)}}>Complete</Button>

                                        {/if}
                                    </div>
                                </div>
                                <p class="mb-2 text-gray-500 dark:text-gray-400">{level.description}</p>
                                {#each level.resources as resource}
                                    <Button gradient color="purpleToBlue" shadow="blue" href="{resource.url}" class="ml-2 mb-2">{resource.display_name}</Button>
                                {/each}
                            </AccordionItem>
                        {/each}
                    </Accordion>
                </AccordionItem>
            {/each}
        </Accordion>
    {/if}
</div>