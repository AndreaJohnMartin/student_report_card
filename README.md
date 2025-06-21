# 🧾 Rust Student Report Card Generator

This is a simple yet powerful Rust-based **console application** that takes student information, calculates average marks, assigns a grade, and generates a **PDF report card** using the [`genpdf`](https://crates.io/crates/genpdf) library.

---

## ✨ Features

- 🔢 Input student name, total marks, and number of subjects
- 🧮 Calculates average using a custom function
- 🏅 Assigns grade:
  - A: 90+
  - B: 75–89
  - C: 60–74
  - D: Below 60
- 📄 Generates a clean, formatted PDF report card
- 📁 Saves output in the `/output` folder

---

## 📦 Dependencies

- `genpdf` - for PDF generation
- `std::io` - for console input/output
   
---

## 🛠 Sample Output
Report Card for J ANDREA
Name         : J ANDREA
Total Marks  : 510
Subjects     : 6
Average      : 85.00
Grade        : B

📄 PDF saved as [output/report_card_J_ANDREA.pdf](https://github.com/AndreaJohnMartin/student_report_card/blob/main/output/report_card_J_ANDREA.pdf)
   

