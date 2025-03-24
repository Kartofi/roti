let imgInp = document.getElementById("preview");

imgInp.onchange = (evt) => {
  const [file] = imgInp.files;
  if (file) {
    blah.src = URL.createObjectURL(file);
  }
};
