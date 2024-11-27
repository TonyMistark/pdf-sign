use chrono::{DateTime, Local};
use lopdf::{Document, Object};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: pdf_sign <path_to_pdf>");
        std::process::exit(1);
    }

    let doc = Document::load(&args[1]).unwrap();
    let date_list = extract_dates(&doc);
    print_date_list(date_list);
}

fn print_date_list(date_list: Vec<DateTime<Local>>) {
    // for dt in date_list.iter() {
    //     println!("{:?}", dt.to_string());
    // }
    let date_strings: Vec<String> = date_list.iter().map(|dt| dt.to_string()).collect();
    let result = date_strings.join(", ");
    println!("{}", result);
}

fn extract_dates(doc: &Document) -> Vec<DateTime<Local>> {
    let mut date_list: Vec<DateTime<Local>> = Vec::new();

    for page in doc.page_iter() {
        let page_annotations = doc.get_page_annotations(page).unwrap();
        for annotation in page_annotations {
            if annotation.type_is(b"Annot") {
                if let Ok(Object::Reference(sign_obj_id)) = annotation.get(b"V") {
                    if let Ok(sin_data) = doc.get_object(*sign_obj_id) {
                        if let Ok(sin_data_dict) = sin_data.as_dict() {
                            if let Ok(date_obj) = sin_data_dict.get(b"M") {
                                date_list.push(date_obj.as_datetime().unwrap());
                                // if let Ok(DateTime(dt)) = date_obj.as_datetime() {
                                //     println!("date time: {:?}", dt);
                                // }
                            }
                        }
                    }
                }
            }
        }
    }
    date_list
}
