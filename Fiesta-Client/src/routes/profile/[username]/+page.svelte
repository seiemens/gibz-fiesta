<script>
    import {page} from '$app/stores';
    import {onMount} from "svelte";
    import {loadSkills, loadSpecificUser} from "$lib/apiCalls.js";
    import {Heading, Hr, Li, List, Spinner} from "flowbite-svelte";


    let username = $page.params.username;
    let loading = true;
    let user;
    let skills;

    onMount(async () => {
        //TODO: DONT LOAD AUTH TOKEN!!! INSECURE AF
        user = await loadSpecificUser(username);
        skills = (await loadSkills()).skills;
        loading = false;
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
                <p class="font-semibold text-xl text-center text-gray-700 dark:text-gray-300 border-b border-red-400">User Not Found!</p>
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
            <Heading tag="h2" customSize="text-xl font-semibold" class="text-gray-700 dark:text-gray-300">Finished Skills</Heading>
            <List tag="ul" class="space-y-1" >
                {#each user.completed_skills as skill}
                    <Li class="mb-2">
<!--                        get display name from skill from id from skill-->
                        {skills.filter((item) => item.id === skill.id)[0].display_name}
                        <List tag="ol" class="pl-5 mt space-y-1">
                            {#each skill.levels as level}
<!--                                get display name from index from level-->
                                <Li>{skills.filter((item) => item.id === skill.id)[0].levels.filter((item) => item.index === level.index)[0].display_name}</Li>
                            {/each}
                        </List>
                    </Li>
                {/each}
            </List>
        {/if}
    {/if}
</div>