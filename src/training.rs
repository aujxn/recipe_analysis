use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn clean() {
    let mut file = File::open("nyt-ingredients-snapshot-2015.csv").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines: Vec<String> = contents
        .as_str()
        .split("\n")
        .skip(1)
        .map(|line| String::from(line))
        .collect();

    lines.pop();

    /*
    let mut i = 0;
    for line in lines {
        match line.find(',') {
            Some(_) => i += 1,
            None => println!("{}", line),
        }
    }
    */

    lines.retain(|line| {
        let start = line.find(',').unwrap();
        line[start + 1..].find(',').unwrap() != 0
    });

    // (phrase, name, qty, range, unit)
    let mut data = vec![];

    for line in lines {
        let start = line.find(',').unwrap();
        let mut end;
        let mut tail = &line[start + 1..];
        let phrase;
        let ingredient;
        let qty;
        let range;
        let unit;
        if let Some(index) = tail.find('"') {
            if index == 0 {
                tail = &tail[1..];
                end = tail.find('"').unwrap();
                phrase = &tail[..end];
                tail = &tail[end + 2..];
            } else {
                end = tail.find(',').unwrap();
                phrase = &tail[..end];
                tail = &tail[end + 1..];
            }
        } else {
            end = tail.find(',').unwrap();
            phrase = &tail[..end];
            tail = &tail[end + 1..];
        }
        let phrase = String::from(phrase);

        end = tail.find(',').unwrap();
        ingredient = &tail[..end];
        tail = &tail[end + 1..];
        let ingredient = String::from(ingredient);

        end = tail.find(',').unwrap();
        qty = &tail[..end];
        tail = &tail[end + 1..];
        let qty = String::from(qty);

        end = tail.find(',').unwrap();
        range = &tail[..end];
        tail = &tail[end + 1..];
        let range = String::from(range);

        end = tail.find(',').unwrap();
        unit = &tail[..end];
        let unit = String::from(unit);

        data.push((phrase, ingredient, qty, range, unit));
    }

    data.retain(|line| line.0 != "" && line.1 != "");

    let mut crf = String::new();
    for line in data.iter() {
        let mut iter = line.0.as_str().split(line.1.as_str());
        let pre = iter.next();
        if pre.is_none() {
            continue;
        }
        let pre = pre.unwrap();
        let post = iter.next();
        if post.is_none() {
            continue;
        }
        let post = post.unwrap();

        for word in pre.split_ascii_whitespace() {
            crf.push_str(word);
            crf.push_str("\te\n");
        }

        for word in line.1.as_str().split_ascii_whitespace() {
            crf.push_str(word);
            crf.push_str("\ti\n");
        }

        for word in post.split_ascii_whitespace() {
            crf.push_str(word);
            crf.push_str("\te\n");
        }
        crf.push('\n');
    }

    println!("{}", crf);
    /*
    for line in data {
        println!("{}", line.0);
        println!("{}, {} to {}, {}\n", line.1, line.2, line.3, line.4);
    }
    */
}

#[test]
fn format_phrase() {
    let input = String::from("1 ¼ things");
    let output = decimal(&input);

    assert_eq!(output, "1.25 things");
}

#[test]
fn format_phrase1() {
    let input = String::from(" ¼ things");
    let output = decimal(&input);

    assert_eq!(output, "0.25 things");
}

#[test]
fn format_phrase2() {
    let input = String::from("1 4/5 to 2     1/3 things");
    let output = decimal(&input);

    assert_eq!(output, "1.80 to 2.33 things");
}

#[test]
fn format_phrase3() {
    let input = String::from("1¼ things");
    let output = decimal(&input);

    assert_eq!(output, "1.25 things");
}

#[test]
fn format_phrase4() {
    let input = String::from("½ things");
    let output = decimal(&input);

    assert_eq!(output, "0.50 things");
}

/// Ingredient phrases have fractions, possibly using unicode characters.
/// It will be easier to work with a phrase for natural language processing
/// if these fractions are converted to a decimal.
pub fn decimal(phrase: &String) -> String {
    let unicode_frac = vec![
        "⅛", "⅜", "⅝", "⅞", "⅙", "⅚", "⅕", "⅖", "⅗", "⅘", "¼", "¾", "⅓", "⅔", "½",
    ];
    let ascii_frac = vec![
        " 1/8", " 3/8", " 5/8", " 7/8", " 1/6", " 5/6", " 1/5", " 2/5", " 3/5", " 4/5", " 1/4",
        " 3/4", " 1/3", " 2/3", " 1/2",
    ];

    let mut new: String = phrase.clone();

    // Replace all unicode fractions with ascii
    for (unicode, ascii) in unicode_frac.iter().zip(ascii_frac) {
        if new.contains(unicode) {
            new = new.replace(unicode, ascii);
        }
    }

    // Regex to match on mixed fractions
    let mixed = Regex::new(r"(?P<mixed>(?P<whole>\d+)\s+(?P<numer>\d)/(?P<denom>\d))").unwrap();

    // Replace all the mixed with decimal counterparts
    for cap in mixed.captures_iter(&new.clone()) {
        let val: f64 = &cap["whole"].parse::<f64>().unwrap()
            + (&cap["numer"].parse::<f64>().unwrap() / &cap["denom"].parse::<f64>().unwrap());

        let val = format!("{:.2}", val);
        new = new.replace(&cap["mixed"], &val);
    }

    // Regex to match on vulgar fractions with no whole part
    let frac = Regex::new(r"(?P<frac>(?P<numer>\d)/(?P<denom>\d))").unwrap();

    // Replace the vulgar fractions
    for cap in frac.captures_iter(&new.clone()) {
        let val: f64 =
            &cap["numer"].parse::<f64>().unwrap() / &cap["denom"].parse::<f64>().unwrap();
        let val = format!("{:.2}", val);
        new = new.replace(&cap["frac"], &val);
    }

    String::from(new.trim())
}
