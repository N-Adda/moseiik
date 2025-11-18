#[cfg(test)]
mod tests {
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        // Fonction compute mosaic prend en arg un type Options

        let args = moseiik::main::Options {
            image: "assets/kit.jpeg",
            output: "tests/x86img.png",
            tiles: "assets",
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: true,
            num_thread: 1,
        }

        // test avx2 or sse2 if available
        assert!(false);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        //TODO 
        assert!(false);
    }

    #[test]
    fn test_generic() {
        //TODO
        assert!(false);
    }
}
