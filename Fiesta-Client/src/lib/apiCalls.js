export const apiURL = "http://127.0.0.1:4200"

export async function loadJobFields() {
    const response = await fetch('/testJobFieldData.json', {
        method: 'GET',
        headers: {
            'content-type': 'application/json'
        }
    });
    if(response.status===500)
        return {}
    return await response.json();
}

export async function loadAllUsers() {
    const response = await fetch(apiURL+"/user/all", {
        method: 'GET',
        credentials: 'include'
    });
    if(response.status===500)
        return {}
    return await response.json();
}

export async function loadSkills() {
    const response = await fetch(apiURL+"/skill/all", {
        method: 'GET',
    });
    if(response.status===500)
        return {}
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
    const response = await fetch(apiURL + '/user/create', {
        method: 'POST',
        credentials: 'include',
        body: JSON.stringify(userData),

    });
    if(response.status===500)
        return {}
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

export async function deleteUserDb(user) {
    return await fetch(apiURL + '/user/delete', {
        method: 'POST',
        credentials: 'include',
        body: JSON.stringify(user)
    });
}

export async function editUser(user) {
    return await fetch(apiURL + '/user/update', {
        method: 'POST',
        credentials: 'include',
        body: JSON.stringify(user)
    });
}

export async function createSkill(skill) {
    return await fetch(apiURL + '/skill/update', {
        method: 'POST',
        credentials: 'include',
        body: JSON.stringify(skill)
    });
}

export async function deleteSkillDb(skill) {
    return await fetch(apiURL + '/skill/delete', {
        method: 'POST',
        credentials: 'include',
        body: JSON.stringify(skill)
    });
}