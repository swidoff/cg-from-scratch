import * as wasm from "cg-from-scratch";

const canvas = document.getElementById("raytracer-canvas")  as HTMLCanvasElement
const ctx = canvas.getContext("2d");

const bytes = wasm.render(canvas.height, canvas.width);
const img_bytes = new Uint8ClampedArray(bytes);
const imageData = new ImageData(img_bytes, canvas.height, canvas.width)
ctx.putImageData(imageData, 0, 0)

