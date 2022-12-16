<script>
    import {page} from '$app/stores';
    import {onMount} from "svelte";
    import {loadSkills, loadSpecificUser} from "$lib/apiCalls.js";
    import {Heading, Hr, Li, List, Spinner} from "flowbite-svelte";

    // get the username from the url. /profile/<username>
    let username = $page.params.username;
    let loading = true;
    let user;
    let skills;
    let preparedSkills = [];

    onMount(async () => {
        user = await loadSpecificUser(username);
        skills = await loadSkills();
        loading = false;
        //decide if user has completed a level for the skill. if yes, display it in the list, if no, do nothing
        for (let i = 0; i < skills.length; i++) {
            let levels = []
            for (let j = 0; j < skills[i].levels.length; j++) {
                let hash = (skills[i]._id.$oid + skills[i].levels[j].id).toString().hashCode();
                if (user.completed_skills.includes(hash.toString())) {
                    levels.push(skills[i].levels[j].name)
                }
            }
            preparedSkills.push({name: skills[i].name, levels: levels})

        }
    })

</script>
<div class="container mx-auto w-full sm:w-2/3 sm:mt-24 sm:mb-24 outline outline-offset-2 outline-1 outline-gray-200 dark:outline-gray-700 p-10 sm:rounded-lg">
    <h1 class="text-4xl text-center mb-2 text-gray-700 dark:text-gray-300">Profile</h1>
    <Hr class="mb-8"/>
    {#if loading}
        <div class="text-center">
            <Spinner size={10}/>
        </div>
    {:else}
        {#if user === null}
            <div class="flex flex-row gap-2 items-center justify-center">
                <p class="font-semibold text-xl text-center text-gray-700 dark:text-gray-300 border-b border-red-400">
                    User Not Found!</p>
            </div>
        {:else}
            <div class="flex flex-row gap-2 items-center">
                <p class="font-semibold text-xl text-gray-700 dark:text-gray-300">Username: </p>
                <p class="text-xl text-gray-700 dark:text-gray-300">{user.username}</p>
            </div>
            <div class="flex flex-row gap-2 items-center">
                <p class="font-semibold text-xl text-gray-700 dark:text-gray-300">Name: </p>
                <p class="text-xl text-gray-700 dark:text-gray-300">{user.name}</p>
            </div>
            <div class="flex flex-row gap-2 items-center">
                <p class="font-semibold text-xl text-gray-700 dark:text-gray-300">Email: </p>
                <p class="text-xl text-gray-700 dark:text-gray-300">{user.email}</p>
            </div>
            <div class="flex flex-row gap-2 items-center mb-6">
                <p class="font-semibold text-xl text-gray-700 dark:text-gray-300">Field: </p>
                <p class="text-xl text-gray-700 dark:text-gray-300">{user.field}</p>
            </div>
            <Heading tag="h2" customSize="text-xl font-semibold" class="text-gray-700 dark:text-gray-300">Finished
                Skills
            </Heading>
            {#if preparedSkills.length > 0}
                <List tag="ul" class="space-y-1">
                    {#each preparedSkills as skill}
                        {#if skill.levels.length > 0}
                            <Li class="mb-2">
                                {skill.name}
                                <List tag="ol" class="pl-5 mt space-y-1">
                                    {#each skill.levels as level}
                                        <Li>{level}</Li>
                                    {/each}
                                </List>
                            </Li>
                        {:else}
                        {/if}

                    {/each}
                </List>
            {:else}
                <div class="flex flex-row gap-2 items-center mb-6">
                    <p class="text-lg text-gray-700 dark:text-gray-300">no skills completed yet</p>
                </div>
            {/if}

        {/if}
    {/if}
</div>