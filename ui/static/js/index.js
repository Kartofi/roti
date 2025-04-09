window.addEventListener(
  "dragover",
  function (e) {
    e.preventDefault();
  },
  false
);
window.addEventListener(
  "drop",
  function (e) {
    e.preventDefault();
  },
  false
);

document.addEventListener("DOMContentLoaded", () => {
  let imgInp = document.getElementById("input_file");

  let text_info = document.getElementById("input_file_name");
  let default_text = text_info.innerText;

  let preview = document.getElementById("preview");
  let drag_background = document.getElementById("drag_background");
  let drag_text = document.getElementById("drag_text");
  let drag_image = document.getElementById("drag_image");

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
      default_text = text_info.innerText;
    }
  }
  function show_drag_background() {
    drag_background.style.height = "100vh";

    drag_text.style.opacity = "100";

    drag_image.style.height = "100%";
    drag_image.style.opacity = "100";
    drag_image.style.backdropFilter = "blur(0px)";
  }
  function hide_drag_background() {
    drag_background.style.height = "0px";
    drag_text.style.opacity = "0";

    drag_image.style.height = "0px";
    drag_image.style.opacity = "0";
    drag_image.style.backdropFilter = "blur(100px)";
  }
  let dragging = false;

  function change_text_drag() {
    if (dragging) {
      text_info.innerText = "Dragging..";
      show_drag_background();
    } else {
      text_info.innerText = default_text;
      hide_drag_background();
    }
  }

  window.ondrop = (event) => {
    if (
      !(
        event.dataTransfer.files.length == 1 &&
        is_image(event.dataTransfer.files[0].name) == true
      )
    ) {
      text_info.innerText = "Only images are allowed!";
      hide_drag_background();
      return;
    }
    dragging = false;
    change_text_drag();
    imgInp.files = event.dataTransfer.files;
    previewUpdate();
  };
  window.ondragover = (event) => {
    dragging = true;
    change_text_drag();
  };
  window.ondragend = (event) => {
    dragging = false;
    change_text_drag();
  };
  window.ondragleave = (event) => {
    dragging = false;
    change_text_drag();
  };

  imgInp.onchange = (evt) => {
    previewUpdate();
  };
});
