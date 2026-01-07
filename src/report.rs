pub mod report {
    use docx_rs::*;

    pub fn print_to_document(week_num: i32) -> Result<(), Box<dyn std::error::Error>> {
        let path = std::path::Path::new("./report.docx");
        let file = std::fs::File::create(path).unwrap();

        let new_count = 0;
        let resolved_count = 0;
        let open_count = 0;
        let in_progress_count = 0;
        let total_count = 0;
        let table_issues = Table::new(vec![
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("New")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text(new_count.to_string())
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
            ]),
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("Resolved")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(resolved_count.to_string())
                            .fonts(RunFonts::new().ascii("Arial")),
                    ),
                ),
            ]),
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("Open")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(open_count.to_string())
                            .fonts(RunFonts::new().ascii("Arial")),
                    ),
                ),
            ]),
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("In Progress")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(in_progress_count.to_string())
                            .fonts(RunFonts::new().ascii("Arial")),
                    ),
                ),
            ]),
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("Total")
                                .bold()
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(total_count.to_string())
                            .fonts(RunFonts::new().ascii("Arial")),
                    ),
                ),
            ]),
        ])
        .width(10000, WidthType::Dxa)
        .set_grid(vec![2000, 2000, 2000, 2000, 2000]);

        let table_categories = Table::new(vec![
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("Category ID")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("Category Type")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("# Issues Reported")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
            ]),
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("C001")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("Transaction Errors/Delays/Info")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(resolved_count.to_string())
                            .fonts(RunFonts::new().ascii("Arial")),
                    ),
                ),
            ]),
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("C002")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("Protocol related questions (blocks, finalization, chain specifics)")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(resolved_count.to_string())
                            .fonts(RunFonts::new().ascii("Arial")),
                    ),
                ),
            ]),
                    TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("C003")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("Staking Rewards")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(resolved_count.to_string())
                            .fonts(RunFonts::new().ascii("Arial")),
                    ),
                ),
            ]),
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("C004")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("Comms/Events/Marketing/Business")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(resolved_count.to_string())
                            .fonts(RunFonts::new().ascii("Arial")),
                    ),
                ),
            ]),
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("C005")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("AHM")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(resolved_count.to_string())
                            .fonts(RunFonts::new().ascii("Arial")),
                    ),
                ),
            ]),
        ])
        .width(10000, WidthType::Dxa)
        .set_grid(vec![2000, 2000, 2000, 2000, 2000]);

        let table_issues_per_partner = Table::new(vec![
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("External Partner")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("Number of Questions")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
            ]),
            TableRow::new(vec![
                TableCell::new()
                    .add_paragraph(
                        Paragraph::new().add_run(
                            Run::new()
                                .add_text("")
                                .fonts(RunFonts::new().ascii("Arial")),
                        ),
                    )
                    .width(2000, WidthType::Dxa),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .add_text(resolved_count.to_string())
                            .fonts(RunFonts::new().ascii("Arial")),
                    ),
                ),
            ]),
        ])
        .width(10000, WidthType::Dxa)
        .set_grid(vec![2000, 2000, 2000, 2000, 2000]);

        Docx::new()
            .add_paragraph(
                Paragraph::new().add_run(
                    Run::new()
                        .add_text(&format!("Week {}", week_num))
                        .size(30)
                        .fonts(RunFonts::new().ascii("Arial")), // Size in half-points (28 = 14pt),
                ),
            )
            .add_paragraph(
                Paragraph::new().add_run(
                    Run::new()
                        .add_text("📅 Tracking Period")
                        .bold()
                        .fonts(RunFonts::new().ascii("Arial")),
                ),
            )
            .add_paragraph(
                Paragraph::new().add_run(
                    Run::new()
                        .add_text("#️⃣ Issue Status Overview")
                        .bold()
                        .fonts(RunFonts::new().ascii("Arial")),
                ),
            )
            .add_table(table_issues)
            .add_paragraph(
                Paragraph::new().add_run(
                    Run::new()
                        .add_text("🗂️ Issue Categories & Impact Analysis")
                        .bold()
                        .fonts(RunFonts::new().ascii("Arial")),
                ),
            )
            .add_table(table_categories)
            .add_paragraph(
                Paragraph::new().add_run(
                    Run::new()
                        .add_text("🙋 Number of Questions/Issues per External Partner")
                        .bold()
                        .fonts(RunFonts::new().ascii("Arial")),
                ),
            )
            .add_table(table_issues_per_partner)
            .build()
            .pack(file)?;

        Ok(())
    }
}
