export function hideAccordion(from, to) {
    if (from.url.pathname !== to.url.pathname) {
        let e = document.getElementById("rootDiv");
        if (e != null)
            e.style.display = "none";
    }
}