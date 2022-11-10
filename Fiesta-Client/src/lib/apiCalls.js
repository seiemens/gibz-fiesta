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
    const response = await fetch('/testUserData.json', {
        method: 'GET',
        headers: {
            'content-type': 'application/json'
        }
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
    let allUsers = (await loadAllUsers()).users;
    for (let i = 0; i < allUsers.length; i++) {
        if (allUsers[i].username === username) {
            return allUsers[i];
        }
    }
    return null;
}