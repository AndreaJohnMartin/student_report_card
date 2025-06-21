use genpdf::{elements, style};
use genpdf::Element;
use std::fs;
use std::io::{self, Write};

fn calculate_average(total_marks: u32, num_subjects: u32) -> f32 {
    total_marks as f32 / num_subjects as f32
}

fn calculate_grade(average: f32) -> String {
    match average {
        avg if avg >= 90.0 => "A".to_string(),
        avg if avg >= 75.0 => "B".to_string(),
        avg if avg >= 60.0 => "C".to_string(),
        _ => "D".to_string(),
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    println!("📘 Welcome to the Rust Student Report Card Generator!\n");

    let name = get_input("👤 Enter Student Name: ");
    let total_marks: u32 = get_input("✏️  Enter Total Marks: ")
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("⚠️  Invalid input! Defaulting total marks to 0.");
            0
        });

    let num_subjects: u32 = get_input("📚 Enter Number of Subjects: ")
        .parse()
        .unwrap_or_else(|_| {
            eprintln!("⚠️  Invalid input! Defaulting subjects to 1.");
            1
        });

    let average = calculate_average(total_marks, num_subjects);
    let grade = calculate_grade(average);

    println!("\n✅ Student data processed successfully.");

    // ✅ Correct way to load fonts using from_files
    let font = genpdf::fonts::from_files("./fonts", "LiberationSans", None)
        .expect("Failed to load font. Make sure LiberationSans-Regular.ttf exists in /fonts");

    let mut doc = genpdf::Document::new(font);
    doc.set_title("Student Report Card");
    doc.set_minimal_conformance();

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    let mut layout = elements::LinearLayout::vertical();
    layout.push(
        elements::Paragraph::new(format!("Report Card for {}", name))
            .styled(style::Style::new().bold().with_font_size(20)),
    );
    layout.push(elements::Break::new(1));
    layout.push(elements::Paragraph::new(format!("Name         : {}", name)));
    layout.push(elements::Paragraph::new(format!("Total Marks : {}", total_marks)));
    layout.push(elements::Paragraph::new(format!("Subjects     : {}", num_subjects)));
    layout.push(elements::Paragraph::new(format!("Average      : {:.2}", average)));
    layout.push(elements::Paragraph::new(format!("Grade        : {}", grade)));

    doc.push(layout);

    let sanitized_name = name.replace(" ", "_");
    let filename = format!("report_card_{}.pdf", sanitized_name);

    fs::create_dir_all("output").unwrap_or_else(|_| println!("Could not create 'output' directory."));
    let filepath = format!("output/{}", filename);

    doc.render_to_file(&filepath).expect("Failed to write PDF");

    println!("\n📄 Report card saved as: {}", filepath);
    println!("🧾 You can open the PDF from the 'output' folder.\n");
}
