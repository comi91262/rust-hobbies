const js = import("./pkg/sierpinski_triangle_generator.js");

js.then(wasm => {
  const canvas = document.getElementById("canvas");
  canvas.addEventListener("click", onClick, false);

  for (var i = 0; i < 1000; i++) {
    let a = wasm.generate();
    drawRect(a.x, a.y, 10, 10);
  }
});

function onClick(e) {
  console.log("click");
  var x = e.clientX - canvas.offsetLeft;
  var y = e.clientY - canvas.offsetTop;
  console.log("x:", x, "y:", y);

  drawRect(x, y, 1, 1);
}

function drawRect(x, y, width, height) {
  var context = canvas.getContext("2d");
  context.fillStyle = "blue";
  context.fillRect(x, y, width, height);
}
