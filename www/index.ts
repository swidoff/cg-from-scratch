import * as wasm from "cg-from-scratch";

const raytracer_tab = document.getElementById("raytracer-tab")
const rasterizer_tab = document.getElementById("rasterizer-tab")

const raytracer = document.getElementById("raytracer")
const rasterizer = document.getElementById("rasterizer")

raytracer_tab.onclick = () => openTab("raytracer")
rasterizer_tab.onclick = () => openTab("rasterizer")

export function openTab(tabName: "raytracer" | "rasterizer") {
    switch (tabName) {
        case "raytracer":
            rasterizer.style.display = "none";
            rasterizer_tab.className = "tablinks";
            raytracer.style.display = "block";
            raytracer_tab.className = "tablinks active";

            const canvas = document.getElementById("raytracer-canvas")  as HTMLCanvasElement
            const ctx = canvas.getContext("2d");
            const bytes = wasm.render(canvas.height, canvas.width);
            const img_bytes = new Uint8ClampedArray(bytes);
            const imageData = new ImageData(img_bytes, canvas.height, canvas.width)
            ctx.putImageData(imageData, 0, 0)
            break;
        case "rasterizer":
            raytracer.style.display = "none";
            raytracer_tab.className = "tablinks";
            rasterizer.style.display = "block";
            rasterizer_tab.className = "tablinks active";
    }
}

openTab("raytracer")
