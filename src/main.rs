
use std::collections::HashMap;
use sfml::system::Vector2f;
use sfml::cpp::FBox;

use sfml::graphics::*;
use sfml::system::Vector2;

#[derive(PartialEq, Eq)]
struct Cell {
    letter: Option<char>,
    number: Option<usize>,
}

#[derive(Clone)]
struct Clue {
    lines: Vec<String>,
}

fn main() {

    // Initialize the puzzle data.

    let mut board: Vec<Vec<Cell>> = vec![];
    let mut title: Option<String> = None;
    let mut author: Option<String> = None;
    let mut flat_title: bool = true;
    let mut squish: f32 = 0.0;
    let mut clue_texts: HashMap<String, Vec<Clue>> = HashMap::new();
    let mut across_words: Vec<(usize, String)> = vec![];
    let mut down_words: Vec<(usize, String)> = vec![];

    // Read the puzzle file.

    let arg = std::env::args().skip(1).next().expect("need one argument");
    let input = std::fs::read_to_string(&arg).expect("couldn't open puzzle.txt");

    for line in input.lines() {
        if line.contains("%%%") {
            break;
        }

        if line.trim().to_uppercase() == "@TALL-TITLE" {
            flat_title = false;
            continue;
        }

        if let Some((left_raw, right)) = line.split_once(":") {
            let left = left_raw.to_uppercase();

            match &*left {
                "@TITLE"  => {title  = Some(right.trim().to_owned());}
                "@AUTHOR" => {author = Some(right.trim().to_owned());}
                "@X-SQUISH" => {squish = right.trim().parse().unwrap();}
                _ => {
                    let [word, ref lengths@..] =
                        left.split(['(',',',' ',')'])
                            .filter(|bit| *bit != "")
                            .collect::<Vec<_>>()[..]
                    else {panic!()};

                    let lengths_bit = if lengths.is_empty() {
                        format!("({})", word.trim().len())
                    } else {
                        format!("({})", lengths.join(",\u{a0}"))
                    };

                    clue_texts
                         .entry(word.trim().to_owned())
                         .or_insert(vec![])
                         .push(Clue {
                             lines: (right.replace("'", "’").trim().to_owned() + "\u{a0}" + &lengths_bit).split("\\").map(str::trim).map(str::to_owned).collect(),
                         });
                }
            }
        } else {
            if line.trim() != "" {
                let row: Vec<Cell> = line.chars().filter(|c| *c != ' ').map(|c| Cell {letter: match c {'.' => None, c if !c.is_ascii_alphabetic() => Some(' '), c => Some(c)}, number: None}).collect();
                if row.len() == 1 {
                    //println!("{:?}", row[0].letter);
                    //println!("*{line}*");
                }
                board.push(row);
            }
        }
    }

    /*
    for row in &board {
        println!("line length: {}", row.len());
    }
    */

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
                    (x..width).map_while(|x| board[y][x].letter).collect::<String>().to_uppercase()
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
                    (y..height).map_while(|y| board[y][x].letter).collect::<String>().to_uppercase()
                ));
            }
        }
    }

    // Define render constants.

    let scale: f32 = 384.0; // Side length of each cell in pixels.

    let clue_text_size = 120;
    let header_text_size = 165;
    let number_size = 105;
    let key_size = 210;
    let title_size = 120;

    let clue_line_height = 140.0;
    let header_line_height = 247.5;
    let title_line_height = 165.0;

    let clue_sep = 180.0;
    let section_sep = 90.0;

    let clue_indent = 50.0;
    let clue_content_indent = 195.0;
    let header_dedent = 12.0;

    // Create the text objects that will be used to draw text.

    let lora = Font::from_memory_static(include_bytes!("Lora-Regular.ttf")).expect("couldn't load Lora font");
    //let lora_bold = Font::from_memory_static(include_bytes!("Lora-Bold.ttf")).expect("couldn't load Lora font");
    let dejavusans = Font::from_memory_static(include_bytes!("DejaVuSans.ttf")).expect("couldn't load Deja Vu Sans font");
    let dejavusans_bold = Font::from_memory_static(include_bytes!("DejaVuSans-Bold.ttf")).expect("couldn't load Deja Vu Sans Bold font");
    let dejavusans_italic = Font::from_memory_static(include_bytes!("DejaVuSans-Oblique.ttf")).expect("couldn't load Deja Vu Sans Italic font");

    let mut number_text = create_text(&dejavusans,      number_size,        Color::BLACK);
    let mut clue_text   = create_text(&lora,            clue_text_size,     Color::BLACK);
    let mut header_text = create_text(&dejavusans_bold, header_text_size,   Color::BLACK);
    let mut title_text  = create_text(&dejavusans_bold, title_size,         Color::BLACK);
    let mut key_text    = create_text(&dejavusans_bold, key_size,           Color::rgb(0, 0, 255));

    // Draw the board.

    let mut texture = RenderTexture::new(8192, 8192)
        .expect("could not create render texture");

    texture.clear(Color::WHITE);

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
                    xpos - scale * 0.41,
                    ypos - scale * 0.27,
                ));
                number_text.set_string(&format!("{number}"));
                texture.draw(&number_text);
            }
        }
    }

    let mut max_x_drawn = (width  as f32 + 1.0) * scale;
    let mut max_y_drawn = (height as f32 + 1.0) * scale;

    // Pre-compute the max width of a clue line so that we can apply
    // the user's requested horizontal squish amount.

    let max_clue_line_length = across_words
        .iter()
        .chain(down_words.iter())
        .map(|(_cell, word)| word.clone()) // words
        .flat_map(|word| clue_texts.get(&word)) // clue vecs
        .flatten() // individual clues
        .flat_map(|clue| &clue.lines) // individual lines
        .map(|line| {
            clue_text.set_string(&format!("{line}"));
            clue_text.local_bounds().width
        })
        .fold(1000.0, f32::max) - squish;

    // Draw the clues.

    let mut x = scale * (width as f32 + 1.05);
    let mut y = scale * 0.75;

    let across_count = across_words.len();
    let down_count = down_words.len();

    for c in 0..across_count+down_count {
        if c == 0 {
            header_text.set_position(Vector2f::new(x - header_dedent, y));
            header_text.set_string("Across");
            texture.draw(&header_text);
            y += header_line_height;
        }

        if c == across_count {
            y += section_sep;
            header_text.set_position(Vector2f::new(x - header_dedent, y));
            header_text.set_string("Down");
            texture.draw(&header_text);
            y += header_line_height;
        }

        let (number, word) = if c < across_count {
            across_words[c].clone()
        } else {
            down_words[c-across_count].clone()
        };

        let mut default = vec![Clue {lines: vec![word.replace(|c: char| !c.is_ascii_alphabetic(), " _ ").replace("  ", " ").trim().to_owned()]}];

        let (clue_vec, clue_color, clue_font) = if let Some(clue_vec) = clue_texts.get_mut(&word.to_uppercase()) {
            (clue_vec, Color::BLACK, &lora)
        } else {
            (&mut default, Color::rgb(240, 0, 0), &dejavusans_bold)
        };

        if clue_vec.is_empty() {
            panic!("ran out of clue entries for {word}");
        }

        let clue = clue_vec.remove(0);

        clue_text.set_position(Vector2f::new(x + clue_indent, y));
        clue_text.set_string(&format!("{number}."));
        texture.draw(&clue_text);

        let old_clue_color = clue_text.fill_color();
        let old_clue_font = clue_text.font().unwrap();
        clue_text.set_fill_color(clue_color);
        clue_text.set_font(clue_font);

        //println!("{} lines to consider", clue.lines.len());

        for i in 0..clue.lines.len() {
            let words = clue.lines[i].split(" ").collect::<Vec<_>>();
            let mut line = format!("{}", words[0]);

            //println!("{} words to consider", words.len());

            for j in 1..=words.len() {
                if j < words.len() {
                    clue_text.set_string(&format!("{line} {}", words[j]));
                }

                if j == words.len() || clue_text.local_bounds().width > max_clue_line_length {
                    clue_text.set_string(&line);
                    clue_text.set_position(Vector2f::new(x + clue_content_indent + clue_indent, y));
                    texture.draw(&clue_text);

                    max_x_drawn = max_x_drawn.max(
                        x + clue_indent + clue_content_indent + clue_text.local_bounds().width + scale * 0.5
                    );

                    max_y_drawn = max_y_drawn.max(
                        y + clue_text.local_bounds().height + scale * 0.3
                    );

                    y += clue_line_height;
                    line = format!("");
                }

                if j < words.len() {
                    line = if line == "" {
                        line + words[j]
                    } else {
                        line + " " + words[j]
                    };
                }
            }
        }

        clue_text.set_fill_color(old_clue_color);
        clue_text.set_font(old_clue_font);
        y += clue_sep - clue_line_height;

        //clue_text.set_string(&format!("{number}. {}", clue.lines[0]));
    }

    // Draw the title and author if there are any.

    if !flat_title {
        x = scale * 0.5;
        y = scale * (height as f32 + 1.08);

        if let Some(title) = title {
            title_text.set_position(Vector2f::new(x, y));
            title_text.set_string(&title);
            texture.draw(&title_text);

            max_y_drawn = max_y_drawn.max(
                y + title_text.local_bounds().height + scale * 0.3
            );

            y += title_line_height;
        }

        if let Some(author) = author {
            title_text.set_font(&dejavusans_italic);
            title_text.set_position(Vector2f::new(x, y));
            title_text.set_string(&author);
            texture.draw(&title_text);

            max_y_drawn = max_y_drawn.max(
                y + title_text.local_bounds().height + scale * 0.3
            );

            //y += title_line_height;
        }
    } else {
        y = scale * (height as f32 + 0.95);
        x = scale * 0.5;

        if let Some(title) = title {
            title_text.set_position(Vector2f::new(x, y));
            title_text.set_string(&title);
            texture.draw(&title_text);

            max_y_drawn = max_y_drawn.max(
                y + title_text.local_bounds().height + scale * 0.2
            );

            title_text.set_string(&(title + "  "));
            x += title_text.local_bounds().width;
        }

        if let Some(author) = author {
            title_text.set_font(&dejavusans_italic);
            title_text.set_string("—");
            title_text.set_position(Vector2f::new(x, y));
            texture.draw(&title_text);

            x += title_text.local_bounds().width * 1.3;

            title_text.set_string(&author);
            title_text.set_position(Vector2f::new(x, y));
            texture.draw(&title_text);

            max_y_drawn = max_y_drawn.max(
                y + title_text.local_bounds().height + scale * 0.2
            );
        }
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

    for y in 0..height {
        for x in 0..width {
            let xpos = scale * (1.00 + x as f32);
            let ypos = scale * (1.00 + y as f32);

            if let Some(letter) = board[y][x].letter {
                let letter = letter.to_ascii_uppercase();
                let glyph = dejavusans_bold.glyph(
                    letter as u32,
                    key_size as u32,
                    false,
                    0.0
                );

                key_text.set_string(&format!("{letter}"));
                key_text.set_position(Vector2f::new(
                    xpos - glyph.advance()/2.0,
                    ypos,
                ));
                texture.draw(&key_text);
            }
        }
    }

    // Now save this texture as the answer key image.

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
        .save_to_file("answer_key.png")
        .expect("failed to save image file");
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

// Create a text object.

fn create_text(font: &FBox<Font>, size: u32, color: Color) -> Text<'_> {
    let mut ret = Text::new("", &font, size);
    ret.set_fill_color(color);
    ret.set_origin(sfml::system::Vector2f::new(
        0.0, //glyph.advance() / 2.0,
        size as f32 * 0.615,
    ));
    ret
}

