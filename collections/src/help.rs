pub mod helpers {
    use std::collections::HashMap;
    use unicode_segmentation::UnicodeSegmentation;
    use std::iter::FromIterator;
    #[derive(Debug)]
    pub struct MMM(f32, Option<i32>, i32);
    pub fn mean_mode_median(list:&mut Vec<i32>) -> MMM {
        let mut hash: HashMap<i32, i32> = HashMap::new();
        let mut sum: i32 = 0;
        let size = list.len();
        for elem in list.into_iter() {
            sum+= *elem;
            let count = hash.entry(*elem).or_insert(0);
            *count+=1;
        }
        
        let mut mode = Some(0);
        let mut mode_val = 0;
        for (key, value) in hash.into_iter() {
            if value > mode_val {
                mode_val = value;
                mode = Some(key);
            }
        }
        if mode_val == 1 {
            mode = Option::None;
        }
        sort(list, 0, size - 1);
    
        let mid = ((size - 1) / 2 )as usize;
    
    
        return MMM( (sum / size as i32) as f32, mode , list[mid]);
    }

    pub fn sort(list: &mut Vec<i32>, i: usize, j: usize) -> () {
        if i >= j {
            return;
        }
        let mid = i + ((j - i) / 2) as usize;
        sort(list, i, mid);
        sort(list, mid + 1, j);
        let n1 = (mid + 1) - i;
        let n2 = (j + 1) - (mid + 1);
        let mut l = vec![0; n1];
        l.copy_from_slice(&list[i..(mid + 1)]);
        let mut r = vec![0; n2];
        r.copy_from_slice(&list[(mid + 1)..(j + 1)]);
    
        let mut pointer_left: usize = 0;
        let mut pointer_right: usize = 0;
        let mut k: usize = i;
    
        while pointer_left < n1 && pointer_right < n2 {
            if l[pointer_left] <= r[pointer_right] {
                list[k] = l[pointer_left];
                pointer_left += 1;
                k += 1;
            } else {
                list[k] = r[pointer_right];
                pointer_right += 1;
                k += 1;
            }
        }
        while pointer_left < n1 {
            list[k] = l[pointer_left];
            k += 1;
            pointer_left += 1;
        }
        while pointer_right < n2 {
            list[k] = r[pointer_right];
            k += 1;
            pointer_right += 1;
        }
    }

    pub fn to_pig_latin(string: &str) -> String {
        let vowels = ["a", "e", "i", "o", "u"];
        let words = string.unicode_words().collect::<Vec<&str>>();
        let mut result: Vec<String> = Vec::new();
        for word in words.into_iter() {
            let g = word.graphemes(true).collect::<Vec<&str>>();
            if vowels.contains(&(&g[0].to_string().to_lowercase()[0..])) {
                result.push(format!("{}-hay ", word)); 
            }
            else {
                let mut  rest = String::new();
                for ch in g[1..].into_iter() {
                    rest = rest + ch;
                }
                result.push(format!("{}-{}ay ", &rest[0..], g[0] ));
            }
        }
        String::from_iter(result)
    }
    use std::io;
    pub fn dept() -> () {
        let mut depts: HashMap<String, Vec<String>> = HashMap::new();
        let mut buffer = "".to_string();
        println!("Add a user to a department using this format `Add Sally to Engineering`, `Add Amir to Sales`\nTo retrieve the users in a department, type something like `List Engineering`, type `q` to exit the program");
        loop {
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => {
                 let input_split = buffer.trim().split(' ').collect::<Vec<&str>>();
                 let command = input_split[0];
                 if command == "Add" &&  input_split.len() == 4 {
                    let dept = input_split[3];
                    let user = input_split[1];
                    let entry = depts.entry(String::from(dept)).or_insert(Vec::new());
                    entry.push(user.to_string());
                    entry.sort();
                 }

                 else if command == "List" && input_split.len() == 2 {
                    let dept = input_split[1];
                    println!("Members of {}", dept);
                    match depts.get(dept) {
                        Some(value) => {
                            for user in value.into_iter() {
                                println!("{}", user);
                            }
                        },
                        None => ()
                    }
                 }
                 else if command == "q" && command.len() == 1 {
                    break;
                 }
                },
                Err(_) => {
                    println!("Use the required format")
                }
            }
            buffer.clear()
        }
    }
}

pub use crate::helpers::MMM;
