use std::collections::HashMap;

pub fn opteric(input: &String) -> (HashMap<String, Option<String>>, String) {
    let chars: Vec<char> = input.chars().collect();
    
    let mut map: HashMap<String, Option<String>> = HashMap::new();
    let mut content = String::new();
    let mut parsing = String::new();    
    let mut opt_content = String::new();
    
    let mut is_parsing = false;
    let mut is_parsing_content = false;
    let mut is_first_whitespace = true;
    let mut index: usize = 0;
    
    let len: usize = input.len() + 1;
    
    loop {
        let ch = chars.get(index);
        let ch_next = chars.get(index + 1);
        let ch_prev: Option<&char> = if index == 0 { None } else { chars.get(index - 1) };
        
        if is_parsing {
            if ch == None || ch.unwrap().is_whitespace() {
                is_parsing = false;
                
                if ch_next != None {
                    let ch_next_data = ch_next.unwrap();
                    
                    if !ch_next_data.is_whitespace() && *ch_next_data != '-' {
                        is_parsing_content = true;
                        continue;
                    }
                }
                
                map.insert(parsing.clone(), None);
                parsing.clear();
            } else {
                parsing.push(*ch.unwrap());
            }
        } else if is_parsing_content {
            let ch_data: char = if ch == None { 'h' } else { *ch.unwrap() };
            
            if !ch_data.is_whitespace() {
                is_first_whitespace = false;
            }
            
            if ch == None || (ch_next != None && *ch_next.unwrap() == '-' && ch_data.is_whitespace()) {
                map.insert(parsing.clone(), Some(opt_content.clone()));
                
                is_first_whitespace = true;
                is_parsing_content = false;
                
                parsing.clear();
                opt_content.clear();
            } else if !is_first_whitespace {
                opt_content.push(ch_data);
            }
        } else if (ch_prev == None || ch_prev.unwrap().is_whitespace()) && (ch != None && *ch.unwrap() == '-') {
            let truncate_len = if index == 0 { 0 } else { index - 1 };
            
            index += 1;
            is_parsing = true;
            
            if ch_next != None && *ch_next.unwrap() == '-' {
                index += 1;
            }
            
            if content.len() == 0 {
                if ch_prev == None {
                    content = input.clone();
                } else {
                    content = input[0..truncate_len].to_string();
                }
            }
            
            parsing.push(*chars.get(index).unwrap());
        }
        
        index += 1;
        
        if index >= len {
            break;
        }
    }
    
    if content.len() == 0 {
        content = input.clone();
    }
    
    (map, content)
}