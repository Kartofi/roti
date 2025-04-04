document.addEventListener("DOMContentLoaded", () => {
  let imgInp = document.getElementById("input_file");

  let text_info = document.getElementById("input_file_name");
  let preview = document.getElementById("preview");

  let to_format_nums = document.getElementsByClassName("format_num");
  [...to_format_nums].forEach((element) => {
    element.innerText = format_number(Number(element.innerText));
  });

  let to_format_sizes = document.getElementsByClassName("format_size");
  [...to_format_sizes].forEach((element) => {
    element.innerText = format_size(Number(element.innerText));
  });

  function previewUpdate() {
    const [file] = imgInp.files;
    if (file) {
      preview.src = URL.createObjectURL(file);
      let size = format_size(file.size);
      text_info.innerText = shorten_string(file.name) + " (" + size + ")";
    }
  }

  imgInp.onchange = (evt) => {
    previewUpdate();
  };
});
