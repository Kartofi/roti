document.addEventListener("DOMContentLoaded", () => {
  let imgInp = document.getElementById("input_file");
  let preview = document.getElementById("preview");

  let bytes_sizes = ["bytes", "KB", "MB", "GB"];

  function previewUpdate() {
    const [file] = imgInp.files;
    if (file) {
      preview.src = URL.createObjectURL(file);
      let size = file.size;
      let steps = 0;
      while (size >= 1024 && steps < bytes_sizes.length - 1) {
        size /= 1024;
        steps += 1;
      }
      size = size.toFixed(2);
      console.log(size + " " + bytes_sizes[steps]);
    }
  }

  imgInp.onchange = (evt) => {
    previewUpdate();
  };
});
