<script>
    import {
        Button,
        ButtonGroup,
        Hr,
        Input,
        InputAddon,
        Label,
        Modal, Table,
        TableBody, TableBodyCell,
        TableBodyRow,
        TableHead,
        TableHeadCell, TableSearch
    } from "flowbite-svelte";
    import {onMount} from "svelte";
    import {isAdmin} from "$lib/stores.js";
    import {goto} from "$app/navigation";

    let showNewUserPw = false;
    let editUserModal = false;

    onMount(async () => {
        if (!$isAdmin) {
            goto("/login")
        }
    })

    let userSearchTerm = '';
    let items = [
        {id: 1, maker: 'Toyota', type: 'ABC', make: 2017},
        {id: 2, maker: 'Ford', type: 'CDE', make: 2018},
        {id: 3, maker: 'Volvo', type: 'FGH', make: 2019},
        {id: 4, maker: 'Saab', type: 'IJK', make: 2020}
    ];
    $: filteredUsers = items.filter(
        (item) => item.maker.toLowerCase().indexOf(userSearchTerm.toLowerCase()) !== -1
    );
</script>

<div class="container mx-auto w-full sm:w-2/3 my-24 outline outline-offset-2 outline-1 outline-gray-200  dark:outline-gray-700 p-10 rounded-lg">
    <div>
        <h1 class="text-4xl text-center mb-8 text-gray-700 dark:text-gray-300">Admin Panel</h1>
        <h2 class="text-2xl text-gray-700 dark:text-gray-300">User Management</h2>
        <Hr class="mb-4"/>
        <h2 class="text-xl text-gray-700 dark:text-gray-300 mb-8">Create New User</h2>
        <Label class="space-y-2 mb-5">
            <span>Username</span>
            <Input placeholder="Peter" size="md" type="email"/>
        </Label>
        <Label class="mb-2 dark:text-gray-400" for="show-password">Password</Label>
        <ButtonGroup class="w-full">
            <InputAddon>
                <button on:click={() => (showNewUserPw = !showNewUserPw)}>
                    {#if showNewUserPw}
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                             stroke="currentColor" class="w-5 h-5">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                  d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.638 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.638 0-8.573-3.007-9.963-7.178z"/>
                            <path stroke-linecap="round" stroke-linejoin="round"
                                  d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                        </svg>
                    {:else}
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
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
        <div class="flex flex-row gap-4">
            <Button class="mt-5" color="green" gradient shadow="green">Create</Button>
            <Button class="mt-5" color="blue" gradient on:click={()=>{editUserModal= true}} shadow="blue">
                Show All Users
            </Button>
        </div>
        <Modal bind:open={editUserModal} title="Edit Users">
            <TableSearch bind:inputValue={userSearchTerm} hoverable={true} placeholder="Search by username">
                <TableHead>
                    <TableHeadCell>ID</TableHeadCell>
                    <TableHeadCell>Maker</TableHeadCell>
                    <TableHeadCell>Type</TableHeadCell>
                    <TableHeadCell>Make</TableHeadCell>
                </TableHead>
                <TableBody class="divide-y">
                    {#each filteredUsers as item}
                        <TableBodyRow>
                            <TableBodyCell>{item.id}</TableBodyCell>
                            <TableBodyCell>{item.maker}</TableBodyCell>
                            <TableBodyCell>{item.type}</TableBodyCell>
                            <TableBodyCell>{item.make}</TableBodyCell>
                        </TableBodyRow>
                    {/each}
                </TableBody>
            </TableSearch>
            <svelte:fragment slot='footer'>
                <Button>Close</Button>
            </svelte:fragment>
        </Modal>
    </div>
</div>