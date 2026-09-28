use std::fs;

use image::{GenericImage, ImageBuffer, RgbImage, Rgba, RgbaImage};

fn upload_texture(img: &ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> u32 {

    let (width, height) = img.dimensions();

    let mut id = 0;

    unsafe {
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 4);

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_EDGE as i32
        );

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_EDGE as i32
        );

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST_MIPMAP_LINEAR as i32
        );

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::NEAREST as i32
        );

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img.as_ptr() as *const _,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    id
}

pub fn bind(id: u32, unit: u32) {
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0 + unit);
        gl::BindTexture(gl::TEXTURE_2D, id);
    }
}

pub fn load_texture_from_path(path: &str) -> u32 {
    upload_texture(&image::open(path).expect("Failed to load texture").flipv().into_rgba8())
}

pub fn load_texture_atlas(image_pool_path: &str) -> TextureAtlas {

    let tile_size = 16;

    let mut images = Vec::new();
    let mut tiles = Vec::new();

    for entry in fs::read_dir(image_pool_path).expect("Failed to read atlas pool directory") {
        let entry = entry.expect("Failed to read atlas pool entry");
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        if extension != "png" {
            continue;
        }

        println!("Loading texture: {}", path.display());

        let img = image::open(&path)
            .expect("Failed to load texture")
            .flipv()
            .into_rgba8();

        if img.width() != tile_size || img.height() != tile_size {
            println!("Tile {} has incorrect size", path.display());
        };

        images.push((path, img));
    }

    let image_count = images.len() as f32;
    let atlas_tiles_width = image_count.sqrt().ceil() as u32;
    let atlas_width = atlas_tiles_width * tile_size;
    let tile_aspect = 1.0 / atlas_tiles_width as f32;

    let mut atlas = RgbaImage::new(atlas_width, atlas_width);

    for (index, (path, img)) in images.into_iter().enumerate() {
        let index = index as u32;

        let tile_x = (index as u32 % atlas_tiles_width) * tile_size;
        let tile_y = (index as u32 / atlas_tiles_width) * tile_size;

        atlas.copy_from(&img, tile_x, tile_y).expect("Failed to copy tile into atlas");

        println!("tile_x: {}", (tile_x as f32 / 16.0) * tile_aspect);
        println!("tile_y: {}", (tile_y as f32 / 16.0) * tile_aspect);

        tiles.push(Tile{
            name: path.file_stem().unwrap().to_string_lossy().into_owned(), // TODO: replace unwrap logic
            x: (tile_x as f32 / 16.0) * tile_aspect,
            y: (tile_y as f32 / 16.0) * tile_aspect,
        });
    };

    TextureAtlas{id: upload_texture(&atlas), tiles, tiles_per_row: atlas_tiles_width}
}

impl TextureAtlas {
    pub fn search_tile(&self, tile: String) -> (f32, f32) {
        // the first tile in the atlas should be "missing block"
        // since if the tile isn't found this function will return uv offset 0.0
        println!("SearchTile: {}", tile);
        for i in 0..self.tiles.len() {
            if self.tiles[i].name == tile {
                println!("SearchedTile: {}", self.tiles[i].name);
                return (self.tiles[i].x, self.tiles[i].y);
            }
        }
        (0.0, 0.0)
    }
}

pub struct TextureAtlas {
    pub id: u32,
    pub tiles: Vec<Tile>,
    pub tiles_per_row: u32,
}

pub struct Tile {
    name: String,
    x: f32,
    y: f32
}