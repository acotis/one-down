
use std::collections::HashMap;
use sfml::system::Vector2f;

use sfml::graphics::*;
use sfml::system::Vector2;

#[derive(PartialEq, Eq)]
struct Cell {
    letter: Option<char>,
    number: Option<usize>,
}

struct Clue {
    lines: Vec<String>,
    word_lengths: Vec<usize>,
}

fn main() {

    // Initialize the puzzle data.

    let mut board: Vec<Vec<Cell>> = vec![];
    let mut title: Option<String> = None;
    let mut author: Option<String> = None;
    let mut clue_texts: HashMap<String, Vec<Clue>> = HashMap::new();
    let mut across_words: Vec<(usize, String)> = vec![];
    let mut down_words: Vec<(usize, String)> = vec![];

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

                    clue_texts
                         .entry(word.trim().to_owned())
                         .or_insert(vec![])
                         .push(Clue {
                             lines: right.trim().split("\\").map(str::trim).map(str::to_owned).collect(),
                             word_lengths: if !lengths.is_empty() {lengths.iter().flat_map(|s|s.parse()).collect()} else {vec![word.trim().len()]}
                         });
                }
            }
        } else {
            if line.trim() != "" {
                board.push(line.chars().filter(|c| *c != ' ').map(|c| Cell {letter: if c == '.' {None} else {Some(c)}, number: None}).collect());
            }
        }
    }

    let height = board.len();
    let width = board.iter().map(Vec::len).max().unwrap();

    // Find the words.

    let mut next_number = 1;

    for y in 0..height {
        for x in 0..width {
            if x < width-1
            && board[y][x  ].letter != None
            && board[y][x+1].letter != None
            && (x == 0 || board[y][x-1].letter == None)
            {
                if board[y][x].number == None {
                    board[y][x].number = Some(next_number);
                    next_number += 1;
                }

                across_words.push((
                    board[y][x].number.unwrap(),
                    (x..width).map_while(|x| board[y][x].letter).collect()
                ));
            }

            if y < height-1
            && board[y  ][x].letter != None
            && board[y+1][x].letter != None
            && (y == 0 || board[y-1][x].letter == None)
            {
                if board[y][x].number == None {
                    board[y][x].number = Some(next_number);
                    next_number += 1;
                }

                down_words.push((
                    board[y][x].number.unwrap(),
                    (y..width).map_while(|y| board[y][x].letter).collect()
                ));
            }
        }
    }

    // Define render constants.

    let deja = Font::from_memory_static(include_bytes!("DejaVuSans-Bold.ttf")).expect("couldn't load Deja Vu font");
    let lora = Font::from_memory_static(include_bytes!("Lora-Regular.ttf")).expect("couldn't load Lora font");
    let deja_regular = Font::from_memory_static(include_bytes!("DejaVuSans.ttf")).expect("couldn't load Deja Vu font");

    let scale: f32 = 128.0; // Side length of each cell in pixels.

    // Draw the board.

    let mut texture = RenderTexture::new(4096, 4096)
        .expect("could not create render texture");

    texture.clear(Color::WHITE);

    let mut number_text = Text::new(&String::new(), &deja_regular, 0);
    let number_size = 30.0;

    number_text.set_fill_color(Color::rgb(0, 0, 0));
    number_text.set_character_size(number_size as u32);
    number_text.set_origin(sfml::system::Vector2f::new(
        0.0, //glyph.advance() / 2.0,
        number_size * 0.615,
    ));

    for y in 0..height {
        for x in 0..width {
            let xpos = scale * (1.0 + x as f32);
            let ypos = scale * (1.0 + y as f32);
            draw_square(
                &mut texture,
                (xpos, ypos),
                scale / 1.414213,
                if board[y][x].letter == None {Color::BLACK} else {Color::WHITE},
                scale * 0.05,
                Color::BLACK,
            );

            if let Some(number) = board[y][x].number {
                number_text.set_position(Vector2f::new(
                    xpos - scale * 0.4,
                    ypos - scale * 0.28,
                ));
                number_text.set_string(&format!("{number}"));
                texture.draw(&number_text);
            }
        }
    }

    let mut max_x_drawn = (width  as f32 + 1.0) * scale;
    let mut max_y_drawn = (height as f32 + 1.0) * scale;

    // Draw the clues.

    let x = scale * (width as f32 + 1.0);
    let mut y = scale * 0.8;
    let mut lora_text = Text::new(&String::new(), &lora, 0);
    let mut deja_text = Text::new(&String::new(), &deja, 0);
    let lora_size = 45.0; //self.dimensions.tile_size() * 0.5;
    let line_gap = 50.0;
    let lora_gap = 60.0;
    let deja_size = 55.0;
    let deja_gap = 82.5;
    let skip_gap = 36.0;
    let number_gap = 65.0;
    let deja_x_offset = 4.0;

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

    let across_count = across_words.len();
    let down_count = down_words.len();

    for c in 0..across_count+down_count {
        if c == 0 {
            deja_text.set_position(Vector2f::new(x - deja_x_offset, y));
            deja_text.set_string("Across");
            texture.draw(&deja_text);
            y += deja_gap;
        }

        if c == across_count {
            y += skip_gap;
            deja_text.set_position(Vector2f::new(x - deja_x_offset, y));
            deja_text.set_string("Down");
            texture.draw(&deja_text);
            y += deja_gap;
        }

        let (number, word) = if c < across_count {
            across_words[c].clone()
        } else {
            down_words[c-across_count].clone()
        };

        let clue_vec = clue_texts.get_mut(&word).expect(&format!("no clue entry for {word}"));
        if clue_vec.is_empty() {
            panic!("ran out of clue entries for {word}");
        }
        let clue = clue_vec.remove(0);

        lora_text.set_position(Vector2f::new(x, y));
        lora_text.set_string(&format!("{number}."));
        texture.draw(&lora_text);

        for i in 0..clue.lines.len() {
            let mut line = format!("{}", clue.lines[i]);

            if i == clue.lines.len()-1 {
                line += &format!(" ({})", clue.word_lengths.iter().map(|len| format!("{len}")).collect::<Vec<_>>().join(", "));
            }

            lora_text.set_position(Vector2f::new(x + number_gap, y));
            lora_text.set_string(&line);
            texture.draw(&lora_text);

            max_x_drawn = max_x_drawn.max(
                x + number_gap + lora_text.local_bounds().width + scale * 0.5
            );

            max_y_drawn = max_y_drawn.max(
                y + lora_text.local_bounds().height + scale * 0.3
            );

            y += line_gap;
        }

        y += lora_gap - line_gap;

        //lora_text.set_string(&format!("{number}. {}", clue.lines[0]));

    }

    // Save this texture as the puzzle image.

    texture.display();

    let base_image = texture
        .texture()
        .copy_to_image()
        .expect("failed to copy texture to image");

    let mut cropped_image = Image::new_solid(max_x_drawn as _, max_y_drawn as _, Color::WHITE)
        .expect("failed to create cropping image");
    
    cropped_image.copy_image(
        &base_image,
        0,
        0,
        IntRect::new(0, 0, max_x_drawn as _, max_y_drawn as _),
        false
    );

    cropped_image
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

