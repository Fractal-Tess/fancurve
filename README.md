## Fancurve

Got tired of looking at my GPU's temp sky-rocketing to 80 degress and obliterating itself due to the stupidity of nvidia's default fan settings on linux.

This is a simple solution that queries the temp of `GPU-0` and sets its fan speed according to a polynomial curve.

If you want to tweak/customize the curve, I highly recommend using [desmos](https://www.desmos.com/calculator) to easily visualize it.

The current hard-coded fan curve in the source code looks something like this:

<div align="center">
  <img src="https://raw.githubusercontent.com/Fractal-Tess/fancurve/master/fancurve.jpeg" width="580" style="border-radius:2rem"/>
</div>

Where the X axis represents temperature and Y represents fan speed percentage
