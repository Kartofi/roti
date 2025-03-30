document.addEventListener("DOMContentLoaded", () => {
  let password = document.getElementById("admin_pass");

  let get_bans_el = document.getElementById("get_bans");
  let bans_el = document.getElementById("bans");

  get_bans_el.addEventListener("click", async () => {
    console.log(password.value);
    let bans = await get_bans(password.value);

    if (typeof bans === "string") {
      bans_el.innerHTML = bans;
      return;
    }

    bans_el.innerHTML = "";
    bans.forEach((ban) => {
      let ban_el = document.createElement("a");
      let date = new Date(ban.time * 1000);

      let date_string =
        formatWithZero(date.getHours()) +
        ":" +
        formatWithZero(date.getMinutes()) +
        ":" +
        formatWithZero(date.getSeconds()) +
        " " +
        formatWithZero(date.getDate()) +
        "." +
        formatWithZero(date.getMonth()) +
        "." +
        date.getFullYear();

      ban_el.innerHTML =
        "<br>IP: " +
        ban.ip +
        "<br>Reason: " +
        ban.reason +
        "<br>Banned on " +
        date_string +
        "<br>";

      bans_el.appendChild(ban_el);
    });
  });
});
function formatWithZero(number) {
  return String(number).padStart(2, "0");
}
async function get_bans(password) {
  let formData = new FormData();

  formData.append("password", password);

  let res = await fetch("/admin/getbans", {
    method: "POST",
    body: formData,
  });
  if (res.status != 200) {
    return res.text();
  }
  return res.json();
}
