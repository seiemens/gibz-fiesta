<script>
    import {
        Button,
        ButtonGroup,
        Hr,
        Input,
        InputAddon,
        Label,
        Modal,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        TableHead,
        TableHeadCell,
        TableSearch,
        Toggle
    } from "flowbite-svelte";
    import {onMount} from "svelte";
    import {isAdmin} from "$lib/stores.js";
    import {goto} from "$app/navigation";

    let allUsers = []
    let jobFields = []

    let showNewUserPw = false;
    let listUserModal = false;
    let editUserModal = false;
    let createUserIsAdmin = false;

    onMount(async () => {
        if (!$isAdmin) {
            goto("/login")
            return;
        }
        allUsers = await loadAllUsers();
        let x = await loadJobFields();
        jobFields = x.fields;
    })

    async function loadJobFields() {
        const response = await fetch('/testJobFieldData.json', {
            method: 'GET',
            headers: {
                'content-type': 'application/json'
            }
        });
        return await response.json();
    }

    async function loadAllUsers() {
        const response = await fetch('/testUserData.json', {
            method: 'GET',
            headers: {
                'content-type': 'application/json'
            }
        });
        return await response.json();
    }

    let userSearchTerm = '';
    let filteredUsers = [];

    $: try {
        filteredUsers = allUsers.users.filter(
            (item) => item.name.toLowerCase().indexOf(userSearchTerm.toLowerCase()) !== -1
        );
    } catch (e) {
    }

    let userToEdit = null;

    function openEditUserModal(user) {
        userToEdit = user;
        editUserModal = true;
    }

    function closeEditUserModal() {
        editUserModal = false;
        $: try {
            filteredUsers = allUsers.users.filter(
                (item) => item.name.toLowerCase().indexOf(userSearchTerm.toLowerCase()) !== -1
            );
        } catch (e) {
        }

        //TODO: EDIT IN DB
    }

    function deleteUser(email) {
        for (let i = 0; i < allUsers.users.length; i++) {
            if (allUsers.users[i].email == email) {
                allUsers.users.splice(i, 1);
                $: try {
                    filteredUsers = allUsers.users.filter(
                        (item) => item.name.toLowerCase().indexOf(userSearchTerm.toLowerCase()) !== -1
                    );
                } catch (e) {
                }
                //TODO: DELETE FROM DB
                break;
            }
        }

    }


</script>

<div class="container mx-auto w-full sm:w-2/3 sm:mt-24 sm:mb-24 outline outline-offset-2 outline-1 outline-gray-200 dark:outline-gray-700 p-10 sm:rounded-lg">
    <div>
        <h1 class="text-4xl text-center mb-8 text-gray-700 dark:text-gray-300">Admin Panel</h1>
        <h2 class="text-2xl text-gray-700 dark:text-gray-300">User Management</h2>
        <Hr class="mb-4"/>
        <h2 class="text-xl text-gray-700 dark:text-gray-300 mb-8">Create New User</h2>
        <div class="grid gap-6 mb-6 md:grid-cols-2">
            <div>
                <Label class="space-y-2">
                    <span>Username</span>
                    <Input placeholder="Peter" size="md" type="text"/>
                </Label>
            </div>
            <div>
                <Label class="space-y-2 min-w-min">
                    <span>E-Mail</span>
                    <Input placeholder="peter@example.com" size="md" type="email"/>
                </Label>
            </div>
            <div>
                <Label class="mb-2 dark:text-gray-400" for="show-password">Password</Label>
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
                    <Input id="show-password" placeholder="{showNewUserPw ? 'passw0rd' : '********'}"
                           type={showNewUserPw ? 'text' : 'password'}/>
                </ButtonGroup>
            </div>
            <div class="flex gap-4">
                <div>
                    <Label class="mb-2 dark:text-gray-400">Job Field</Label>
                    <select class="block max-w-[9rem] min-w-[9rem] disabled:cursor-not-allowed disabled:opacity-50 focus:border-blue-500 focus:ring-blue-500 dark:focus:border-blue-500 dark:focus:ring-blue-500 bg-gray-50 text-gray-900 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 border-gray-300 dark:border-gray-600 p-2.5 text-sm rounded-lg">
                        {#each jobFields as j}
                            <option value="{j.job}">{j.job}</option>
                        {/each}
                    </select>
                </div>
                <div>
                    <Label class="mb-4 dark:text-gray-400">Is Admin</Label>
                    <Toggle bind:checked={createUserIsAdmin} color="green">{createUserIsAdmin ? "Yes" : "No"}</Toggle>
                </div>
            </div>
        </div>
        <div class="flex flex-row gap-4">
            <Button class="mt-5" color="green" gradient shadow="green">Create</Button>
            <Button class="mt-5" color="blue" gradient on:click={()=>{listUserModal= true}} shadow="blue">
                Show All Users
            </Button>
        </div>
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
                            <TableBodyCell>{item.username}</TableBodyCell>
                            <TableBodyCell>{item.name}</TableBodyCell>
                            <TableBodyCell>{item.email}</TableBodyCell>
                            <TableBodyCell>{item.field}</TableBodyCell>
                            <TableBodyCell>{item.role === 1 ? "Yes" : "No"}</TableBodyCell>
                            <TableBodyCell>
                                <Button gradient shadow="blue" color="blue" class="scale-75"
                                        on:click={()=>{openEditUserModal(item)}}><i class="material-icons">edit</i>
                                </Button>
                                <Button gradient shadow="red" color="red" class="scale-75"
                                        on:click={()=>deleteUser(item.email)}><i
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
                        <Input bind:value={userToEdit.username} placeholder="Peter" size="md" type="text"/>
                    </Label>
                </div>
                <div>
                    <Label class="space-y-2 min-w-min">
                        <span>E-Mail</span>
                        <Input bind:value={userToEdit.email} placeholder="peter@example.com" size="md" type="email"/>
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
                               type={showNewUserPw ? 'text' : 'password'}/>
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
                        <Toggle bind:checked={userToEdit.role} color="green">{createUserIsAdmin ? "Yes" : "No"}</Toggle>
                    </div>
                </div>
            </div>
            <svelte:fragment slot='footer'>
                <Button on:click={()=>{closeEditUserModal()}}>Save</Button>
            </svelte:fragment>
        </Modal>
    </div>
</div>