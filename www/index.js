"use strict";
exports.__esModule = true;
var wasm = require("cg-from-scratch");
var canvas = document.getElementById("raytracer-canvas");
var ctx = canvas.getContext("2d");
var bytes = wasm.render(canvas.height, canvas.width);
var img_bytes = new Uint8ClampedArray(bytes);
var imageData = new ImageData(img_bytes, canvas.height, canvas.width);
ctx.putImageData(imageData, 0, 0);
