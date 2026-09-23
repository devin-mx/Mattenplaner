use crate::models::{Color, Grid, Layout, MatPostion};
use rust_xlsxwriter::FormatBorder::Medium;
use rust_xlsxwriter::{Format, XlsxError, workbook::Workbook};

pub fn writer_function(grid: Grid) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    for i in 0..100u16 {
        worksheet.set_column_width_pixels(i, 38)?;
        worksheet.set_row_height_pixels(i as u32, 38)?;
    }

    let yellow_single_mat = Format::new()
        .set_background_color("#FFFF00")
        .set_border(Medium)
        .set_font_color("#FFFF00");

    let yellow_double_top_mat = Format::new()
        .set_background_color("#D5B60A")
        .set_border_top(Medium)
        .set_border_left(Medium)
        .set_border_right(Medium)
        .set_font_color("#D5B60A");

    let yellow_double_bottom_mat = Format::new()
        .set_background_color("#D5B60A")
        .set_border_bottom(Medium)
        .set_border_left(Medium)
        .set_border_right(Medium)
        .set_font_color("#D5B60A");

    let yellow_double_left_mat = Format::new()
        .set_background_color("#D5B60A")
        .set_border_top(Medium)
        .set_border_left(Medium)
        .set_border_bottom(Medium)
        .set_font_color("#D5B60A");

    let yellow_double_right_mat = Format::new()
        .set_background_color("#D5B60A")
        .set_border_bottom(Medium)
        .set_border_top(Medium)
        .set_border_right(Medium)
        .set_font_color("#D5B60A");

    let blue_single_mat = Format::new()
        .set_border(Medium)
        .set_background_color("#0000FF")
        .set_font_color("#0000FF");

    let blue_double_top_mat = Format::new()
        .set_background_color("#00008B")
        .set_border_top(Medium)
        .set_border_left(Medium)
        .set_border_right(Medium)
        .set_font_color("#00008B");

    let blue_double_bottom_mat = Format::new()
        .set_background_color("#00008B")
        .set_border_bottom(Medium)
        .set_border_left(Medium)
        .set_border_right(Medium)
        .set_font_color("#00008B");

    let blue_double_left_mat = Format::new()
        .set_background_color("#00008B")
        .set_border_top(Medium)
        .set_border_left(Medium)
        .set_border_bottom(Medium)
        .set_font_color("#00008B");

    let blue_double_right_mat = Format::new()
        .set_background_color("#00008B")
        .set_border_bottom(Medium)
        .set_border_right(Medium)
        .set_border_top(Medium)
        .set_font_color("#00008B");

    for mat in grid.mats {
        match mat.position {
            MatPostion::Double {
                first,
                second,
                layout,
            } => match layout {
                Layout::Vertical => {
                    if mat.color == Color::Yellow {
                        worksheet.write_with_format(
                            first.y as u32,
                            first.x as u16,
                            "Y",
                            &yellow_double_top_mat,
                        )?;
                        worksheet.write_with_format(
                            second.y as u32,
                            second.x as u16,
                            "Y",
                            &yellow_double_bottom_mat,
                        )?;
                    } else if mat.color == Color::Blue {
                        worksheet.write_with_format(
                            first.y as u32,
                            first.x as u16,
                            "B",
                            &blue_double_top_mat,
                        )?;
                        worksheet.write_with_format(
                            second.y as u32,
                            second.x as u16,
                            "B",
                            &blue_double_bottom_mat,
                        )?;
                    }
                }
                Layout::Horizontal => {
                    if mat.color == Color::Yellow {
                        worksheet.write_with_format(
                            first.y as u32,
                            first.x as u16,
                            "Y",
                            &yellow_double_left_mat,
                        )?;
                        worksheet.write_with_format(
                            second.y as u32,
                            second.x as u16,
                            "Y",
                            &yellow_double_right_mat,
                        )?;
                    } else if mat.color == Color::Blue {
                        worksheet.write_with_format(
                            first.y as u32,
                            first.x as u16,
                            "B",
                            &blue_double_left_mat,
                        )?;
                        worksheet.write_with_format(
                            second.y as u32,
                            second.x as u16,
                            "B",
                            &blue_double_right_mat,
                        )?;
                    }
                }
            },
            MatPostion::Singe(pos) => {
                if mat.color == Color::Yellow {
                    worksheet.write_with_format(
                        pos.y as u32,
                        pos.x as u16,
                        "Y",
                        &yellow_single_mat,
                    )?;
                } else if mat.color == Color::Blue {
                    worksheet.write_with_format(
                        pos.y as u32,
                        pos.x as u16,
                        "B",
                        &blue_single_mat,
                    )?;
                }
            }
        }
    }

    workbook.save("excel_data/output.xlsx")?;

    Ok(())
}
