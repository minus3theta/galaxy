import * as galaxy from "../pkg";

const canvas = document.getElementById("galaxy_canvas");
canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();

  const canvasLeft = (event.clientX - boundingRect.left) / boundingRect.width;
  const canvasTop = (event.clientY - boundingRect.top) / boundingRect.height;

  galaxy.on_click(canvasLeft, canvasTop);
});
