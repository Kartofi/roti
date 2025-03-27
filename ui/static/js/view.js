document.addEventListener("DOMContentLoaded", () => {
  let file_size_el = document.getElementById("file_size");

  let file_size = file_size_el.innerText;
  file_size_el.innerText = size_format(file_size);
});
