export const apiURL = "http://127.0.0.1:4200"

export async function loadJobFields() {
    const response = await fetch('/testJobFieldData.json', {
        method: 'GET',
        headers: {
            'content-type': 'application/json'
        }
    });
    return await response.json();
}

export async function loadAllUsers() {
    const response = await fetch(apiURL+"/user/all", {
        method: 'GET',
        credentials: 'include'
    });
    return await response.json();
}

export async function loadSkills() {
    const response = await fetch('/testdata.json', {
        method: 'GET',
        headers: {
            'content-type': 'application/json'
        }
    });
    return await response.json();
}

export async function loadSpecificUser(username) {
    let allUsers = await loadAllUsers();
    for (let i = 0; i < allUsers.length; i++) {
        if (allUsers[i].username === username) {
            return allUsers[i];
        }
    }
    return null;
}

export async function createUser(userData) {
    console.log(JSON.stringify(userData))
    const response = await fetch(apiURL + '/user/create', {
        method: 'POST',
        credentials: 'include',
        body: JSON.stringify(userData),

    });
    return await response.json();
}

export async function login(username, password) {
    return await fetch(apiURL + '/user/login', {
        method: 'POST',
        credentials: 'include',
        body: JSON.stringify({username, password}),

    });
}

export async function logout() {
    return await fetch(apiURL + '/user/logout', {
        method: 'GET',
        credentials: 'include'
    });
}

export async function checkAuth() {
    return await fetch(apiURL + '/user/auth', {
        method: 'GET',
        credentials: 'include'
    });
}

export async function deleteUserDb(username) {
    return await fetch(apiURL + '/user/delete', {
        method: 'POST',
        credentials: 'include',
        body: JSON.stringify({username})
    });
}

export async function editUser(username, password) {
    return await fetch(apiURL + '/user/update', {
        method: 'POST',
        credentials: 'include',
        body: JSON.stringify({username, password})
    });
}