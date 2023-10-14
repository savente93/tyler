pub fn write_tiles() {
    println!("writing tyles");
    sub_pannels.iter().enumerate().for_each(|(idx, tile)| {
        let mut output = File::create(format!("testing_assets/out{}.png", idx)).unwrap();

        tile.to_image()
            .write_to(&mut output, ImageFormat::Png)
            .unwrap();
    })
}
