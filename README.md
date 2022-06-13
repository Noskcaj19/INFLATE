# INFLATE

Inflate is a program that uses the INFLATE[^1] algorithm to create ZIP files from input files.

Inflate uses the standard ZIP format and supports all the same types of input files ZIP does.  ZIP files created with INFLATE can be restored back to their original size using any compliant ZIP decompressor (also known as inflate).

## About

The INFLATE expansion algorithm allows for lossless file size inflation with configurable levels and sizes.

INFLATE supports a configurable inflation level and is capable of zero inflation up to many thousands of times the size of the original files.

### Why?

In recent years computer disk drives have been rapidly increasing in size.  This means that now more than ever individuals have abundant free disk space.  INFLATE was created in order to solve this issue.

## Usage

    inflate output_zip [-b num_bytes] [-m multipler] [input_paths]...

* num_bytes only supports increments of 5 bytes at a time

## Changelog

* INFLATE now supports 0% inflation


[^1]: Not to be confused with DEFLATE and its' decompression algorithm also called INFLATE
