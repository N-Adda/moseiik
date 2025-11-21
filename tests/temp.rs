#[cfg(test)]
mod tests {
    use image::RgbImage;
    use image::ImageReader;
    use moseiik::main::compute_mosaic;

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
        let img1: RgbImage = match ImageReader::open("tests/x86img.png") {
            Ok(i) => match i.decode() { // Etape decodage
                Ok(img) => img.into_rgb8(), // Etape conversion en RgbImage comme la fonction prepare_target
                Err(_) => panic!("Erreur lors du décodage de l'image"),
            },
            Err(_) => panic!("Erreur lors de la conversion en RgbImage"),
        };
        // image base
        let img2: RgbImage = match ImageReader::open("assets/ground-truth-kit.png") {
            Ok(i) => match i.decode() { // Etape decodage
                Ok(img) => img.into_rgb8(), // Etape conversion en RgbImage comme la fonction prepare_target
                Err(_) => panic!("Erreur lors du décodage de l'image"),
            },
            Err(_) => panic!("Erreur lors de la conversion en RgbImage"),
        };
        // Récupère la taille et compare les deux images
        let (w_ref,h_ref) = img2.dimensions();
        let (w_test,h_test) = img1.dimensions();
        if w_ref != w_test {
            assert!(false, "Width differentes !: ref {}, test {}", w_ref, w_test);
        }
        if h_ref != h_test {
            assert!(false, "Height differentes !: ref {}, test {}", h_ref, h_test);
        }
        // On teste pixel par pixel si les images sont similaires sinon false
        for i in 0..h_ref-1 {
            for j in 0..w_ref-1{
                let pix1 = img1.get_pixel(j,i);
                let pix2 = img2.get_pixel(j,i);
                if pix1 != pix2 {
                    assert!(false, "Pixel différent ! : w: {}, h: {}", j,i);
                }
            }
        }
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        // Fonction compute mosaic prend en arg un type Options

        let args = moseiik::main::Options {
            image: "assets/kit.jpeg".to_string(),
            output: "tests/x86img.png".to_string(),
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
        let img1: RgbImage = match ImageReader::open("tests/x86img.png") {
            Ok(i) => match i.decode() { // Etape decodage
                Ok(img) => img.into_rgb8(), // Etape conversion en RgbImage comme la fonction prepare_target
                Err(_) => panic!("Erreur lors du décodage de l'image"),
            },
            Err(_) => panic!("Erreur lors de la conversion en RgbImage"),
        };
        // image base
        let img2: RgbImage = match ImageReader::open("assets/ground-truth-kit.png") {
            Ok(i) => match i.decode() { // Etape decodage
                Ok(img) => img.into_rgb8(), // Etape conversion en RgbImage comme la fonction prepare_target
                Err(_) => panic!("Erreur lors du décodage de l'image"),
            },
            Err(_) => panic!("Erreur lors de la conversion en RgbImage"),
        };
        // Récupère la taille et compare les deux images
        let (w_ref,h_ref) = img2.dimensions();
        let (w_test,h_test) = img1.dimensions();
        if w_ref != w_test {
            assert!(false, "Width differentes !: ref {}, test {}", w_ref, w_test);
        }
        if h_ref != h_test {
            assert!(false, "Height differentes !: ref {}, test {}", h_ref, h_test);
        }
        // On teste pixel par pixel si les images sont similaires sinon false
        for i in 0..h_ref-1 {
            for j in 0..w_ref-1{
                let pix1 = img1.get_pixel(j,i);
                let pix2 = img2.get_pixel(j,i);
                if pix1 != pix2 {
                    assert!(false, "Pixel différent ! : w: {}, h: {}", j,i);
                }
            }
        } 
    }

    #[test]
    fn test_generic() {
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
        let img1: RgbImage = match ImageReader::open("tests/x86img.png") {
            Ok(i) => match i.decode() { // Etape decodage
                Ok(img) => img.into_rgb8(), // Etape conversion en RgbImage comme la fonction prepare_target
                Err(_) => panic!("Erreur lors du décodage de l'image"),
            },
            Err(_) => panic!("Erreur lors de la conversion en RgbImage"),
        };
        // image base
        let img2: RgbImage = match ImageReader::open("assets/ground-truth-kit.png") {
            Ok(i) => match i.decode() { // Etape decodage
                Ok(img) => img.into_rgb8(), // Etape conversion en RgbImage comme la fonction prepare_target
                Err(_) => panic!("Erreur lors du décodage de l'image"),
            },
            Err(_) => panic!("Erreur lors de la conversion en RgbImage"),
        };
        // Récupère la taille et compare les deux images
        let (w_ref,h_ref) = img2.dimensions();
        let (w_test,h_test) = img1.dimensions();
        if w_ref != w_test {
            assert!(false, "Width differentes !: ref {}, test {}", w_ref, w_test);
        }
        if h_ref != h_test {
            assert!(false, "Height differentes !: ref {}, test {}", h_ref, h_test);
        }
        // On teste pixel par pixel si les images sont similaires sinon false
        for i in 0..h_ref-1 {
            for j in 0..w_ref-1{
                let pix1 = img1.get_pixel(j,i);
                let pix2 = img2.get_pixel(j,i);
                if pix1 != pix2 {
                    assert!(false, "Pixel différent ! : w: {}, h: {}", j,i);
                }
            }
        }
    }
}
