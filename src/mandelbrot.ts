type Complex = {
  re: number;
  im: number;
};

function pixelToPoint(
  bounds: [number, number],
  pixel: [number, number]
): Complex {
  const upperLeft: Complex = { re: -1.2, im: 0.35 };
  const lowerRight: Complex = { re: -1.0, im: 0.2 };
  const width = lowerRight.re - upperLeft.re;
  const height = upperLeft.im - lowerRight.im;

  return {
    re: upperLeft.re + (pixel[0] * width) / bounds[0],
    im: upperLeft.im - (pixel[1] * height) / bounds[1],
  };
}

function escapeTime(c: Complex, limit: number): number | null {
  let z: Complex = { re: 0.0, im: 0.0 };

  for (let i = 0; i < limit; i++) {
    if (z.re * z.re + z.im * z.im > 4) {
      return i;
    }
    const temp = z.re * z.re - z.im * z.im + c.re;
    z.im = 2 * z.re * z.im + c.im;
    z.re = temp;
  }

  return null;
}

export function js_render(width: number, height: number): Uint8Array {
  const bounds: [number, number] = [width, height];
  const pixels = new Uint8Array(bounds[0] * bounds[1]);

  for (let row = 0; row < bounds[1]; row++) {
    for (let column = 0; column < bounds[0]; column++) {
      const point = pixelToPoint(bounds, [column, row]);
      const escapeResult = escapeTime(point, 255);

      pixels[row * bounds[0] + column] =
        escapeResult === null ? 0 : 255 - escapeResult;
    }
  }

  return pixels;
}
