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

            render("raytracer-canvas", wasm.raytracer)
            break;
        case "rasterizer":
            raytracer.style.display = "none";
            raytracer_tab.className = "tablinks";
            rasterizer.style.display = "block";
            rasterizer_tab.className = "tablinks active";

            render("rasterizer-canvas", wasm.rasterizer)
    }
}

function render(canvas_id: string, func: (width: number, height: number) => Uint8Array) {
    const canvas = document.getElementById(canvas_id)  as HTMLCanvasElement
    const ctx = canvas.getContext("2d");
    const bytes = func(canvas.height, canvas.width);
    const img_bytes = new Uint8ClampedArray(bytes);
    const imageData = new ImageData(img_bytes, canvas.height, canvas.width)
    ctx.putImageData(imageData, 0, 0)
}

openTab("rasterizer")
