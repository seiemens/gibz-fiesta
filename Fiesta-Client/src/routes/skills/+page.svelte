<script>
    import {Accordion, AccordionItem, Button, Hr, Mark, Spinner} from "flowbite-svelte";
    import {onMount} from "svelte";
    import {beforeNavigate, goto} from "$app/navigation";
    import {isLoggedIn} from "$lib/stores.js";

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

        if (!$isLoggedIn) {
            await goto("/login")
        }

        skills = await loadSkills();
        loading = false;
    })

    //fix accordion being visible for 1s after navigating
    beforeNavigate(({from, to}) => {
        if (from.url.pathname !== to.url.pathname) {
            let e = document.getElementById("rootDiv");
            if (e != null)
                e.style.display = "none";
        }
    })

    async function changeComplete(e,skillId, levelIndex, status) {
        e.stopPropagation();
        for (let i = 0; i < skills.skills.length; i++) {
            if (skills.skills[i].id === skillId) {
                for (let j = 0; j < skills.skills[i].levels.length; j++) {
                    if (skills.skills[i].levels[j].index === levelIndex) {
                        skills.skills[i].levels[j].completed = status;
                        //TODO: SYNC WITH DB
                        break;
                    }
                }
                for (let j = 0; j < skills.skills[i].levels.length; j++) {
                    if (skills.skills[i].levels[j].completed === false) {
                        skills.skills[i].all_completed = false;
                        return;
                    }
                }
                skills.skills[i].all_completed = true;
            }
        }
    }

    async function changeMark(e, skillId, status) {
        e.stopPropagation();
        for (let i = 0; i < skills.skills.length; i++) {
            if (skills.skills[i].id === skillId) {
                skills.skills[i].marked = status;
                //TODO: SYNC WITH DB
                break;
            }
        }
    }

</script>
<div class="container my-24" id="rootDiv">
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
                <AccordionItem>
                    <div slot="header" class="flex items-center w-full">
                        <p>
                            {#if skill.marked}
                                <span class="px-2 text-white rounded bg-yellow-300 dark:bg-yellow-400">{skill.display_name}</span>
                            {:else}
                                {skill.display_name}
                            {/if}
                            {#if skill.all_completed}
                                <span class="px-2 text-white rounded bg-green-500 dark:bg-green-700">Done!</span>
                            {/if}
                        </p>
                        <div class="ml-auto">
                            {#if skill.marked}
                                <Button outline color="yellow" class="scale-75" on:click={(e)=>{changeMark(e,skill.id,false)}}>Unmark</Button>
                            {:else}
                                <Button outline color="yellow" class="scale-75" on:click={(e)=>{changeMark(e,skill.id,true)}}>Mark</Button>
                            {/if}
                        </div>
                    </div>
                    <Accordion
                            activeClasses="bg-blue-100 dark:bg-gray-700 text-blue-600 dark:text-white"
                            inactiveClasses="text-gray-500 dark:text-gray-400 hover:bg-blue-100 dark:hover:bg-gray-700">
                        {#each skill.levels as level}
                            <AccordionItem>
                                <div slot="header" class="flex items-center w-full">
                                    <p>
                                        {#if level.completed}
                                            <span class="px-2 text-white rounded bg-green-500 dark:bg-green-700">{level.display_name}</span>
                                        {:else}
                                            {level.display_name}
                                        {/if}
                                    </p>
                                    <div class="ml-auto">
                                        {#if level.completed}
                                            <Button gradient shadow="red" color="red" class="scale-75" on:click={(e)=>{changeComplete(e,skill.id, level.index, false)}}>Uncomplete</Button>
                                        {:else}
                                            <Button gradient shadow="green" color="green" class="scale-75" on:click={(e)=>{changeComplete(e,skill.id, level.index, true)}}>Complete</Button>

                                        {/if}
                                    </div>
                                </div>
                                <p class="mb-2 text-gray-500 dark:text-gray-400">{level.description}</p>
                                <Hr class="my-8" height="h-px"/>
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