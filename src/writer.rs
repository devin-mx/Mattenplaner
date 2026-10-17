use std::error::Error;

use crate::models::{Color, Layout, Mat, MatPosition};
use anyhow::Result;
use rust_xlsxwriter::FormatBorder::Medium;
use rust_xlsxwriter::{Format, XlsxError, workbook::Workbook};

fn format_builder(color: &str, top: bool, bottom: bool, left: bool, right: bool) -> Format {
    let mut f = Format::new()
        .set_font_color(color)
        .set_background_color(color);

    if top {
        f = f.set_border_top(Medium);
    }
    if bottom {
        f = f.set_border_bottom(Medium);
    }
    if left {
        f = f.set_border_left(Medium);
    }
    if right {
        f = f.set_border_right(Medium);
    }

    f
}

fn washed(color: &str, factor: f32) -> String {
    let hex = u32::from_str_radix(color.trim_start_matches('#'), 16).unwrap();
    let factor = factor.clamp(0.0, 1.0);

    // 1. Extract and normalize to 0.0 - 1.0
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;

    // 2. Decode sRGB to Linear RGB (remove gamma compression)
    let r_lin = r.powf(2.2);
    let g_lin = g.powf(2.2);
    let b_lin = b.powf(2.2);

    // 3. Mix with pure white (1.0) in linear space
    let new_r_lin = r_lin + (1.0 - r_lin) * factor;
    let new_g_lin = g_lin + (1.0 - g_lin) * factor;
    let new_b_lin = b_lin + (1.0 - b_lin) * factor;

    // 4. Encode back to sRGB (apply gamma compression)
    let gamma_inv = 1.0 / 2.2;
    let new_r = (new_r_lin.powf(gamma_inv) * 255.0).round() as u32;
    let new_g = (new_g_lin.powf(gamma_inv) * 255.0).round() as u32;
    let new_b = (new_b_lin.powf(gamma_inv) * 255.0).round() as u32;

    // Repack
    let hex = (new_r << 16) | (new_g << 8) | new_b;
    let color = format!("#{hex:06X}");
    color
}

pub fn writer_function(pages: &[Vec<Vec<Mat>>], name: &str) -> Result<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    for i in 0..(pages.len() * pages[0][0].len()) as u16 {
        worksheet.set_column_width_pixels(i, 38)?;
        worksheet.set_row_height_pixels(i as u32, 38)?;
    }

    let mut count = 0;
    for section in pages {
        for mats in section {
            let offset = count * (mats.len().isqrt() + 5);

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
                        let fmt = format_builder(&single_color, true, true, true, true);
                        worksheet.write_with_format(
                            (pos.y + offset) as u32,
                            pos.x as u16,
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
                                format_builder(&double_color, true, true, true, false),
                                format_builder(&double_color, true, true, false, true),
                            ),
                            Layout::Vertical => (
                                format_builder(&double_color, true, false, true, true),
                                format_builder(&double_color, false, true, true, true),
                            ),
                        };
                        worksheet.write_with_format(
                            (first.y + offset) as u32,
                            first.x as u16,
                            text,
                            &first_fmt,
                        )?;
                        worksheet.write_with_format(
                            (second.y + offset) as u32,
                            second.x as u16,
                            text,
                            &second_fmt,
                        )?;
                    }
                }
            }
            count += 1;
        }
    }

    let path_out = format!("excel_data/output-{name}.xlsx");
    workbook.save(path_out)?;

    Ok(())
}
