import {checkAuth} from "./apiCalls.js";
import {isAdmin, isLoggedIn,user} from "./stores.js";

export function hideAccordion(from, to) {
    if (from.url.pathname !== to.url.pathname) {
        let e = document.getElementById("rootDiv");
        if (e != null)
            e.style.display = "none";
    }
}

export function setCookie(cname, cvalue, exdays) {
    const d = new Date();
    d.setTime(d.getTime() + (exdays * 24 * 60 * 60 * 1000));
    let expires = "expires=" + d.toUTCString();
    document.cookie = cname + "=" + cvalue + ";" + expires + ";path=/";
}

export function getCookie(cname) {
    let name = cname + "=";
    let decodedCookie = decodeURIComponent(document.cookie);
    let ca = decodedCookie.split(';');
    for (let i = 0; i < ca.length; i++) {
        let c = ca[i];
        while (c.charAt(0) == ' ') {
            c = c.substring(1);
        }
        if (c.indexOf(name) == 0) {
            return c.substring(name.length, c.length);
        }
    }
    return "";
}

export async function checkSignIn() {
    let res = await checkAuth().then((res) => {
        if (res.status === 500) {
            return {role: -1}
        }
        return res.json().then((j) => {
            return j;
        });
    });

    isLoggedIn.update(() => res.role !== -1);
    isAdmin.update(() => res.role === 1);

    return res;
}