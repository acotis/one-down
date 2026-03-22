
use std::collections::HashMap;
use sfml::system::Vector2f;

use sfml::graphics::*;
use sfml::system::Vector2;

struct Clue {
    lines: Vec<String>,
    word_lengths: Vec<usize>,
}

fn main() {

    // Initialize the puzzle data.

    let mut board: Vec<Vec<Option<char>>> = vec![];
    let mut title: Option<String> = None;
    let mut author: Option<String> = None;
    let mut clues: HashMap<String, Vec<Clue>> = HashMap::new();

    // Read the puzzle file.

    let input = std::fs::read_to_string("puzzle.txt").expect("couldn't open puzzle.txt");

    for line in input.lines() {
        if let Some((left_raw, right)) = line.split_once(":") {
            let left = left_raw.to_uppercase();

            match &*left {
                "@TITLE"  => {title  = Some(right.trim().to_owned());}
                "@AUTHOR" => {author = Some(right.trim().to_owned());}
                _ => {
                    let [word, ref lengths@..] =
                        left.split(['(',',',')'])
                            .collect::<Vec<_>>()[..]
                    else {panic!()};

                    clues.entry(word.trim().to_owned())
                         .or_insert(vec![])
                         .push(Clue {
                             lines: right.trim().split("\\").map(str::trim).map(str::to_owned).collect(),
                             word_lengths: if !lengths.is_empty() {lengths.iter().flat_map(|s|s.parse()).collect()} else {vec![word.trim().len()]}
                         });
                }
            }
        } else {
            if line.trim() != "" {
                board.push(line.chars().filter(|c| *c != ' ').map(|c| if c == '.' {None} else {Some(c)}).collect());
            }
        }
    }

    // Define render constants.

    let deja = Font::from_memory_static(include_bytes!("DejaVuSans-Bold.ttf")).expect("couldn't load Deja Vu font");
    let lora = Font::from_memory_static(include_bytes!("Lora-Regular.ttf")).expect("couldn't load Lora font");

    let scale: f32 = 128.0; // Side length of each cell in pixels.
    let height = board.len();
    let width = board.iter().map(Vec::len).max().unwrap();

    // Draw the board.

    let mut texture = RenderTexture::new(2048, 2048)
        .expect("could not create render texture");

    texture.clear(Color::WHITE);

    for y in 0..height {
        for x in 0..width {
            draw_square(
                &mut texture,
                (scale * (1.0 + x as f32), scale * (1.0 + y as f32)),
                scale / 1.414213,
                if matches!(board[y].get(x), Some(Some(_))) {Color::WHITE} else {Color::BLACK},
                scale * 0.05,
                Color::BLACK,
            );
        }
    }

    // Draw the clues.

    let x = scale * (width as f32 + 0.96);
    let mut y = scale * 0.78;
    let mut lora_text = Text::new(&String::new(), &lora, 0);
    let mut deja_text = Text::new(&String::new(), &deja, 0);
    let lora_size = 40.0; //self.dimensions.tile_size() * 0.5;
    let lora_gap = 60.0;
    let deja_size = 55.0;
    let deja_gap = 82.5;
    let skip_gap = 30.0;

    lora_text.set_fill_color(Color::BLACK);
    lora_text.set_character_size(lora_size as u32);
    lora_text.set_origin(sfml::system::Vector2f::new(
        0.0, //glyph.advance() / 2.0,
        lora_size * 0.615,
    ));

    deja_text.set_fill_color(Color::BLACK);
    deja_text.set_character_size(deja_size as u32);
    deja_text.set_origin(sfml::system::Vector2f::new(
        0.0, //glyph.advance() / 2.0,
        deja_size * 0.615,
    ));

    for c in 0..11 {
        if c == 0 {
            deja_text.set_position(Vector2f::new(x, y));
            deja_text.set_string("Across");
            texture.draw(&deja_text);
            y += deja_gap;
        }

        if c == 6 {
            y += skip_gap;
            deja_text.set_position(Vector2f::new(x, y));
            deja_text.set_string("Down");
            texture.draw(&deja_text);
            y += deja_gap;
        }


        lora_text.set_position(Vector2f::new(x, y));
        lora_text.set_string("Hello world");
        texture.draw(&lora_text);

        y += lora_gap;
    }

    // Save this texture as the puzzle image.

    texture.display();
    texture
        .texture()
        .copy_to_image()
        .expect("failed to copy texture to image")
        .save_to_file("puzzle.png")
        .expect("failed to save image file");

    // Add in the answers.

    // Now save this texture as the answer key image.
}

// Draw utilities.

fn draw_square(
    texture: &mut RenderTexture,
    center: (f32, f32),
    radius: f32,
    color: Color,
    outline_thickness: f32,
    outline_color: Color
) {
    draw_polygon(texture, 4, 0.125, center, radius, color, outline_thickness, outline_color);
}

fn draw_polygon(
    texture: &mut RenderTexture,
    side_count: u32,
    rotation: f32,
    center: (f32, f32),
    radius: f32,
    color: Color,
    outline_thickness: f32,
    outline_color: Color
) {
    let mut cs = CircleShape::new(radius, side_count as usize);
    cs.set_origin(Vector2::new(radius, radius));
    cs.set_position(Vector2::new(center.0, center.1));
    cs.rotate(rotation * 360.0);
    cs.set_fill_color(color);
    cs.set_outline_thickness(outline_thickness);
    cs.set_outline_color(outline_color);
    texture.draw(&cs);
}

