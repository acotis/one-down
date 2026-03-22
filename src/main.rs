
use std::collections::HashMap;

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

    let scale: f32 = 128.0; // Side length of each cell in pixels.

    // Draw the board, clues, and title to a texture object.

    let mut texture = RenderTexture::new(2048, 2048)
        .expect("could not create render texture");

    texture.clear(Color::WHITE);

    for y in 0..10 {
        for x in 0..10 {
            draw_square(
                &mut texture,
                (scale * (1.0 + x as f32), scale * (1.0 + y as f32)),
                scale / 1.414213,
                if x % 3 > 0 && y % 2 == 0 {Color::BLACK} else {Color::WHITE},
                //if (x + y) % 2 == 0 {Color::WHITE} else {Color::BLACK},
                //Color::WHITE,
                scale * 0.05,
                Color::BLACK,
            );
        }
    }

    // Save this texture as the puzzle image.

    texture.display();
    texture
        .texture()
        .copy_to_image()
        .expect("failed to copy texture to image")
        .save_to_file("output.png")
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

