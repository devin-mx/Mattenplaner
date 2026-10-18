use crate::models::{Color, Layout, MatPosition, MatTile};
use anyhow::{Ok, Result};
use rust_xlsxwriter::FormatBorder::Medium;
use rust_xlsxwriter::{Format, Worksheet, workbook::Workbook};

#[allow(clippy::struct_excessive_bools)]
struct Borders {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

fn format_builder(color: &str, borders: &Borders) -> Format {
    let mut f = Format::new()
        .set_font_color(color)
        .set_background_color(color);

    if borders.top {
        f = f.set_border_top(Medium);
    }
    if borders.bottom {
        f = f.set_border_bottom(Medium);
    }
    if borders.left {
        f = f.set_border_left(Medium);
    }
    if borders.right {
        f = f.set_border_right(Medium);
    }

    f
}

fn washed(color: &str, factor: f32) -> String {
    let hex = u32::from_str_radix(color.trim_start_matches('#'), 16).unwrap();
    let factor = factor.clamp(0.0, 1.0);

    // 1. Extract and normalize to 0.0 - 1.0

    let [_, r, g, b] = hex.to_be_bytes();

    let red = f32::from(r) / 255.0;
    let green = f32::from(g) / 255.0;
    let blue = f32::from(b) / 255.0;

    // 2. Decode sRGB to Linear RGB (remove gamma compression)
    let red_lin = red.powf(2.2);
    let green_lin = green.powf(2.2);
    let blue_lin = blue.powf(2.2);

    // 3. Mix with pure white (1.0) in linear space
    let new_red_lin = red_lin + (1.0 - red_lin) * factor;
    let new_green_lin = green_lin + (1.0 - green_lin) * factor;
    let new_blue_lin = blue_lin + (1.0 - blue_lin) * factor;

    // 4. Encode back to sRGB (apply gamma compression)
    let gamma_inv = 1.0 / 2.2;

    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let new_red = (new_red_lin.powf(gamma_inv) * 255.0).round() as u32;

    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let new_green = (new_green_lin.powf(gamma_inv) * 255.0).round() as u32;

    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let new_blue = (new_blue_lin.powf(gamma_inv) * 255.0).round() as u32;

    // Repack
    let hex = (new_red << 16) | (new_green << 8) | new_blue;
    let color = format!("#{hex:06X}");
    color
}

fn set_cell_shape_to_square(worksheet: &mut Worksheet, length: u16) -> Result<()> {
    for i in 0..length {
        worksheet.set_column_width_pixels(i, 38)?;
        worksheet.set_row_height_pixels(u32::from(i), 38)?;
    }
    Ok(())
}

fn print_tatami_on_worksheet(
    worksheet: &mut Worksheet,
    offset: usize,
    mats: &[MatTile],
) -> Result<()> {
    for mat in mats {
        let (text, mut single_color, mut double_color) = match mat.color {
            Color::Yellow => ("Y", "#FFFF00".to_string(), "#D5B60A".to_string()),
            Color::Blue => ("B", "#1D4ED8".to_string(), "#1E3A8A".to_string()),
            Color::None => continue,
        };

        if !mat.owned {
            single_color = washed(&single_color, 0.75);
            double_color = washed(&double_color, 0.75);
        }

        match mat.position {
            MatPosition::Single(pos) => {
                let fmt = format_builder(
                    &single_color,
                    &Borders {
                        top: true,
                        bottom: true,
                        left: true,
                        right: true,
                    },
                );
                worksheet.write_with_format(
                    u32::try_from(pos.y + offset)?,
                    u16::try_from(pos.x)?,
                    text,
                    &fmt,
                )?;
            }
            MatPosition::Double {
                first,
                second,
                layout,
            } => {
                let (first_fmt, second_fmt) = match layout {
                    Layout::Horizontal => (
                        format_builder(
                            &double_color,
                            &Borders {
                                top: true,
                                bottom: true,
                                left: true,
                                right: false,
                            },
                        ),
                        format_builder(
                            &double_color,
                            &Borders {
                                top: true,
                                bottom: true,
                                left: false,
                                right: true,
                            },
                        ),
                    ),
                    Layout::Vertical => (
                        format_builder(
                            &double_color,
                            &Borders {
                                top: true,
                                bottom: false,
                                left: true,
                                right: true,
                            },
                        ),
                        format_builder(
                            &double_color,
                            &Borders {
                                top: false,
                                bottom: true,
                                left: true,
                                right: true,
                            },
                        ),
                    ),
                };
                worksheet.write_with_format(
                    u32::try_from(first.y + offset)?,
                    u16::try_from(first.x)?,
                    text,
                    &first_fmt,
                )?;
                worksheet.write_with_format(
                    u32::try_from(second.y + offset)?,
                    u16::try_from(second.x)?,
                    text,
                    &second_fmt,
                )?;
            }
        }
    }
    Ok(())
}

pub fn write_output_excel(pages: &[Vec<MatTile>], name: &str) -> Result<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let length = u16::try_from(pages.len() * pages[0].len())?;

    set_cell_shape_to_square(worksheet, length)?;

    for (count, mats) in pages.iter().enumerate() {
        let offset = count * (mats.len().isqrt() + 5);
        print_tatami_on_worksheet(worksheet, offset, mats)?;
    }

    let path_out = format!("excel_data/output-{name}.xlsx");
    workbook.save(path_out)?;

    Ok(())
}
