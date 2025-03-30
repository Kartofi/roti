document.addEventListener("DOMContentLoaded", () => {
  let imgInp = document.getElementById("input_file");

  let text_info = document.getElementById("input_file_name");
  let preview = document.getElementById("preview");

  function previewUpdate() {
    const [file] = imgInp.files;
    if (file) {
      preview.src = URL.createObjectURL(file);
      let size = size_format(file.size);
      text_info.innerText = shorten_string(file.name) + " (" + size + ")";
    }
  }

  imgInp.onchange = (evt) => {
    previewUpdate();
  };
});
