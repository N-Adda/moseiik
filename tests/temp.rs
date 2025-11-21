#[cfg(test)]
mod tests {
    use image::RgbImage;
    use image::ImageReader;
    use moseiik::main::compute_mosaic;
    use std::error::Error;

    // Fonction decode in RGB a part pour regler le prob d'ouverture fichier
    // Meme principe que main ligne 95
    fn tile_result(image_path: &str) -> Result<RgbImage, Box<dyn Error>>  {
        let target = ImageReader::open(image_path)?.decode()?.into_rgb8();
        Ok(target)
    } 

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        // Fonction compute mosaic prend en arg un type Options

        let args = moseiik::main::Options {
            image: "assets/kit.jpeg".to_string(),
            output: "tests/x86img.png".to_string(),
            tiles: "assets/images".to_string(),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: false,
            num_thread: 1,
        };
        // On recrée l'image en utilisant la fonction
        compute_mosaic(args);

        // image générée
        let img1 ;
        match tile_result("tests/x86img.png") {
            Ok(i) => img1 = i,
            Err(_) => return,
        };
        // image base
        let img2 ;
        match tile_result("assets/ground-truth-kit.jpeg") {
            Ok(i) => img2 = i,
            Err(_) => return,
        };
        // Récupère la taille et compare les deux images
        let w_ref = img2.width();
        let h_ref = img2.height();
        let w_test = img1.width();
        let h_test = img1.height();
        if w_ref != w_test {
            assert!(false, "Width differentes !: ref {}, test {}", w_ref, w_test);
        }
        if h_ref != h_test {
            assert!(false, "Height differentes !: ref {}, test {}", h_ref, h_test);
        }
        // On teste pixel par pixel si les images sont similaires sinon false
        for i in 0..h_ref {
            for j in 0..w_ref {
                let pix1 = img1.get_pixel(j,i);
                let pix2 = img2.get_pixel(j,i);
                assert_eq!(pix1, pix2, "Pixel différent ! : w: {}, h: {}", j,i);
            }
        }
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        // Fonction compute mosaic prend en arg un type Options

        let args = moseiik::main::Options {
            image: "assets/kit.jpeg".to_string(),
            output: "tests/aarch64img.png".to_string(),
            tiles: "assets/images".to_string(),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: true, 
            num_thread: 1,
        };
        // On recrée l'image en utilisant la fonction
        compute_mosaic(args);

        // image générée
        let img1 ;
        match tile_result("tests/aarch64img.png") {
            Ok(i) => img1 = i,
            Err(_) => return,
        };
        // image base
        let img2 ;
        match tile_result("assets/ground-truth-kit.jpeg") {
            Ok(i) => img2 = i,
            Err(_) => return,
        };
        // Récupère la taille et compare les deux images
        let w_ref = img2.width();
        let h_ref = img2.height();
        let w_test = img1.width();
        let h_test = img1.height();
        if w_ref != w_test {
            assert!(false, "Width differentes !: ref {}, test {}", w_ref, w_test);
        }
        if h_ref != h_test {
            assert!(false, "Height differentes !: ref {}, test {}", h_ref, h_test);
        }
        // On teste pixel par pixel si les images sont similaires sinon false
        for i in 0..h_ref {
            for j in 0..w_ref {
                let pix1 = img1.get_pixel(j,i);
                let pix2 = img2.get_pixel(j,i);
                assert_eq!(pix1, pix2, "Pixel différent ! : w: {}, h: {}", j,i);
            }
        }
    }

    #[test]
    fn test_generic() {
        // Fonction compute mosaic prend en arg un type Options

        let args = moseiik::main::Options {
            image: "assets/kit.jpeg".to_string(),
            output: "tests/genericimg.png".to_string(),
            tiles: "assets/images".to_string(),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: true,
            num_thread: 1,
        };
        // On recrée l'image en utilisant la fonction
        compute_mosaic(args);

        // image générée
        let img1 ;
        match tile_result("tests/genericimg.png") {
            Ok(i) => img1 = i,
            Err(_) => return,
        };
        // image base
        let img2 ;
        match tile_result("assets/ground-truth-kit.jpeg") {
            Ok(i) => img2 = i,
            Err(_) => return,
        };
        // Récupère la taille et compare les deux images
        let w_ref = img2.width();
        let h_ref = img2.height();
        let w_test = img1.width();
        let h_test = img1.height();
        if w_ref != w_test {
            assert!(false, "Width differentes !: ref {}, test {}", w_ref, w_test);
        }
        if h_ref != h_test {
            assert!(false, "Height differentes !: ref {}, test {}", h_ref, h_test);
        }
        // On teste pixel par pixel si les images sont similaires sinon false
        for i in 0..h_ref {
            for j in 0..w_ref {
                let pix1 = img1.get_pixel(j,i);
                let pix2 = img2.get_pixel(j,i);
                assert_eq!(pix1, pix2, "Pixel différent ! : w: {}, h: {}", j,i);
            }
        }
    }
}
