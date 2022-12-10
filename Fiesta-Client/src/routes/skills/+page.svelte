<script>
    import {Accordion, AccordionItem, Button, Hr, Input, Spinner} from "flowbite-svelte";
    import {onMount} from "svelte";
    import {beforeNavigate, goto} from "$app/navigation";
    import {isLoggedIn} from "$lib/stores.js";
    import {hideAccordion} from "$lib/utils.js";
    import {loadSkills} from "$lib/apiCalls.js";
    import {user} from "../../lib/stores.js";
    import {checkSignIn} from "../../lib/utils.js";
    import {markSkill} from "../../lib/apiCalls.js";
    import {completeSkill} from "../../lib/apiCalls";

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
        skills = await loadSkills();
        setupMarkings();
        loading = false;
    })

    //fix accordion being visible for 1s after navigating
    beforeNavigate(({from, to}) => {
        hideAccordion(from, to)
    })

    $: try {
        filteredSkills = skills.filter(
            (item) => item.name.toLowerCase().indexOf(filterQuery.toLowerCase()) !== -1
        );
    } catch (e) {
    }

    function setupMarkings() {
        for (let i = 0; i < skills.length; i++) {
            //set markings
            let tmp;
            tmp = $user.marked_skills.find((item) => item.$oid === skills[i]._id.$oid);
            skills[i].marked = tmp !== undefined;

            //set completed levels
            for (let j = 0; j < skills[i].levels.length; j++) {
                let hash = (skills[i]._id.$oid + skills[i].levels[j].id).toString().hashCode();
                skills[i].levels[j].completed = !!$user.completed_skills.includes(hash.toString());
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
            if (skills[i].display_id === skillId) {
                for (let j = 0; j < skills[i].levels.length; j++) {
                    if (skills[i].levels[j].id === levelIndex) {
                        skills[i].levels[j].completed = status;
                        let hash = (skills[i]._id.$oid + levelIndex).toString().hashCode();
                        completeSkill(hash);
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

    async function changeMark(e, skill, status) {
        e.stopPropagation();
        markSkill(skill).then((res) => {
            if (res.status === 202) {
                for (let i = 0; i < skills.length; i++) {
                    if (skills[i].display_id === skill.display_id) {
                        skills[i].marked = status;
                        break;
                    }
                }
            }
        });
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
                                <span class="px-2 text-white rounded bg-yellow-300 dark:bg-yellow-400">{skill.name}</span>
                            {:else}
                                {skill.name}
                            {/if}
                            {#if skill.all_completed}
                                <span class="px-2 text-white rounded bg-green-500 dark:bg-green-700">Done!</span>
                            {/if}
                        </p>
                        <div class="ml-auto">
                            {#if skill.marked}
                                <Button outline color="yellow" class="scale-75" on:click={(e)=>{changeMark(e,skill,false)}}>Unmark</Button>
                            {:else}
                                <Button outline color="yellow" class="scale-75" on:click={(e)=>{changeMark(e,skill,true)}}>Mark</Button>
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
                                            <span class="px-2 text-white rounded bg-green-500 dark:bg-green-700">{level.name}</span>
                                        {:else}
                                            {level.name}
                                        {/if}
                                    </p>
                                    <div class="ml-auto">
                                        {#if level.completed}
                                            <Button gradient shadow="red" color="red" class="scale-75" on:click={(e)=>{changeComplete(e,skill.display_id, level.id, false)}}>Uncomplete</Button>
                                        {:else}
                                            <Button gradient shadow="green" color="green" class="scale-75" on:click={(e)=>{changeComplete(e,skill.display_id, level.id, true)}}>Complete</Button>

                                        {/if}
                                    </div>
                                </div>
                                <p class="mb-2 text-gray-500 dark:text-gray-400">{level.description}</p>
                                <Hr class="my-8" height="h-px"/>
                                {#each level.resources as resource}
                                    <Button gradient color="purpleToBlue" shadow="blue" href="{resource.url}" class="ml-2 mb-2">{resource.name}</Button>
                                {/each}
                            </AccordionItem>
                        {/each}
                    </Accordion>
                </AccordionItem>
            {/each}
        </Accordion>
    {/if}
</div>