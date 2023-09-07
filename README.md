# Demonstration code for `laz-rs` seek issue

Demonstrates a problem with the `laz-rs` crate when using `seek` to seek into the last chunk of an LAZ file, it appears to read the wrong points. This example uses the `las` crate, but since `LASReader::seek` only redirects to the `LasZipDecompressor`, I'm assuming the issue is with the `laz-rs` crate.