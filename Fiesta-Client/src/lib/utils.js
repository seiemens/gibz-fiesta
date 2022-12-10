import {checkAuth} from "./apiCalls.js";
import {isAdmin, isLoggedIn,user} from "./stores.js";

export function hideAccordion(from, to) {
    if (from.url.pathname !== to.url.pathname) {
        let e = document.getElementById("rootDiv");
        if (e != null)
            e.style.display = "none";
    }
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

String.prototype.hashCode = function() {
    var hash = 0,
        i, chr;
    if (this.length === 0) return hash;
    for (i = 0; i < this.length; i++) {
        chr = this.charCodeAt(i);
        hash = ((hash << 5) - hash) + chr;
        hash |= 0; // Convert to 32bit integer
    }
    return hash;
}