enum StudentDirectory {
    StudnetAM(String),
    Student12(String),
    StudentPm(String),
}

// A function that transfers ownership of a StudentDirectory and adds it to the student list
fn add_item(studnet_list: &mut Vec<StudentDirectory>, item: StudentDirectory) {
    studnet_list.push(item);
}

// A function that borrows the grocery list reference and displays its contents
fn display_list(grocery_list: &Vec<StudentDirectory>) {
    println!("Student List:");
    for item in grocery_list {
        match item {
            StudentDirectory::StudnetAM(name) => println!("9am Student: {}", name),
            StudentDirectory::Student12(name) => println!("noon Stduent: {}", name),
            StudentDirectory::StudentPm(name) => println!("2pm Student: {}", name),
        }
    }
}

// Main function
fn main() {
    // Immutable variable
    let max_items = 5;

    // Mutable grocery list (using Vec)
    let mut student_list: Vec<StudentDirectory> = Vec::new();

    // Loop to add items until reaching max_items
    let mut count = 0;
    while count < max_items {
        count += 1;

        // Match statement to add different items to the list based on count
        let item = match count {
            1 => StudentDirectory::StudnetAM(String::from("Gavin")),
            2 => StudentDirectory::Student12(String::from("Ethan")),
            3 => StudentDirectory::StudentPm(String::from("Bryce")),
            4 => StudentDirectory::StudnetAM(String::from("Hudson")),
            _ => StudentDirectory::Student12(String::from("Brian")),
        };

        add_item(&mut student_list, item); // Transfer ownership to add item to the list
    }

    // Display the list using a borrowed reference
    display_list(&student_list);

    // Remove the last item and show the list again
    student_list.pop();
    println!("\nAfter removing the last item:");
    display_list(&student_list);
}




