**DISCLAIMER:** This is an attempt at making a visualization tool similar to
matplotlib in pure rust. This lib is COMPLETELY experimental.

Objective:
- Reproduce a matplotlib-like interface: https://matplotlib.org/api/pyplot_summary.html#the-object-oriented-api
- Make it rusty
- Have support for several back-end (piston/glium/printpdf/gfx considered)
- Make it reactive

Add ID (hash from input data) to image. ID is set on image creation.
