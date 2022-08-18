use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
struct Furni {
    is_roomitem: String,
    id: String,
    class_name: String,
    revision: String,
    x: String,
    y: String,
    z: String,
    colors: Vec<String>,
    name: String,
    description: String,
    adurl: String,
    offerid: String,
    buyout: String,
    customparams: String,
    specialtype: String,
    rentofferid: String,
    rentbuyout: String,
    bc: String,
    cansiton: String,
    canstandon: String
}

fn process_furni(node: roxmltree::Node) -> Furni {

    let mut output = Furni {
        is_roomitem: if node.parent().unwrap().tag_name().name().eq("roomitemtypes") { "s".to_string() } else { "i".to_string() },
        id: node.attribute("id").unwrap().to_string(),
        class_name: node.attribute("classname").unwrap().to_string(),
        revision: String::new(),
        name: String::new(),
        description: String::new(),
        x: String::new(),
        y: String::new(),
        z: "0".to_string(),
        colors: Vec::new(),
        adurl: String::new(),
        offerid: String::new(),
        buyout: String::new(),
        customparams: String::new(),
        specialtype: String::new(),
        rentofferid: String::new(),
        rentbuyout: String::new(),
        bc: String::new(),
        cansiton: String::new(),
        canstandon: String::new()
    };

    for child in node.children() {
        match child.tag_name().name() {
            "name" => match child.text() {
                Some(name) => output.name = name.to_string(),
                None => {
                    //println!("Warning: Furni with classname {} has no name attribute", output.class_name.as_str());
                    output.name.push_str(output.class_name.as_str());
                    output.name.push_str(" name");
                }
            },
            "description" => match child.text() {
                Some(desc) => output.description = desc.to_string(),
                None => {
                    //println!("Warning: Furni with classname {} has no description attribute", output.class_name.as_str());
                    output.name.push_str(output.class_name.as_str());
                    output.name.push_str(" desc");
                }
            },
            "revision" => match child.text() {
                Some(rev) => output.revision = rev.to_string(),
                None => {} 
            },
            "xdim" => match child.text() {
                Some(x) => output.x = x.to_string(),
                None => output.x = "0".to_string()
            },
            "ydim" => match child.text() {
                Some(y) => output.y = y.to_string(),
                None => output.y = "0".to_string()
            },
            "partcolors" => {
                for second_child in child.children() {
                    output.colors.push(second_child.text().unwrap().to_string());
                }
            },
            "adurl" => match child.text() {
                Some(adurl) => output.adurl = adurl.to_string(),
                None => {}
            },
            "offerid" => match child.text() {
                Some(offerid) => output.offerid = offerid.to_string(),
                None => {}
            },
            "buyout" => match child.text() {
                Some(buyout) => output.buyout = (if buyout.eq("1") { "true" } else { "false"} ).to_string(),
                None => {}
            },
            "customparams" => match child.text() {
                Some(customparams) => output.customparams = customparams.to_string(),
                None => {}
            },
            "specialtype" => match child.text() {
                Some(specialtype) => output.specialtype = specialtype.to_string(),
                None => {}
            },
            "rentofferid" => match child.text() {
                Some(rentofferid) => output.rentofferid = rentofferid.to_string(),
                None => {}
            },
            "rentbuyout" => match child.text() {
                Some(rentbuyout) => output.rentbuyout = if rentbuyout.eq("1") {"true".to_string()} else {"false".to_string()},
                None => {}
            },
            "bc" => match child.text() {
                Some(bc) => output.bc = if bc.eq("1") {"true".to_string()} else {"false".to_string()},
                None => {}
            },
            "cansiton" => match child.text() {
                Some(cansiton) => output.cansiton = if cansiton.eq("1") {"true".to_string()} else {"false".to_string()},
                None => {}
            },
            "canstandon" => match child.text() {
                Some(canstandon) => output.canstandon = if canstandon.eq("1") {"true".to_string()} else {"false".to_string()},
                None => {}
            },
            _ => {}
        }
    }

    return output;
}

fn furni_to_str(input: Furni) -> String {
    let mut output = ("[\"").to_string();
    output.push_str(input.is_roomitem.as_str());
    output.push_str("\",\"");
    output.push_str(input.id.as_str());
    output.push_str("\",\"");
    output.push_str(input.class_name.as_str());
    output.push_str("\",\"");
    output.push_str(input.revision.as_str());
    output.push_str("\",\"");
    output.push_str(input.x.as_str());
    output.push_str("\",\"");
    output.push_str(input.y.as_str());
    output.push_str("\",\"");
    output.push_str(input.z.as_str());
    output.push_str("\",\"");
    for color in input.colors {
       output.push_str(color.as_str());
       output.push_str(",");
    }
    output.replace_range(output.len()-1..,"\",\"");
    output.push_str(input.name.as_str());
    output.push_str("\",\"");
    output.push_str(input.description.as_str());
    output.push_str("\",\"");
    output.push_str(input.adurl.as_str());
    output.push_str("\",\"");

    if !input.offerid.is_empty() {
        output.push_str(input.offerid.as_str()); 
        output.push_str("\",\"");
    }

    if !input.buyout.is_empty() {
        output.push_str(input.buyout.as_str());
        output.push_str("\",\"");
    }
    
    if !input.rentofferid.is_empty() {
        output.push_str(input.rentofferid.as_str());
        output.push_str("\",\"");
    }
    
    if !input.rentbuyout.is_empty() {
        output.push_str(input.rentofferid.as_str());
        output.push_str("\",\"");
    }
    
    output.push_str(input.customparams.as_str());
    output.push_str("\",\"");
    output.push_str(input.specialtype.as_str());

    if !input.bc.is_empty() {
        output.push_str("\",\"");
        output.push_str(input.bc.as_str());
        
        output.push_str("\",\"");
        if input.is_roomitem.eq("s") {
            output.push_str(input.canstandon.as_str());
            output.push_str("\",\"");
            output.push_str(input.cansiton.as_str());
        }
    }

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
                    match node.attribute("id") {
                        Some(_) => curr_furnis.push(process_furni(node)),
                        None => {
                            println!("Warning: furni with classname {} has no id. Skipping...",classname);
                            continue;
                        }
                    }
                },
                None => println!("Warning: no classname. Skipping...")
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
