[![Build Status](https://img.shields.io/travis/patrickkerrigan/rust-cnc.svg?style=flat-square)](https://travis-ci.org/patrickkerrigan/rust-cnc)

# rust-cnc

A rust library for generating G-Code from CAD files for use with CNC machines.

This is highly experimental and probably not that useful to others in its present state.

Generated G-Code is currently only targeted at laser cutters and the only supported input format is AutoCAD DXF. The following structures are converted:

* Lines
* Polylines (including bulges)
* Bezier splines
* Circles
