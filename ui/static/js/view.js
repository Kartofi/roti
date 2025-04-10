document.addEventListener("DOMContentLoaded", () => {
  let file_size_el = document.getElementById("file_size");
  let views_el = document.getElementById("views");
  let meta_desc_el = document.getElementById("meta-desc");

  let meta_desc = meta_desc_el.content;
  let parts = meta_desc.split(" ");

  parts[1] = format_size(parts[1]);
  parts[3] = format_number(parts[3]);

  meta_desc_el.content = parts.join(" ");

  let file_size = file_size_el.innerText;
  file_size_el.innerText = format_size(file_size);

  let views = views_el.innerText;
  views_el.innerText = format_number(views);
});
