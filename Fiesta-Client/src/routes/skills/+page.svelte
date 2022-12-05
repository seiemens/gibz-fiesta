<script>
    import {Accordion, AccordionItem, Button, Hr, Input, Spinner} from "flowbite-svelte";
    import {onMount} from "svelte";
    import {beforeNavigate, goto} from "$app/navigation";
    import {isLoggedIn} from "$lib/stores.js";
    import {hideAccordion} from "$lib/utils.js";
    import {loadSkills} from "$lib/apiCalls.js";
    import {user} from "../../lib/stores.js";
    import {checkSignIn} from "../../lib/utils.js";

    let skills;
    let filteredSkills = [];
    let filterQuery = "";
    let loading = true;

    onMount(async () => {

        if ($user === undefined)
            $user = await checkSignIn();

        if (!$isLoggedIn) {
            await goto("/")
        }
        skills = (await loadSkills()).skills;
        setupMarkings();
        loading = false;
    })

    //fix accordion being visible for 1s after navigating
    beforeNavigate(({from, to}) => {
        hideAccordion(from, to)
    })

    $: try {
        filteredSkills = skills.filter(
            (item) => item.display_name.toLowerCase().indexOf(filterQuery.toLowerCase()) !== -1
        );
    } catch (e) {
    }

    function setupMarkings() {
        for (let i = 0; i < skills.length; i++) {
            //set markings
            let tmp;
            tmp = $user.marked_skills.find((item) => item.id === skills[i].id);
            skills[i].marked = tmp !== undefined;

            //set completed levels
            for (let j = 0; j < skills[i].levels.length; j++) {
                tmp = $user.completed_skills.find((item) => item.id === skills[i].id)
                if (tmp !== undefined) {
                    tmp = tmp.levels.find((item) => item.index === skills[i].levels[j].index)
                }
                skills[i].levels[j].completed = tmp !== undefined;
            }

            //set done mark
            for (let j = 0; j < skills[i].levels.length; j++) {
                skills[i].all_completed = true;
                if (skills[i].levels[j].completed === false) {
                    skills[i].all_completed = false;
                    break;
                }
            }
        }

    }


    //Problem: What when level completed if skill not yet in completed_skills? (occurs on first complete)
    //Solution: on complete change send request to backend with change. user object is updated on route change.
    async function changeComplete(e, skillId, levelIndex, status) {
        e.stopPropagation();
        for (let i = 0; i < skills.length; i++) {
            if (skills[i].id === skillId) {
                for (let j = 0; j < skills[i].levels.length; j++) {
                    if (skills[i].levels[j].index === levelIndex) {
                        skills[i].levels[j].completed = status;
                        //TODO: SYNC WITH DB
                        break;
                    }
                }
                for (let j = 0; j < skills[i].levels.length; j++) {
                    if (skills[i].levels[j].completed === false) {
                        skills[i].all_completed = false;
                        return;
                    }
                }
                skills[i].all_completed = true;
            }
        }
    }

    async function changeMark(e, skillId, status) {
        e.stopPropagation();
        for (let i = 0; i < skills.length; i++) {
            if (skills[i].id === skillId) {
                skills[i].marked = status;
                //TODO: SYNC WITH DB
                break;
            }
        }
    }

</script>
<div class="container my-24" id="rootDiv">
    <div class="flex flex-row justify-between flex-wrap">
        <h1 class="text-4xl mb-8 text-gray-700 dark:text-gray-300 ml-auto">Skills</h1>
        <Input bind:value={filterQuery} class="ml-auto w-2/6 h-1/2" placeholder="Search" size="md" type="text"/>
    </div>
    {#if loading}
        <div class="text-center">
            <Spinner size={10}/>
        </div>
    {:else}
        <Accordion
                activeClasses="bg-blue-100 dark:bg-gray-700 text-blue-600 dark:text-white"
                inactiveClasses="text-gray-500 dark:text-gray-400 hover:bg-blue-100 dark:hover:bg-gray-700">
            {#each filteredSkills as skill}
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