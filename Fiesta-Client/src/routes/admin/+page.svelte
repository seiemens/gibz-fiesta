<script>
    import {
        Accordion,
        AccordionItem,
        Button,
        ButtonGroup,
        Hr,
        Input,
        InputAddon,
        Label,
        Modal,
        Spinner,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        TableHead,
        TableHeadCell,
        TableSearch,
        Textarea,
        Toggle
    } from "flowbite-svelte";
    import {onMount} from "svelte";
    import {isAdmin} from "$lib/stores.js";
    import {beforeNavigate, goto} from "$app/navigation";
    import {hideAccordion} from "$lib/utils.js";
    import {loadAllUsers, loadJobFields, loadSkills} from "$lib/apiCalls.js";
    import {createSkill, createUser, deleteSkillDb, deleteUserDb, editUser} from "../../lib/apiCalls.js";
    import {isLoggedIn, user} from "../../lib/stores.js";
    import {checkSignIn} from "../../lib/utils.js";

    let allUsers = []
    let jobFields = []
    let skills = [];

    let showNewUserPw = false;
    let listUserModal = false;
    let editUserModal = false;
    let loading = true;

    beforeNavigate(({from, to}) => {
        hideAccordion(from, to)
    })

    onMount(async () => {
        if ($user === undefined)
            $user = await checkSignIn();

        if (!$isLoggedIn) {
            goto("/skills")
            return;
        }
        if (!$isAdmin) {
            goto("/")
            return;
        }

        allUsers = await loadAllUsers();
        let x = await loadJobFields();
        jobFields = x.fields;
        skills = await loadSkills();
        loading = false;
    })


    let userSearchTerm = '';
    let filteredUsers = [];

    $: try {
        filteredUsers = allUsers.filter(
            (item) => item.name.toLowerCase().indexOf(userSearchTerm.toLowerCase()) !== -1
        );
    } catch (e) {
    }

    let userToEdit = null;
    let userToEditPw = "";

    function openEditUserModal(user) {
        userToEdit = user;
        userToEditPw = "";
        editUserModal = true;
    }

    function closeEditUserModal() {
        editUserModal = false;
        try {
            filteredUsers = allUsers.filter(
                (item) => item.name.toLowerCase().indexOf(userSearchTerm.toLowerCase()) !== -1
            );
        } catch (e) {
        }
        userToEdit.password = userToEditPw;
        editUser(userToEdit).then((res) => {
        })
    }

    async function deleteUser(item) {
        // workaround for problem in backend with this particular route
        item.username = ""
        item.password = ""
        deleteUserDb(item).then((res) => {
            if (res.status === 202) {
                allUsers = allUsers.filter((x) => x != item)
                try {
                    filteredUsers = allUsers.filter(
                        (item) => item.name.toLowerCase().indexOf(userSearchTerm.toLowerCase()) !== -1
                    );
                } catch (e) {
                }
            }
        });
    }

    let createNewUserData = {
        "username": "",
        "name": "",
        "email": "",
        "field": "",
        "role": 0,
        "password": "",
        "active": true
    }

    async function createNewUser(e) {
        e.preventDefault()
        if (createNewUserData.role) {
            createNewUserData.role = 1;
        } else {
            createNewUserData.role = 0;
        }

        allUsers.push(createNewUserData);
        await createUser(createNewUserData);

        try {
            filteredUsers = allUsers.filter(
                (item) => item.name.toLowerCase().indexOf(userSearchTerm.toLowerCase()) !== -1
            );
        } catch (e) {
        }

        createNewUserData = {
            "username": "",
            "name": "",
            "email": "",
            "field": "",
            "role": 0,
            "password": "",
            "active": true
        }
    }

    async function deleteSkill(e, skill) {
        e.stopPropagation();
        deleteSkillDb(skill).then((res) => {
            if (res.status === 202) {
                skills = skills.filter((x) => x != skill)
            }
        });
    }


    async function deleteLevel(e, skillId, levelIndex) {
        e.stopPropagation();
        for (let i = 0; i < skills.length; i++) {
            if (skills[i].display_id === skillId) {
                for (let j = 0; j < skills[i].levels.length; j++) {
                    if (skills[i].levels[j].id === levelIndex) {
                        skills[i].levels.splice(j, 1);
                        skills = skills;
                        break;
                    }
                }
            }
        }
    }

    async function deleteResource(skillId, levelIndex, resId) {
        for (let i = 0; i < skills.length; i++) {
            if (skills[i].display_id === skillId) {
                for (let j = 0; j < skills[i].levels.length; j++) {
                    if (skills[i].levels[j].id === levelIndex) {
                        for (let l = 0; l < skills[i].levels[j].resources.length; l++) {
                            if (skills[i].levels[j].resources[l].id === resId) {
                                skills[i].levels[j].resources.splice(l, 1);
                                skills = skills;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    function saveChangesSkill(e, skill) {
        e.stopPropagation();
        createSkill(skill).then((res) => {
            if (res.status === 200) {
                res.json().then((json) => {
                    skill._id = {}
                    skill._id.$oid = json.insertedId.$oid;
                })
            }
        });
    }

    let newResourceData = {
        "id": -1,
        "name": "",
        "url": ""
    }

    async function addNewResource(skillId, levelIndex) {
        for (let i = 0; i < skills.length; i++) {
            if (skills[i].display_id === skillId) {
                for (let j = 0; j < skills[i].levels.length; j++) {
                    if (skills[i].levels[j].id === levelIndex) {
                        newResourceData.id = skills[i].levels[j].resources.length
                        skills[i].levels[j].resources.push(structuredClone(newResourceData))
                        //yes looks stupid but else svelte does not update the accordion
                        skills = skills;
                        isNewResourceModalOpen = false;
                        newResourceData = {
                            "id": -1,
                            "name": "",
                            "url": ""
                        }
                        break;
                    }
                }
            }
        }
    }

    let newLevelData = {
        "id": -1,
        "name": "",
        "description": "",
        "resources": []
    }

    async function addNewlevel(skillId) {
        for (let i = 0; i < skills.length; i++) {
            if (skills[i].display_id === skillId) {
                newLevelData.id = skills[i].levels.length.toString()
                skills[i].levels.push(structuredClone(newLevelData))
                //yes looks stupid but else svelte does not update the accordion
                skills = skills;
                break;
            }
        }

    }

    let newSkillData = {
        "display_id": -1,
        "name": "",
        "levels": []
    }
    let resourceParent = {
        "skill": 0,
        "level": 0
    }

    async function addNewSkill() {
        newSkillData.display_id = skills.length
        skills.push(structuredClone(newSkillData))
        //yes looks stupid but else svelte does not update the accordion
        skills = skills;
    }

    let isNewResourceModalOpen = false;

    function openNewResourceModal(skillId, levelIndex) {
        isNewResourceModalOpen = true;
        resourceParent.skill = skillId
        resourceParent.level = levelIndex
    }

    function closeNewResourceModal() {
        isNewResourceModalOpen = false;
        addNewResource(resourceParent.skill, resourceParent.level)
    }

</script>

<div class="container mx-auto w-full sm:w-2/3 sm:mt-24 sm:mb-24 outline outline-offset-2 outline-1 outline-gray-200 dark:outline-gray-700 p-10 sm:rounded-lg"
     id="rootDiv">
    <div>
        <h1 class="text-4xl text-center mb-8 text-gray-700 dark:text-gray-300">Admin Panel</h1>
        {#if loading}
            <div class="text-center">
                <Spinner size={10}/>
            </div>
        {:else}
            <!--<editor-fold desc="User Management">-->
            <h2 class="text-2xl text-gray-700 dark:text-gray-300">User Management</h2>
            <Hr class="mb-4"/>
            <h2 class="text-xl text-gray-700 dark:text-gray-300 mb-8">Create New User</h2>
            <form class="grid gap-6 mb-6 md:grid-cols-2" on:submit={(e)=>{createNewUser(e)}}>
                <div>
                    <Label class="space-y-2">
                        <span>Username</span>
                        <Input bind:value={createNewUserData.username} placeholder="johnny" size="md" type="text"
                               required/>
                    </Label>
                </div>
                <div>
                    <Label class="space-y-2">
                        <span>Name</span>
                        <Input bind:value={createNewUserData.name} placeholder="John Doe" size="md" type="text"
                               required/>
                    </Label>
                </div>
                <div>
                    <Label class="space-y-2 min-w-min">
                        <span>E-Mail</span>
                        <Input bind:value={createNewUserData.email} placeholder="john.doe@example.com" size="md"
                               type="email" required/>
                    </Label>
                </div>
                <div>
                    <Label class="mb-2 dark:text-gray-400" for="show-password">Password</Label>
                    <ButtonGroup class="w-full">
                        <InputAddon>
                            <button on:click={(e) => {e.preventDefault(); showNewUserPw = !showNewUserPw;}}>
                                {#if showNewUserPw}
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                                         stroke-width="1.5"
                                         stroke="currentColor" class="w-5 h-5">
                                        <path stroke-linecap="round" stroke-linejoin="round"
                                              d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z"/>
                                        <path stroke-linecap="round" stroke-linejoin="round"
                                              d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                                    </svg>
                                {:else}
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                                         stroke-width="1.5"
                                         stroke="currentColor" class="w-5 h-5">
                                        <path stroke-linecap="round" stroke-linejoin="round"
                                              d="M3.98 8.223A10.477 10.477 0 001.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.45 10.45 0 0112 4.5c4.756 0 8.773 3.162 10.065 7.498a10.523 10.523 0 01-4.293 5.774M6.228 6.228L3 3m3.228 3.228l3.65 3.65m7.894 7.894L21 21m-3.228-3.228l-3.65-3.65m0 0a3 3 0 10-4.243-4.243m4.242 4.242L9.88 9.88"/>
                                    </svg>
                                {/if}
                            </button>
                        </InputAddon>
                        <Input id="show-password" placeholder="{showNewUserPw ? 'passw0rd' : '********'}"
                               type={showNewUserPw ? 'text' : 'password'} bind:value={createNewUserData.password}
                               required/>
                    </ButtonGroup>
                </div>
                <div class="flex gap-4">
                    <div>
                        <Label class="mb-2 dark:text-gray-400">Job Field</Label>
                        <select bind:value={createNewUserData.field}
                                class="block max-w-[9rem] min-w-[9rem] disabled:cursor-not-allowed disabled:opacity-50 focus:border-blue-500 focus:ring-blue-500 dark:focus:border-blue-500 dark:focus:ring-blue-500 bg-gray-50 text-gray-900 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 border-gray-300 dark:border-gray-600 p-2.5 text-sm rounded-lg">
                            {#each jobFields as j}
                                <option value="{j.job}">{j.job}</option>
                            {/each}
                        </select>
                    </div>
                    <div>
                        <Label class="mb-4 dark:text-gray-400">Is Admin</Label>
                        <Toggle bind:checked={createNewUserData.role}
                                color="green">{createNewUserData.role ? "Yes" : "No"}</Toggle>
                    </div>
                    <div>
                        <Label class="mb-4 dark:text-gray-400">Is Active</Label>
                        <Toggle bind:checked={createNewUserData.active}
                                color="green">{createNewUserData.active ? "Yes" : "No"}</Toggle>
                    </div>
                </div>
                <div></div>
                <div class="flex flex-row gap-4">
                    <Button class="mt-5" color="green" gradient type="submit" shadow="green">Create</Button>
                    <Button class="mt-5" color="blue" gradient on:click={()=>{listUserModal= true}} shadow="blue">
                        Show All Users
                    </Button>
                </div>
            </form>

            <Modal autoclose={false} bind:open={listUserModal} size="xl" title="User List">
                <TableSearch bind:inputValue={userSearchTerm} hoverable={true} placeholder="Search by username">
                    <TableHead>
                        <TableHeadCell>Username</TableHeadCell>
                        <TableHeadCell>Name</TableHeadCell>
                        <TableHeadCell>Email</TableHeadCell>
                        <TableHeadCell>Job Field</TableHeadCell>
                        <TableHeadCell>Admin</TableHeadCell>
                        <TableHeadCell>Actions</TableHeadCell>
                    </TableHead>
                    <TableBody class="divide-y">
                        {#each filteredUsers as item}
                            <TableBodyRow>
                                <TableBodyCell
                                        class="{item.active ? '': 'text-gray-400 dark:text-gray-500'}">{item.username}</TableBodyCell>
                                <TableBodyCell
                                        class="{item.active ? '': 'text-gray-400 dark:text-gray-500'}">{item.name}</TableBodyCell>
                                <TableBodyCell
                                        class="{item.active ? '': 'text-gray-400 dark:text-gray-500'}">{item.email}</TableBodyCell>
                                <TableBodyCell
                                        class="{item.active ? '': 'text-gray-400 dark:text-gray-500'}">{item.field}</TableBodyCell>
                                <TableBodyCell
                                        class="{item.active ? '': 'text-gray-400 dark:text-gray-500'}">{item.role ? "Yes" : "No"}</TableBodyCell>
                                <TableBodyCell class="{item.active ? '': 'text-gray-400 dark:text-gray-500'}">
                                    <Button gradient shadow="blue" color="blue" class="scale-75"
                                            on:click={()=>{openEditUserModal(item)}}><i class="material-icons">edit</i>
                                    </Button>
                                    <Button gradient shadow="red" color="red" class="scale-75"
                                            on:click={()=>deleteUser(item)}><i
                                            class="material-icons">delete_forever</i></Button>
                                </TableBodyCell>
                            </TableBodyRow>
                        {/each}
                    </TableBody>
                </TableSearch>
                <svelte:fragment slot='footer'>
                    <Button on:click={()=>listUserModal=false}>Close</Button>
                </svelte:fragment>
            </Modal>
            <Modal autoclose={false} bind:open={editUserModal} size="lg"
                   title="Edit User [{userToEdit !== null?userToEdit.name:''}]">
                <div class="grid gap-6 mb-6 md:grid-cols-2">
                    <div>
                        <Label class="space-y-2">
                            <span>Username</span>
                            <Input bind:value={userToEdit.username} placeholder="Peter.m" size="md" type="text"/>
                        </Label>
                    </div>
                    <div>
                        <Label class="space-y-2">
                            <span>Name</span>
                            <Input bind:value={userToEdit.name} placeholder="Peter Meier" size="md" type="text"/>
                        </Label>
                    </div>
                    <div>
                        <Label class="space-y-2 min-w-min">
                            <span>E-Mail</span>
                            <Input bind:value={userToEdit.email} placeholder="peter@example.com" size="md"
                                   type="email"/>
                        </Label>
                    </div>
                    <div>
                        <Label class="mb-2 dark:text-gray-400" for="show-password-edit">Password</Label>
                        <ButtonGroup class="w-full">
                            <InputAddon>
                                <button on:click={() => (showNewUserPw = !showNewUserPw)}>
                                    {#if showNewUserPw}
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                                             stroke-width="1.5"
                                             stroke="currentColor" class="w-5 h-5">
                                            <path stroke-linecap="round" stroke-linejoin="round"
                                                  d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z"/>
                                            <path stroke-linecap="round" stroke-linejoin="round"
                                                  d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                                        </svg>
                                    {:else}
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                                             stroke-width="1.5"
                                             stroke="currentColor" class="w-5 h-5">
                                            <path stroke-linecap="round" stroke-linejoin="round"
                                                  d="M3.98 8.223A10.477 10.477 0 001.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.45 10.45 0 0112 4.5c4.756 0 8.773 3.162 10.065 7.498a10.523 10.523 0 01-4.293 5.774M6.228 6.228L3 3m3.228 3.228l3.65 3.65m7.894 7.894L21 21m-3.228-3.228l-3.65-3.65m0 0a3 3 0 10-4.243-4.243m4.242 4.242L9.88 9.88"/>
                                        </svg>
                                    {/if}
                                </button>
                            </InputAddon>
                            <Input id="show-password-edit"
                                   placeholder="{showNewUserPw ? 'cannot view password, only change' : '********'}"
                                   type={showNewUserPw ? 'text' : 'password'} bind:value={userToEditPw}/>
                        </ButtonGroup>
                    </div>
                    <div class="flex gap-4">
                        <div>
                            <Label class="mb-2 dark:text-gray-400">Job Field</Label>
                            <select bind:value={userToEdit.field}
                                    class="block max-w-[9rem] min-w-[9rem] disabled:cursor-not-allowed disabled:opacity-50 focus:border-blue-500 focus:ring-blue-500 dark:focus:border-blue-500 dark:focus:ring-blue-500 bg-gray-50 text-gray-900 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 border-gray-300 dark:border-gray-600 p-2.5 text-sm rounded-lg">
                                {#each jobFields as j}
                                    <option value="{j.job}">{j.job}</option>
                                {/each}
                            </select>
                        </div>
                        <div>
                            <Label class="mb-4 dark:text-gray-400">Is Admin</Label>
                            <Toggle bind:checked={userToEdit.role}
                                    color="green">{userToEdit.role ? "Yes" : "No"}</Toggle>
                        </div>
                        <div>
                            <Label class="mb-4 dark:text-gray-400">Is Active</Label>
                            <Toggle bind:checked={userToEdit.active}
                                    color="green">{userToEdit.active ? "Yes" : "No"}</Toggle>
                        </div>
                    </div>
                </div>
                <svelte:fragment slot='footer'>
                    <Button on:click={()=>{closeEditUserModal()}}>Save</Button>
                </svelte:fragment>
            </Modal>
            <!--</editor-fold>-->
            <h2 class="text-2xl text-gray-700 dark:text-gray-300 mt-8">Skills Management</h2>
            <Hr class="mb-4"/>
            <Accordion
                    activeClasses="bg-blue-100 dark:bg-gray-700 text-blue-600 dark:text-white"
                    inactiveClasses="text-gray-500 dark:text-gray-400 hover:bg-blue-100 dark:hover:bg-gray-700">
                {#each skills as skill}
                    <AccordionItem>
                        <div slot="header" class="flex items-center w-full">
                            <Input bind:value={skill.name} size="sm" class="w-1/2" type="text"
                                   placeholder="Skill Title" on:click={(e)=>{e.stopPropagation()}}/>
                            <div class="ml-auto scale-75 gap-4 flex flex-row">
                                <Button gradient color="blue" shadow="blue" on:click={(e)=>{saveChangesSkill(e,skill)}}>
                                    Save Skill
                                </Button>
                                <Button gradient color="red" shadow="red" on:click={(e)=>{deleteSkill(e,skill)}}>
                                    Delete
                                </Button>
                            </div>
                        </div>
                        <Accordion
                                activeClasses="bg-blue-100 dark:bg-gray-700 text-blue-600 dark:text-white"
                                inactiveClasses="text-gray-500 dark:text-gray-400 hover:bg-blue-100 dark:hover:bg-gray-700">
                            {#each skill.levels as level}
                                <AccordionItem>
                                    <div slot="header" class="flex items-center w-full">
                                        <Input bind:value={level.name} size="sm" class="w-1/2" type="text"
                                               placeholder="Level Title" on:click={(e)=>{e.stopPropagation()}}/>
                                        <ButtonGroup class="ml-auto scale-75">
                                            <Button gradient shadow="red" color="red"
                                                    on:click={(e)=>{deleteLevel(e,skill.display_id, level.id)}}>Delete
                                            </Button>
                                        </ButtonGroup>
                                    </div>
                                    <Textarea rows="4" bind:value={level.description} placeholder="Level Description"/>
                                    <Hr class="my-8" height="h-px"/>
                                    {#each level.resources as resource}
                                        <ButtonGroup class="ml-2">
                                            <Button gradient color="purpleToBlue" shadow="blue" href="{resource.url}"
                                                    class="mb-2">{resource.name}</Button>
                                            <Button gradient shadow="red" color="red" class="mb-2"
                                                    on:click={()=>{deleteResource(skill.display_id,level.id,resource.id)}}>
                                                Delete
                                            </Button>
                                        </ButtonGroup>
                                    {/each}
                                    <Button gradient color="green" shadow="green" class="ml-2 mb-2"
                                            on:click={()=>{openNewResourceModal(skill.display_id, level.id)}}>Add New
                                        Resource
                                    </Button>
                                </AccordionItem>
                            {/each}
                        </Accordion>
                        <div class="flex justify-items-center">
                            <Button gradient color="green" shadow="green" class="mx-auto mt-4"
                                    on:click={()=>{addNewlevel(skill.display_id)}}>Add New Level
                            </Button>
                        </div>
                    </AccordionItem>
                {/each}
            </Accordion>
            <div class="flex justify-items-center">
                <Button gradient color="green" shadow="green" class="mx-auto mt-4" on:click={()=>{addNewSkill()}}>Add
                    New Skill
                </Button>
            </div>

            <Modal bind:open={isNewResourceModalOpen} size="lg"
                   title="">
                <div class="grid gap-6 mb-6 md:grid-cols-2">
                    <div>
                        <Label class="space-y-2 min-w-min">
                            <span>Display Name</span>
                            <Input bind:value={newResourceData.name} placeholder="PDF File" size="md"
                                   type="text"/>
                        </Label>
                    </div>
                    <div>
                        <Label class="space-y-2 min-w-min">
                            <span>Link</span>
                            <Input bind:value={newResourceData.url} placeholder="https://example.com" size="md"
                                   type="text"/>
                        </Label>
                    </div>
                </div>
                <svelte:fragment slot='footer'>
                    <Button on:click={()=>{closeNewResourceModal()}}>Save</Button>
                </svelte:fragment>
            </Modal>
        {/if}
    </div>
</div>