#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

extern crate bmp;

mod block;
// mod utils

const PLAINTEXT_FILENAME:&str = "plain.bmp";
const CIPHER_FILENAME_SUFFIX: &str = "_crypto.bmp";


fn main() {
    let (source_vector, source_dimensions) = read_image(PLAINTEXT_FILENAME);

    let crypted_ecb = block::crypt_ecb(&source_vector);
    let crypted_cbc = block::crypt_cbc(&source_vector);

    // TODO: partial application? :D
    // TODO: Refactor call
    write_image(&format!("ecb{}", CIPHER_FILENAME_SUFFIX), crypted_ecb, source_dimensions);
    write_image(&format!("cbc{}", CIPHER_FILENAME_SUFFIX), crypted_cbc, source_dimensions);
}
// TODO: other todos

// TODO: Result
// TODO: Dimensions type
fn read_image(filename: &str) -> (Vec<bool>, (u32, u32))
{
    let source_image = bmp::open(PLAINTEXT_FILENAME)
        .unwrap_or_else(|e| {
            panic!("Failed to open: {}", e);
        })
    ;

    (source_image
            .coordinates() 
            .map(|(x, y)| { 
                match source_image.get_pixel(x,y) {
                    bmp::consts::BLACK => true,
                    bmp::consts::WHITE => false,
                    _ => panic!("Wrong color detected - pixel ({}, {}) is neither black nor white", x, y),
                }
            })
        .collect::<Vec<_>>(), (source_image.get_width(), source_image.get_height()))
}

// TODO: result
fn write_image(filename: &str, content: Vec<bool>, dimensions: (u32, u32)) -> ()
{
    let mut crypted_img = bmp::Image::new(dimensions.0, dimensions.1);

    for (x,y) in crypted_img.coordinates()
    {
        let index = (y * dimensions.0 + x) as usize;

        crypted_img.set_pixel(x, y, (match content[index] {
            true => bmp::consts::BLACK,
            false => bmp::consts::WHITE,
        }));
    }
    let _ = crypted_img
        .save(filename)
        .unwrap_or_else(|e| {
            panic!("Failed to save: {}", e);
        })
    ;

}

//fn exit_err<T: Display, U: Display>(message: T, context: U) -> !
//{
//    let _ = writeln!(&mut io::stderr(), "ERROR at {}: {}", context, message);
//    process::exit(1);
//}
