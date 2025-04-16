let bans = [];

let bans_count;
let bans_el;

document.addEventListener("DOMContentLoaded", () => {
  let ip_search_input = document.getElementById("ip_search_input");

  let fetch_bans_el = document.getElementById("fetch_bans");

  let refresh_bans_el = document.getElementById("refresh_bans");

  let search_el = document.getElementById("search");

  bans_count = document.getElementById("bans_count");
  let found_bans_count = document.getElementById("found_bans_count");

  bans_el = document.getElementById("bans");

  // ban
  let ban_ip_input = document.getElementById("ban_ip_input");
  let ban_reason_input = document.getElementById("ban_reason_input");
  let ban_button = document.getElementById("ban");
  let ban_result = document.getElementById("ban_result");

  ban_button.addEventListener("click", async () => {
    let result = await ban_ip(ban_ip_input.value, ban_reason_input.value);
    if (result.result == false) {
      ban_result.innerHTML = "<red>" + result.error + "</red>";
    } else {
      ban_result.innerHTML = 'Banned "' + ban_ip_input.value + '"';
    }
  });
  fetch_bans_el.addEventListener("click", async () => {
    update_bans();
  });
  refresh_bans_el.addEventListener("click", async () => {
    found_bans_count.innerText = "Found 0 bans";
    ip_search_input.value = "";
    display_bans(bans, bans_el);
  });
  search_el.addEventListener("click", async () => {
    let filteredBans = bans
      .filter(
        (ban) =>
          ip_search_input.value.length === 0 ||
          ban.ip.includes(ip_search_input.value)
      )
      .map((ban) => {
        return {
          ...ban,
          ip: ban.ip.replaceAll(
            ip_search_input.value,
            "<red>" + ip_search_input.value + "</red>"
          ),
        };
      });

    found_bans_count.innerText = "Found " + filteredBans.length + " bans";
    display_bans(filteredBans, bans_el);
  });
});
async function update_bans() {
  bans = await get_bans();
  if (bans.error != undefined) {
    bans_el.innerHTML = bans;
    return;
  }

  bans_count.innerText = "Loaded " + bans.length + " bans";
}
async function unban(ip) {
  let res = await unban_ip(ip);
  if (res.result == false) {
    return;
  }
  await update_bans();
  display_bans(bans);
}
// Http
async function ban_ip(ip, reason) {
  let formData = new FormData();

  formData.append("ip", ip);
  formData.append("reason", reason);

  let res = await fetch("/admin/ban", {
    method: "POST",
    body: formData,
  });

  return res.json();
}
async function unban_ip(ip) {
  let formData = new FormData();

  formData.append("ip", ip);

  let res = await fetch("/admin/unban", {
    method: "DELETE",
    body: formData,
  });

  return res.json();
}
async function get_bans() {
  let res = await fetch("/admin/getbans", {
    method: "POST",
  });

  return res.json();
}
// HTML
function display_bans(bans) {
  bans_el.innerHTML = "";
  bans.forEach((ban) => {
    let ban_el = document.createElement("a");
    let date = new Date(ban.time * 1000);

    let date_string = format_date(date);

    ban_el.innerHTML =
      "<br>IP: " +
      ban.ip +
      "<br>Reason: " +
      ban.reason +
      "<br>Banned on " +
      date_string +
      `<br><button onclick="unban('` +
      ban.ip +
      `')">Unban</button><br>`;

    bans_el.appendChild(ban_el);
  });
}
