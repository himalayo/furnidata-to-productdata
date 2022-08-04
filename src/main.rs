use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
struct Furni {
    class_name: String,
    name: String,
    description: String
}

fn process_furni(node: roxmltree::Node) -> Furni {
    let mut output = Furni {
        class_name: node.attribute("classname").unwrap().to_string(),
        name: String::new(),
        description: String::new()
    };

    for child in node.children() {
        match child.tag_name().name() {
            "name" => match child.text() {
                Some(name) => output.name = name.to_string(),
                None => {
                    println!("Warning: Furni with classname {} has no name attribute", output.class_name.as_str());
                    output.name.push_str(output.class_name.as_str());
                    output.name.push_str(" name");
                }
            },
            "description" => match child.text() {
                Some(desc) => output.description = desc.to_string(),
                None => {
                    println!("Warning: Furni with classname {} has no description attribute", output.class_name.as_str());
                    output.name.push_str(output.class_name.as_str());
                    output.name.push_str(" desc");
                }
            },
            _ => {}
        }
    }

    return output;
}

fn furni_to_str(input: Furni) -> String {
    let mut output = ("[\"").to_string();
    output.push_str(input.class_name.as_str());
    output.push_str("\",\"");
    output.push_str(input.name.as_str());
    output.push_str("\",\"");
    output.push_str(input.description.as_str());
    output.push_str("\"]");
    return output;

}

fn productdata_line(furni_vec: Vec<Furni>) -> String {
    let mut output = String::new();
    
    output.push_str("[");
    for furni in furni_vec.into_iter() {
        output.push_str(furni_to_str(furni).as_str());
        output.push_str(",");
    }
    output.replace_range(output.len()-1..,"]");

    return output;
}

fn main() -> std::io::Result<()> {
    let mut raw_furnidata_file = File::open("furnidata.xml")?;
    let mut raw_furnidata_contents = String::new();
    raw_furnidata_file.read_to_string(&mut raw_furnidata_contents)?;
    
    let mut productdata = String::new();
    
    //Check for product data
    match File::open("proudctdata.txt") {
        Ok(mut file) => {
            match file.read_to_string(&mut productdata) {
                Ok(_) => {},
                Err(_) => println!("Warning: couldn't read from product data"),
            };
        },
        Err(_) => println!("Warning: no proudct data"),
    };
    
    let furnidata = match roxmltree::Document::parse(&raw_furnidata_contents) {
        Ok(parsed_xml) => parsed_xml,
        Err(_) => panic!("Error: Couldn't parse furnidata"),
    }; 
    
    
    let mut furnis: Vec<Vec<Furni>> = Vec::new();
    let mut curr_furnis: Vec<Furni> = Vec::new();
    for node in furnidata.descendants() {

        if node.tag_name().name().eq("furnitype") {
            match node.attribute("classname") {
                Some(classname) => {
                    curr_furnis.push(process_furni(node));
                },
                None => println!("Warning: no classname")
            }
        }
        
        if curr_furnis.len() == 50 {
            furnis.push(curr_furnis.clone());
            curr_furnis.clear();
            curr_furnis.shrink_to_fit();
        }
    }
    
    let mut output_string = String::new(); 
    for line in furnis {
        output_string.push_str(productdata_line(line).as_str());
        output_string.push_str("\n");
    }
    if curr_furnis.len() < 50 {
        output_string.push_str(productdata_line(curr_furnis).as_str());
        output_string.push_str("\n");
    }

    let mut output_file = File::create("output.txt")?;
    output_file.write_all(output_string.as_bytes());
    Ok(())
}
