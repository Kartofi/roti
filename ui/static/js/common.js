document.addEventListener("DOMContentLoaded", () => {
  let panel = document.getElementById("panel");
  document.body.onmousemove = (event) => {
    let x = event.pageX - panel.offsetLeft - 325;
    let y = event.pageY - panel.offsetTop - 175;
    x = clamp(x, -10, 10);
    y = clamp(y, -5, 5);

    panel.style.setProperty("--x", x + "px");
    panel.style.setProperty("--y", y + "px");
  };
});
