"use strict";
exports.__esModule = true;
exports.openTab = void 0;
var wasm = require("cg-from-scratch");
var raytracer_tab = document.getElementById("raytracer-tab");
var rasterizer_tab = document.getElementById("rasterizer-tab");
var raytracer = document.getElementById("raytracer");
var rasterizer = document.getElementById("rasterizer");
raytracer_tab.onclick = function () { return openTab("raytracer"); };
rasterizer_tab.onclick = function () { return openTab("rasterizer"); };
function openTab(tabName) {
    switch (tabName) {
        case "raytracer":
            rasterizer.style.display = "none";
            rasterizer_tab.className = "tablinks";
            raytracer.style.display = "block";
            raytracer_tab.className = "tablinks active";
            var canvas = document.getElementById("raytracer-canvas");
            var ctx = canvas.getContext("2d");
            var bytes = wasm.render(canvas.height, canvas.width);
            var img_bytes = new Uint8ClampedArray(bytes);
            var imageData = new ImageData(img_bytes, canvas.height, canvas.width);
            ctx.putImageData(imageData, 0, 0);
            break;
        case "rasterizer":
            raytracer.style.display = "none";
            raytracer_tab.className = "tablinks";
            rasterizer.style.display = "block";
            rasterizer_tab.className = "tablinks active";
    }
}
exports.openTab = openTab;
openTab("raytracer");
