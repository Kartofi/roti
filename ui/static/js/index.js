document.addEventListener("DOMContentLoaded", () => {
  let imgInp = document.getElementById("input_file");
  let preview = document.getElementById("preview");

  function previewUpdate() {
    const [file] = imgInp.files;
    if (file) {
      preview.src = URL.createObjectURL(file);
      let size = file.size;

      console.log(size_format(size));
    }
  }

  imgInp.onchange = (evt) => {
    previewUpdate();
  };
});
