import { useEffect, useRef } from "react";
import initwasm, { render, parallel_render, initThreadPool, render_shared } from "../mandelbrot/pkg";
import { js_render } from "./mandelbrot";

export default function App() {
  const width = 4000;
  const height = 3000;
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    main()
  }, [])

  const main = async () => {
    const wasm = await initwasm();
    console.time("wasm time");
    const pixels = render(width, height);
    // const pixels = js_render(width, height);
    // const pixels = await webworker_parallel_task(width, height)
    // const ptr = render_shared(width, height)
    // const pixels = new Uint8Array(wasm.memory.buffer, ptr, width * height)
    console.timeEnd("wasm time");

    const canvas = canvasRef.current!;
    const ctx = canvas.getContext("2d")!;
    drawGrayscaleImage(ctx, width, height, pixels);
  }

  const webworker_parallel_task = async (width: number, height: number) => {
    await initThreadPool(navigator.hardwareConcurrency);
    return parallel_render(width, height)
  }



  return (
    <div>
      <canvas width={width} height={height} ref={canvasRef} />
    </div>
  );
}

function drawGrayscaleImage(
  context: CanvasRenderingContext2D,
  width: number,
  height: number,
  grayscaleValues: Uint8Array
) {
  let imageData = context.createImageData(width, height);
  for (let i = 0; i < grayscaleValues.length; i++) {
    const gray = grayscaleValues[i];
    imageData.data[4 * i + 0] = Math.max(gray - 30, 0);
    imageData.data[4 * i + 1] = Math.max(gray - 30, 0);
    imageData.data[4 * i + 2] = 50
    imageData.data[4 * i + 3] = 255;
  }
  context.putImageData(imageData, 0, 0);
}
