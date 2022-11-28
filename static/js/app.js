let mn = {
    "getToken": function () {
        let cookies = document.cookie.replace(";", "").split(" ");

        try {
            return cookies[0].split("=")[1].trim();
        } catch (_) {
            return ""
        }
    },
    "setNav": function (link) {
        let links = document.getElementsByClassName("header-link");
        [...links].forEach(link_el => {
            if (link_el.id === link) {
                document.getElementById(link).classList.add("selected-link")
            }
        });
    },
}