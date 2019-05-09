const js = import("./pkg/sierpinski_triangle_generator.js");
const N = 1
const canvas = document.getElementById("canvas");

js.then(wasm => {
  for (let i = 0; i < N; i++) {
    let point = wasm.generate();
    drawRect(point.x, point.y, 2, 2);
  }
});

function drawRect(x, y, width, height) {
  var context = canvas.getContext("2d");
  context.fillStyle = "black";
  context.fillRect(x, y, width, height);
}
