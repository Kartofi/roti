document.addEventListener("DOMContentLoaded", () => {
  let file_size_el = document.getElementById("file_size");
  let views_el = document.getElementById("views");

  let file_size = file_size_el.innerText;
  file_size_el.innerText = format_size(file_size);

  let views = views_el.innerText;
  views_el.innerText = format_number(views);
});
