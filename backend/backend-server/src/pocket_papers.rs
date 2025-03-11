use std::{collections::HashMap, fs, num::NonZero, process::Command};

use regex::Regex;
use serde_with::NoneAsEmptyString;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{fs_helpers::get_uuid_dir, trait_helpers::AppError};

pub static HTK_STL_MAIN: &'static [u8] = include_bytes!("../typst_files/HTK_STL_Template/main.typ");
pub static HTK_STL_CONF: &'static [u8] = include_bytes!("../typst_files/HTK_STL_Template/conf.typ");

pub static HTK_SPI_MAIN: &'static [u8] = include_bytes!("../typst_files/HTK_SPI_Template/main.typ");
pub static HTK_SPI_CONF: &'static [u8] = include_bytes!("../typst_files/HTK_SPI_Template/conf.typ");

pub static HTK_VLT_MAIN: &'static [u8] = include_bytes!("../typst_files/HTK_VLT_Template/main.typ");
pub static HTK_VLT_CONF: &'static [u8] = include_bytes!("../typst_files/HTK_VLT_Template/conf.typ");

pub static HTK_DTB_LOGO: &'static [u8] = include_bytes!("../typst_files/shared/dtb.svg");
pub static HTK_ABT_LOGO: &'static [u8] = include_bytes!("../typst_files/shared/abteilung.svg");

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct VLTRequest {
    name: String,
    club: String,
    #[serde(rename = "agegroup")]
    age_group: String,
    discipline: String,
    #[serde(rename = "1_abbr")]
    #[serde_as(as = "NoneAsEmptyString")]
    abbr_1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "1_desc")]
    desc_1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "1_diff")]
    diff_1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "2_abbr")]
    abbr_2: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "2_desc")]
    desc_2: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "2_diff")]
    diff_2: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SPIRequest {
    name: String,
    club: String,
    #[serde(rename = "agegroup")]
    age_group: String,
    discipline: String,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "1_abbr")]
    abbr_1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "1_desc")]
    desc_1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "1_sgs")]
    sgs_1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "1_diff")]
    diff_1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "2_abbr")]
    abbr_2: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "2_desc")]
    desc_2: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "2_sgs")]
    sgs_2: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "2_diff")]
    diff_2: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "3_abbr")]
    abbr_3: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "3_desc")]
    desc_3: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "3_sgs")]
    sgs_3: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "3_diff")]
    diff_3: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "4_abbr")]
    abbr_4: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "4_desc")]
    desc_4: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "4_sgs")]
    sgs_4: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "4_diff")]
    diff_4: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "5_abbr")]
    abbr_5: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "5_desc")]
    desc_5: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "5_sgs")]
    sgs_5: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "5_diff")]
    diff_5: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "6_abbr")]
    abbr_6: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "6_desc")]
    desc_6: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "6_sgs")]
    sgs_6: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "6_diff")]
    diff_6: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "7_abbr")]
    abbr_7: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "7_desc")]
    desc_7: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "7_sgs")]
    sgs_7: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "7_diff")]
    diff_7: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "8_abbr")]
    abbr_8: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "8_desc")]
    desc_8: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "8_sgs")]
    sgs_8: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "8_diff")]
    diff_8: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "9_abbr")]
    abbr_9: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "9_desc")]
    desc_9: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "9_sgs")]
    sgs_9: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "9_diff")]
    diff_9: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "10_abbr")]
    abbr_10: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "10_desc")]
    desc_10: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "10_sgs")]
    sgs_10: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "10_diff")]
    diff_10: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "11_abbr")]
    abbr_11: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "11_desc")]
    desc_11: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "11_sgs")]
    sgs_11: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "11_diff")]
    diff_11: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "12_abbr")]
    abbr_12: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "12_desc")]
    desc_12: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "12_sgs")]
    sgs_12: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "12_diff")]
    diff_12: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "Abg_abbr")]
    abg_abbr: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "Abg_desc")]
    abg_desc: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "Abg_sgs")]
    abg_sgs: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "Abg_diff")]
    abg_diff: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct STLRequest {
    name: String,
    club: String,
    #[serde(rename = "agegroup")]
    age_group: String,
    discipline: String,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "1_abbr")]
    abbr_1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "1_desc")]
    desc_1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "1_sgs")]
    sgs_1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "1_diff")]
    diff_1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "2_abbr")]
    abbr_2: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "2_desc")]
    desc_2: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "2_sgs")]
    sgs_2: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "2_diff")]
    diff_2: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "3_abbr")]
    abbr_3: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "3_desc")]
    desc_3: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "3_sgs")]
    sgs_3: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "3_diff")]
    diff_3: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "4_abbr")]
    abbr_4: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "4_desc")]
    desc_4: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "4_sgs")]
    sgs_4: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "4_diff")]
    diff_4: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "5_abbr")]
    abbr_5: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "5_desc")]
    desc_5: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "5_sgs")]
    sgs_5: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "5_diff")]
    diff_5: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "6_abbr")]
    abbr_6: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "6_desc")]
    desc_6: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "6_sgs")]
    sgs_6: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "6_diff")]
    diff_6: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "7_abbr")]
    abbr_7: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "7_desc")]
    desc_7: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "7_sgs")]
    sgs_7: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "7_diff")]
    diff_7: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "8_abbr")]
    abbr_8: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "8_desc")]
    desc_8: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "8_sgs")]
    sgs_8: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "8_diff")]
    diff_8: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "9_abbr")]
    abbr_9: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "9_desc")]
    desc_9: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "9_sgs")]
    sgs_9: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "9_diff")]
    diff_9: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "10_abbr")]
    abbr_10: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "10_desc")]
    desc_10: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "10_sgs")]
    sgs_10: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "10_diff")]
    diff_10: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "11_abbr")]
    abbr_11: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "11_desc")]
    desc_11: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "11_sgs")]
    sgs_11: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "11_diff")]
    diff_11: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "12_abbr")]
    abbr_12: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "12_desc")]
    desc_12: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "12_sgs")]
    sgs_12: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "12_diff")]
    diff_12: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "Abg_abbr")]
    abg_abbr: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "Abg_desc")]
    abg_desc: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "Abg_sgs")]
    abg_sgs: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "Abg_diff")]
    abg_diff: Option<String>,
}

pub enum PocketPaperKind {
    STL(STLRequest),
    VLT(VLTRequest),
    SPI(SPIRequest)
}

fn roman_to_arabic(roman: &str) -> Option<u8> {
    let roman_map: HashMap<&str, u8> = [
        ("I", 1), ("II", 2), ("III", 3), ("IV", 4), ("V", 5),
        ("VI", 6), ("VII", 7), ("VIII", 8), ("IX", 9), ("X", 10),
        ("XI", 11), ("XII", 12), ("XIII", 13), ("XIV", 14), ("XV", 15),
        ("XVI", 16), ("XVII", 17),

        ("i", 1), ("ii", 2), ("iii", 3), ("iv", 4), ("v", 5),
        ("vi", 6), ("vii", 7), ("viii", 8), ("ix", 9), ("x", 10),
        ("xi", 11), ("xii", 12), ("xiii", 13), ("xiv", 14), ("xv", 15),
        ("xvi", 16), ("xvii", 17)
    ].iter().cloned().collect();

    roman_map.get(roman).copied()
}

fn extract_structure_groups(input: &str) -> Result<Vec<u8>, anyhow::Error> {
    let regex = Regex::new(r"(XVII|XVI|XV|XIV|XIII|XII|XI|X|IX|VIII|VII|VI|V|IV|III|II|I|xvii|xvi|xv|xiv|xiii|xii|xi|x|ix|viii|vii|vi|v|iv|iii|ii|i|1[0-7]|\d+)").unwrap();
    let matches: Vec<&str> = regex.find_iter(input).map(|mtx| { mtx.as_str() }).collect();
    let mut return_vec = vec![];
    for mtx in matches {
        if let Some(parsed_roman) = roman_to_arabic(mtx) {
            return_vec.push(parsed_roman);
        } else {
            if let Ok(parsed) = mtx.parse::<u8>() {
                return_vec.push(parsed);
            }
        }
    }
    Ok(return_vec)
}

fn extract_difficulty_value(input: &str) -> Result<Vec<f32>, anyhow::Error> {
    let regex1 = Regex::new(r"\b(Pkt\.?|P\.?|P|pkt\.?|p\.?|pkt|p)\b").unwrap();
    let decor_removed = regex1.replace_all(input, "").trim().to_string();
    let regex2 = Regex::new(r"-?\d+(?:[.,]\d)?").unwrap();
    let matches: Vec<String> = regex2.find_iter(&decor_removed).map(|mtx| { mtx.as_str().replace(",", ".") }).collect();
    let mut return_vec = vec![];
    for mtx in matches {
        if let Ok(parsed) = mtx.parse::<f32>() {
            return_vec.push(parsed);
        }
    }
    Ok(return_vec)
}

fn write_stl_request_to_conf(request: &STLRequest, template_clone: String) -> Result<String, AppError> {

    // Extract difficulty values
    let diff_values = vec![
        extract_difficulty_value(&request.diff_1.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_2.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_3.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_4.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_5.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_6.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_7.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_8.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_9.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_10.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_11.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_12.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.abg_diff.as_ref().unwrap_or(&String::new()))?,
    ];

    // Extract difficulty values
    let sgs_values = vec![
        extract_structure_groups(&request.sgs_1.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_2.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_3.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_4.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_5.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_6.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_7.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_8.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_9.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_10.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_11.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_12.as_ref().unwrap_or(&String::new()))?,
    ];

    // Get highest 8 difficulty values
    let mut diff_values_flat: Vec<f32> = diff_values.into_iter().flatten().collect::<Vec<f32>>();
    diff_values_flat.sort_unstable_by(|a, b| a.total_cmp(b));
    let highest8: Vec<f32> = diff_values_flat.iter().take(8).copied().collect();
    let mut diff_value_map: HashMap<&str, u8> = [
        ("0", 0), ("A", 0), ("B", 0),
        ("C", 0), ("D", 0), ("E", 0)
    ].iter().cloned().collect();
    for value in &highest8 {
        match value {
            0.0 => diff_value_map.insert("0", diff_value_map.get("0").unwrap_or(&0) + 1),
            0.2 => diff_value_map.insert("A", diff_value_map.get("A").unwrap_or(&0) + 1),
            0.4 => diff_value_map.insert("B", diff_value_map.get("B").unwrap_or(&0) + 1),
            0.6 => diff_value_map.insert("C", diff_value_map.get("C").unwrap_or(&0) + 1),
            0.8 => diff_value_map.insert("D", diff_value_map.get("D").unwrap_or(&0) + 1),
            1.0 => diff_value_map.insert("E", diff_value_map.get("E").unwrap_or(&0) + 1),
            _ => continue
        };
    }

    // Get difficulty sum
    let mut sum_difficulty: f32 = highest8.iter().sum();

    // Get fulfilled structure groups
    let structure_groups_flat: Vec<u8> = sgs_values.into_iter().flatten().collect::<Vec<u8>>();
    let sgs_map: HashMap<u8, bool> = [
        (1, structure_groups_flat.contains(&1)),
        (2, structure_groups_flat.contains(&2)),
        (3, structure_groups_flat.contains(&3)),
        (4, structure_groups_flat.contains(&4)),
        (5, structure_groups_flat.contains(&5)),
        (6, structure_groups_flat.contains(&6)),
        (7, structure_groups_flat.contains(&7)),
        (8, structure_groups_flat.contains(&8)),
        (9, structure_groups_flat.contains(&9)),
        (10, structure_groups_flat.contains(&10)),
        (11, structure_groups_flat.contains(&11)),
        (12, structure_groups_flat.contains(&12)),
        (13, structure_groups_flat.contains(&13)),
        (14, structure_groups_flat.contains(&14)),
        (15, structure_groups_flat.contains(&15)),
        (16, structure_groups_flat.contains(&16)),
        (17, structure_groups_flat.contains(&17)),
    ].iter().cloned().collect();

    // Add structure group bonus
    sum_difficulty += (sgs_map.values().filter(|val| **val).count().min(10) as f32) * 0.2;

    // Define replacements based on the struct fields
    let replacements = [
        ("#let name = \"\"", format!("#let name = \"{}\"", request.name)),
        ("#let club = \"\"", format!("#let club = \"{}\"", request.club)),
        ("#let age_group = \"\"", format!("#let age_group = \"{}\"", request.age_group)),

        ("#let abbr_1 = \"\"", format!("#let abbr_1 = \"{}\"", request.abbr_1.clone().unwrap_or_default())),
        ("#let desc_1 = \"\"", format!("#let desc_1 = \"{}\"", request.desc_1.clone().unwrap_or_default())),
        ("#let sgs_1 = \"\"", format!("#let sgs_1 = \"{}\"", request.sgs_1.clone().unwrap_or_default())),
        ("#let diff_1 = \"\"", format!("#let diff_1 = \"{}\"", request.diff_1.clone().unwrap_or_default())),

        ("#let abbr_2 = \"\"", format!("#let abbr_2 = \"{}\"", request.abbr_2.clone().unwrap_or_default())),
        ("#let desc_2 = \"\"", format!("#let desc_2 = \"{}\"", request.desc_2.clone().unwrap_or_default())),
        ("#let sgs_2 = \"\"", format!("#let sgs_2 = \"{}\"", request.sgs_2.clone().unwrap_or_default())),
        ("#let diff_2 = \"\"", format!("#let diff_2 = \"{}\"", request.diff_2.clone().unwrap_or_default())),

        ("#let abbr_3 = \"\"", format!("#let abbr_3 = \"{}\"", request.abbr_3.clone().unwrap_or_default())),
        ("#let desc_3 = \"\"", format!("#let desc_3 = \"{}\"", request.desc_3.clone().unwrap_or_default())),
        ("#let sgs_3 = \"\"", format!("#let sgs_3 = \"{}\"", request.sgs_3.clone().unwrap_or_default())),
        ("#let diff_3 = \"\"", format!("#let diff_3 = \"{}\"", request.diff_3.clone().unwrap_or_default())),

        ("#let abbr_4 = \"\"", format!("#let abbr_4 = \"{}\"", request.abbr_4.clone().unwrap_or_default())),
        ("#let desc_4 = \"\"", format!("#let desc_4 = \"{}\"", request.desc_4.clone().unwrap_or_default())),
        ("#let sgs_4 = \"\"", format!("#let sgs_4 = \"{}\"", request.sgs_4.clone().unwrap_or_default())),
        ("#let diff_4 = \"\"", format!("#let diff_4 = \"{}\"", request.diff_4.clone().unwrap_or_default())),

        ("#let abbr_5 = \"\"", format!("#let abbr_5 = \"{}\"", request.abbr_5.clone().unwrap_or_default())),
        ("#let desc_5 = \"\"", format!("#let desc_5 = \"{}\"", request.desc_5.clone().unwrap_or_default())),
        ("#let sgs_5 = \"\"", format!("#let sgs_5 = \"{}\"", request.sgs_5.clone().unwrap_or_default())),
        ("#let diff_5 = \"\"", format!("#let diff_5 = \"{}\"", request.diff_5.clone().unwrap_or_default())),

        ("#let abbr_6 = \"\"", format!("#let abbr_6 = \"{}\"", request.abbr_6.clone().unwrap_or_default())),
        ("#let desc_6 = \"\"", format!("#let desc_6 = \"{}\"", request.desc_6.clone().unwrap_or_default())),
        ("#let sgs_6 = \"\"", format!("#let sgs_6 = \"{}\"", request.sgs_6.clone().unwrap_or_default())),
        ("#let diff_6 = \"\"", format!("#let diff_6 = \"{}\"", request.diff_6.clone().unwrap_or_default())),

        ("#let abbr_7 = \"\"", format!("#let abbr_7 = \"{}\"", request.abbr_7.clone().unwrap_or_default())),
        ("#let desc_7 = \"\"", format!("#let desc_7 = \"{}\"", request.desc_7.clone().unwrap_or_default())),
        ("#let sgs_7 = \"\"", format!("#let sgs_7 = \"{}\"", request.sgs_7.clone().unwrap_or_default())),
        ("#let diff_7 = \"\"", format!("#let diff_7 = \"{}\"", request.diff_7.clone().unwrap_or_default())),

        ("#let abbr_8 = \"\"", format!("#let abbr_8 = \"{}\"", request.abbr_8.clone().unwrap_or_default())),
        ("#let desc_8 = \"\"", format!("#let desc_8 = \"{}\"", request.desc_8.clone().unwrap_or_default())),
        ("#let sgs_8 = \"\"", format!("#let sgs_8 = \"{}\"", request.sgs_8.clone().unwrap_or_default())),
        ("#let diff_8 = \"\"", format!("#let diff_8 = \"{}\"", request.diff_8.clone().unwrap_or_default())),

        ("#let abbr_9 = \"\"", format!("#let abbr_9 = \"{}\"", request.abbr_9.clone().unwrap_or_default())),
        ("#let desc_9 = \"\"", format!("#let desc_9 = \"{}\"", request.desc_9.clone().unwrap_or_default())),
        ("#let sgs_9 = \"\"", format!("#let sgs_9 = \"{}\"", request.sgs_9.clone().unwrap_or_default())),
        ("#let diff_9 = \"\"", format!("#let diff_9 = \"{}\"", request.diff_9.clone().unwrap_or_default())),

        ("#let abbr_10 = \"\"", format!("#let abbr_10 = \"{}\"", request.abbr_10.clone().unwrap_or_default())),
        ("#let desc_10 = \"\"", format!("#let desc_10 = \"{}\"", request.desc_10.clone().unwrap_or_default())),
        ("#let sgs_10 = \"\"", format!("#let sgs_10 = \"{}\"", request.sgs_10.clone().unwrap_or_default())),
        ("#let diff_10 = \"\"", format!("#let diff_10 = \"{}\"", request.diff_10.clone().unwrap_or_default())),

        ("#let abbr_11 = \"\"", format!("#let abbr_11 = \"{}\"", request.abbr_11.clone().unwrap_or_default())),
        ("#let desc_11 = \"\"", format!("#let desc_11 = \"{}\"", request.desc_11.clone().unwrap_or_default())),
        ("#let sgs_11 = \"\"", format!("#let sgs_11 = \"{}\"", request.sgs_11.clone().unwrap_or_default())),
        ("#let diff_11 = \"\"", format!("#let diff_11 = \"{}\"", request.diff_11.clone().unwrap_or_default())),

        ("#let abbr_12 = \"\"", format!("#let abbr_12 = \"{}\"", request.abbr_12.clone().unwrap_or_default())),
        ("#let desc_12 = \"\"", format!("#let desc_12 = \"{}\"", request.desc_12.clone().unwrap_or_default())),
        ("#let sgs_12 = \"\"", format!("#let sgs_12 = \"{}\"", request.sgs_12.clone().unwrap_or_default())),
        ("#let diff_12 = \"\"", format!("#let diff_12 = \"{}\"", request.diff_12.clone().unwrap_or_default())),

        ("#let abbr_abg = \"\"", format!("#let abbr_abg = \"{}\"", request.abg_abbr.clone().unwrap_or_default())),
        ("#let desc_abg = \"\"", format!("#let desc_abg = \"{}\"", request.abg_desc.clone().unwrap_or_default())),
        ("#let sgs_abg = \"\"", format!("#let sgs_abg = \"{}\"", request.abg_sgs.clone().unwrap_or_default())),
        ("#let diff_abg = \"\"", format!("#let diff_abg = \"{}\"", request.abg_diff.clone().unwrap_or_default())),

        ("#let zero_elem = \"\"", format!("#let zero_elem = \"{} x 0\"", diff_value_map.get("0").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),
        ("#let A_elem = \"\"", format!("#let A_elem = \"{} x A\"", diff_value_map.get("A").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),
        ("#let B_elem = \"\"", format!("#let B_elem = \"{} x B\"", diff_value_map.get("B").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),
        ("#let C_elem = \"\"", format!("#let C_elem = \"{} x C\"", diff_value_map.get("C").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),
        ("#let D_elem = \"\"", format!("#let D_elem = \"{} x D\"", diff_value_map.get("D").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),
        ("#let E_elem = \"\"", format!("#let E_elem = \"{} x E\"", diff_value_map.get("E").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),

        ("#let sg_1 = \"\"", format!("#let sg_1 = \"{}\"", format!("{} I", if *sgs_map.get(&1).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_2 = \"\"", format!("#let sg_2 = \"{}\"", format!("{} II", if *sgs_map.get(&2).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_3 = \"\"", format!("#let sg_3 = \"{}\"", format!("{} III", if *sgs_map.get(&3).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_4 = \"\"", format!("#let sg_4 = \"{}\"", format!("{} IV", if *sgs_map.get(&4).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_5 = \"\"", format!("#let sg_5 = \"{}\"", format!("{} V", if *sgs_map.get(&5).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_6 = \"\"", format!("#let sg_6 = \"{}\"", format!("{} VI", if *sgs_map.get(&6).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_7 = \"\"", format!("#let sg_7 = \"{}\"", format!("{} VII", if *sgs_map.get(&7).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_8 = \"\"", format!("#let sg_8 = \"{}\"", format!("{} VIII", if *sgs_map.get(&8).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_9 = \"\"", format!("#let sg_9 = \"{}\"", format!("{} IX", if *sgs_map.get(&9).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_10 = \"\"", format!("#let sg_10 = \"{}\"", format!("{} X", if *sgs_map.get(&10).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_11 = \"\"", format!("#let sg_11 = \"{}\"", format!("{} XI", if *sgs_map.get(&11).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_12 = \"\"", format!("#let sg_12 = \"{}\"", format!("{} XII", if *sgs_map.get(&12).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_13 = \"\"", format!("#let sg_13 = \"{}\"", format!("{} XIII", if *sgs_map.get(&13).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_14 = \"\"", format!("#let sg_14 = \"{}\"", format!("{} XIV", if *sgs_map.get(&14).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_15 = \"\"", format!("#let sg_15 = \"{}\"", format!("{} XV", if *sgs_map.get(&15).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_16 = \"\"", format!("#let sg_16 = \"{}\"", format!("{} XVI", if *sgs_map.get(&16).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_17 = \"\"", format!("#let sg_17 = \"{}\"", format!("{} XVII", if *sgs_map.get(&17).unwrap() { "☑" } else { "☐" }))),

        ("#let sum_diff = \"\"", format!("#let sum_diff = \"{}\"", format!("{:.1} Pkt.", if sum_difficulty == 0.0 { 0.0 } else { sum_difficulty }))),
    ];

    // Replace placeholders
    let mut output_content = template_clone;
    for (placeholder, replacement) in &replacements {
        output_content = output_content.replace(placeholder, replacement);
    }

    Ok(output_content)
}

fn write_spi_request_to_conf(request: &SPIRequest, template_clone: String) -> Result<String, AppError> {

    // Extract difficulty values
    let diff_values = vec![
        extract_difficulty_value(&request.diff_1.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_2.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_3.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_4.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_5.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_6.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_7.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_8.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_9.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_10.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_11.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.diff_12.as_ref().unwrap_or(&String::new()))?,
        extract_difficulty_value(&request.abg_diff.as_ref().unwrap_or(&String::new()))?,
    ];

    // Extract difficulty values
    let sgs_values = vec![
        extract_structure_groups(&request.sgs_1.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_2.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_3.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_4.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_5.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_6.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_7.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_8.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_9.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_10.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_11.as_ref().unwrap_or(&String::new()))?,
        extract_structure_groups(&request.sgs_12.as_ref().unwrap_or(&String::new()))?,
    ];

    // Get highest 8 difficulty values
    let mut diff_values_flat: Vec<f32> = diff_values.into_iter().flatten().collect::<Vec<f32>>();
    diff_values_flat.sort_unstable_by(|a, b| a.total_cmp(b));
    let highest8: Vec<f32> = diff_values_flat.iter().take(8).copied().collect();
    let mut diff_value_map: HashMap<&str, u8> = [
        ("0", 0), ("A", 0), ("B", 0),
        ("C", 0), ("D", 0), ("E", 0)
    ].iter().cloned().collect();
    for value in &highest8 {
        match value {
            0.0 => diff_value_map.insert("0", diff_value_map.get("0").unwrap_or(&0) + 1),
            0.2 => diff_value_map.insert("A", diff_value_map.get("A").unwrap_or(&0) + 1),
            0.4 => diff_value_map.insert("B", diff_value_map.get("B").unwrap_or(&0) + 1),
            0.6 => diff_value_map.insert("C", diff_value_map.get("C").unwrap_or(&0) + 1),
            0.8 => diff_value_map.insert("D", diff_value_map.get("D").unwrap_or(&0) + 1),
            1.0 => diff_value_map.insert("E", diff_value_map.get("E").unwrap_or(&0) + 1),
            _ => continue
        };
    }

    // Get difficulty sum
    let mut sum_difficulty: f32 = highest8.iter().sum();

    // Get fulfilled structure groups
    let structure_groups_flat: Vec<u8> = sgs_values.into_iter().flatten().collect::<Vec<u8>>();
    let sgs_map: HashMap<u8, bool> = [
        (1, structure_groups_flat.contains(&1)),
        (2, structure_groups_flat.contains(&2)),
        (3, structure_groups_flat.contains(&3)),
        (4, structure_groups_flat.contains(&4)),
        (5, structure_groups_flat.contains(&5)),
        (6, structure_groups_flat.contains(&6)),
        (7, structure_groups_flat.contains(&7)),
        (8, structure_groups_flat.contains(&8)),
        (9, structure_groups_flat.contains(&9)),
        (10, structure_groups_flat.contains(&10)),
        (11, structure_groups_flat.contains(&11)),
        (12, structure_groups_flat.contains(&12)),
        (13, structure_groups_flat.contains(&13)),
        (14, structure_groups_flat.contains(&14)),
        (15, structure_groups_flat.contains(&15)),
        (16, structure_groups_flat.contains(&16)),
    ].iter().cloned().collect();

    // Add structure group bonus
    sum_difficulty += (sgs_map.values().filter(|val| **val).count().min(10) as f32) * 0.2;

    // Define replacements based on the struct fields
    let replacements = [
        ("#let name = \"\"", format!("#let name = \"{}\"", request.name)),
        ("#let club = \"\"", format!("#let club = \"{}\"", request.club)),
        ("#let age_group = \"\"", format!("#let age_group = \"{}\"", request.age_group)),

        ("#let abbr_1 = \"\"", format!("#let abbr_1 = \"{}\"", request.abbr_1.clone().unwrap_or_default())),
        ("#let desc_1 = \"\"", format!("#let desc_1 = \"{}\"", request.desc_1.clone().unwrap_or_default())),
        ("#let sgs_1 = \"\"", format!("#let sgs_1 = \"{}\"", request.sgs_1.clone().unwrap_or_default())),
        ("#let diff_1 = \"\"", format!("#let diff_1 = \"{}\"", request.diff_1.clone().unwrap_or_default())),

        ("#let abbr_2 = \"\"", format!("#let abbr_2 = \"{}\"", request.abbr_2.clone().unwrap_or_default())),
        ("#let desc_2 = \"\"", format!("#let desc_2 = \"{}\"", request.desc_2.clone().unwrap_or_default())),
        ("#let sgs_2 = \"\"", format!("#let sgs_2 = \"{}\"", request.sgs_2.clone().unwrap_or_default())),
        ("#let diff_2 = \"\"", format!("#let diff_2 = \"{}\"", request.diff_2.clone().unwrap_or_default())),

        ("#let abbr_3 = \"\"", format!("#let abbr_3 = \"{}\"", request.abbr_3.clone().unwrap_or_default())),
        ("#let desc_3 = \"\"", format!("#let desc_3 = \"{}\"", request.desc_3.clone().unwrap_or_default())),
        ("#let sgs_3 = \"\"", format!("#let sgs_3 = \"{}\"", request.sgs_3.clone().unwrap_or_default())),
        ("#let diff_3 = \"\"", format!("#let diff_3 = \"{}\"", request.diff_3.clone().unwrap_or_default())),

        ("#let abbr_4 = \"\"", format!("#let abbr_4 = \"{}\"", request.abbr_4.clone().unwrap_or_default())),
        ("#let desc_4 = \"\"", format!("#let desc_4 = \"{}\"", request.desc_4.clone().unwrap_or_default())),
        ("#let sgs_4 = \"\"", format!("#let sgs_4 = \"{}\"", request.sgs_4.clone().unwrap_or_default())),
        ("#let diff_4 = \"\"", format!("#let diff_4 = \"{}\"", request.diff_4.clone().unwrap_or_default())),

        ("#let abbr_5 = \"\"", format!("#let abbr_5 = \"{}\"", request.abbr_5.clone().unwrap_or_default())),
        ("#let desc_5 = \"\"", format!("#let desc_5 = \"{}\"", request.desc_5.clone().unwrap_or_default())),
        ("#let sgs_5 = \"\"", format!("#let sgs_5 = \"{}\"", request.sgs_5.clone().unwrap_or_default())),
        ("#let diff_5 = \"\"", format!("#let diff_5 = \"{}\"", request.diff_5.clone().unwrap_or_default())),

        ("#let abbr_6 = \"\"", format!("#let abbr_6 = \"{}\"", request.abbr_6.clone().unwrap_or_default())),
        ("#let desc_6 = \"\"", format!("#let desc_6 = \"{}\"", request.desc_6.clone().unwrap_or_default())),
        ("#let sgs_6 = \"\"", format!("#let sgs_6 = \"{}\"", request.sgs_6.clone().unwrap_or_default())),
        ("#let diff_6 = \"\"", format!("#let diff_6 = \"{}\"", request.diff_6.clone().unwrap_or_default())),

        ("#let abbr_7 = \"\"", format!("#let abbr_7 = \"{}\"", request.abbr_7.clone().unwrap_or_default())),
        ("#let desc_7 = \"\"", format!("#let desc_7 = \"{}\"", request.desc_7.clone().unwrap_or_default())),
        ("#let sgs_7 = \"\"", format!("#let sgs_7 = \"{}\"", request.sgs_7.clone().unwrap_or_default())),
        ("#let diff_7 = \"\"", format!("#let diff_7 = \"{}\"", request.diff_7.clone().unwrap_or_default())),

        ("#let abbr_8 = \"\"", format!("#let abbr_8 = \"{}\"", request.abbr_8.clone().unwrap_or_default())),
        ("#let desc_8 = \"\"", format!("#let desc_8 = \"{}\"", request.desc_8.clone().unwrap_or_default())),
        ("#let sgs_8 = \"\"", format!("#let sgs_8 = \"{}\"", request.sgs_8.clone().unwrap_or_default())),
        ("#let diff_8 = \"\"", format!("#let diff_8 = \"{}\"", request.diff_8.clone().unwrap_or_default())),

        ("#let abbr_9 = \"\"", format!("#let abbr_9 = \"{}\"", request.abbr_9.clone().unwrap_or_default())),
        ("#let desc_9 = \"\"", format!("#let desc_9 = \"{}\"", request.desc_9.clone().unwrap_or_default())),
        ("#let sgs_9 = \"\"", format!("#let sgs_9 = \"{}\"", request.sgs_9.clone().unwrap_or_default())),
        ("#let diff_9 = \"\"", format!("#let diff_9 = \"{}\"", request.diff_9.clone().unwrap_or_default())),

        ("#let abbr_10 = \"\"", format!("#let abbr_10 = \"{}\"", request.abbr_10.clone().unwrap_or_default())),
        ("#let desc_10 = \"\"", format!("#let desc_10 = \"{}\"", request.desc_10.clone().unwrap_or_default())),
        ("#let sgs_10 = \"\"", format!("#let sgs_10 = \"{}\"", request.sgs_10.clone().unwrap_or_default())),
        ("#let diff_10 = \"\"", format!("#let diff_10 = \"{}\"", request.diff_10.clone().unwrap_or_default())),

        ("#let abbr_11 = \"\"", format!("#let abbr_11 = \"{}\"", request.abbr_11.clone().unwrap_or_default())),
        ("#let desc_11 = \"\"", format!("#let desc_11 = \"{}\"", request.desc_11.clone().unwrap_or_default())),
        ("#let sgs_11 = \"\"", format!("#let sgs_11 = \"{}\"", request.sgs_11.clone().unwrap_or_default())),
        ("#let diff_11 = \"\"", format!("#let diff_11 = \"{}\"", request.diff_11.clone().unwrap_or_default())),

        ("#let abbr_12 = \"\"", format!("#let abbr_12 = \"{}\"", request.abbr_12.clone().unwrap_or_default())),
        ("#let desc_12 = \"\"", format!("#let desc_12 = \"{}\"", request.desc_12.clone().unwrap_or_default())),
        ("#let sgs_12 = \"\"", format!("#let sgs_12 = \"{}\"", request.sgs_12.clone().unwrap_or_default())),
        ("#let diff_12 = \"\"", format!("#let diff_12 = \"{}\"", request.diff_12.clone().unwrap_or_default())),

        ("#let abbr_abg = \"\"", format!("#let abbr_abg = \"{}\"", request.abg_abbr.clone().unwrap_or_default())),
        ("#let desc_abg = \"\"", format!("#let desc_abg = \"{}\"", request.abg_desc.clone().unwrap_or_default())),
        ("#let sgs_abg = \"\"", format!("#let sgs_abg = \"{}\"", request.abg_sgs.clone().unwrap_or_default())),
        ("#let diff_abg = \"\"", format!("#let diff_abg = \"{}\"", request.abg_diff.clone().unwrap_or_default())),

        ("#let zero_elem = \"\"", format!("#let zero_elem = \"{} x 0\"", diff_value_map.get("0").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),
        ("#let A_elem = \"\"", format!("#let A_elem = \"{} x A\"", diff_value_map.get("A").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),
        ("#let B_elem = \"\"", format!("#let B_elem = \"{} x B\"", diff_value_map.get("B").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),
        ("#let C_elem = \"\"", format!("#let C_elem = \"{} x C\"", diff_value_map.get("C").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),
        ("#let D_elem = \"\"", format!("#let D_elem = \"{} x D\"", diff_value_map.get("D").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),
        ("#let E_elem = \"\"", format!("#let E_elem = \"{} x E\"", diff_value_map.get("E").map_or_else(|| { "?".into() }, |val| { val.to_string() }))),

        ("#let sg_1 = \"\"", format!("#let sg_1 = \"{}\"", format!("{} I", if *sgs_map.get(&1).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_2 = \"\"", format!("#let sg_2 = \"{}\"", format!("{} II", if *sgs_map.get(&2).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_3 = \"\"", format!("#let sg_3 = \"{}\"", format!("{} III", if *sgs_map.get(&3).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_4 = \"\"", format!("#let sg_4 = \"{}\"", format!("{} IV", if *sgs_map.get(&4).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_5 = \"\"", format!("#let sg_5 = \"{}\"", format!("{} V", if *sgs_map.get(&5).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_6 = \"\"", format!("#let sg_6 = \"{}\"", format!("{} VI", if *sgs_map.get(&6).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_7 = \"\"", format!("#let sg_7 = \"{}\"", format!("{} VII", if *sgs_map.get(&7).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_8 = \"\"", format!("#let sg_8 = \"{}\"", format!("{} VIII", if *sgs_map.get(&8).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_9 = \"\"", format!("#let sg_9 = \"{}\"", format!("{} IX", if *sgs_map.get(&9).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_10 = \"\"", format!("#let sg_10 = \"{}\"", format!("{} X", if *sgs_map.get(&10).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_11 = \"\"", format!("#let sg_11 = \"{}\"", format!("{} XI", if *sgs_map.get(&11).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_12 = \"\"", format!("#let sg_12 = \"{}\"", format!("{} XII", if *sgs_map.get(&12).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_13 = \"\"", format!("#let sg_13 = \"{}\"", format!("{} XIII", if *sgs_map.get(&13).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_14 = \"\"", format!("#let sg_14 = \"{}\"", format!("{} XIV", if *sgs_map.get(&14).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_15 = \"\"", format!("#let sg_15 = \"{}\"", format!("{} XV", if *sgs_map.get(&15).unwrap() { "☑" } else { "☐" }))),
        ("#let sg_16 = \"\"", format!("#let sg_16 = \"{}\"", format!("{} XVI", if *sgs_map.get(&16).unwrap() { "☑" } else { "☐" }))),

        ("#let sum_diff = \"\"", format!("#let sum_diff = \"{}\"", format!("{:.1} Pkt.", if sum_difficulty == 0.0 { 0.0 } else { sum_difficulty }))),
    ];

    // Replace placeholders
    let mut output_content = template_clone;
    for (placeholder, replacement) in &replacements {
        output_content = output_content.replace(placeholder, replacement);
    }

    Ok(output_content)
}

fn write_vlt_request_to_conf(request: &VLTRequest, template_clone: String) -> Result<String, AppError> {

    // Define replacements based on the struct fields
    let replacements = [
        ("#let name = \"\"", format!("#let name = \"{}\"", request.name)),
        ("#let club = \"\"", format!("#let club = \"{}\"", request.club)),
        ("#let age_group = \"\"", format!("#let age_group = \"{}\"", request.age_group)),

        ("#let abbr_1 = \"\"", format!("#let abbr_1 = \"{}\"", request.abbr_1.clone().unwrap_or_default())),
        ("#let desc_1 = \"\"", format!("#let desc_1 = \"{}\"", request.desc_1.clone().unwrap_or_default())),
        ("#let diff_1 = \"\"", format!("#let diff_1 = \"{}\"", request.diff_1.clone().unwrap_or_default())),

        ("#let abbr_2 = \"\"", format!("#let abbr_2 = \"{}\"", request.abbr_2.clone().unwrap_or_default())),
        ("#let desc_2 = \"\"", format!("#let desc_2 = \"{}\"", request.desc_2.clone().unwrap_or_default())),
        ("#let diff_2 = \"\"", format!("#let diff_2 = \"{}\"", request.diff_2.clone().unwrap_or_default())),
    ];

    // Replace placeholders
    let mut output_content = template_clone;
    for (placeholder, replacement) in &replacements {
        output_content = output_content.replace(placeholder, replacement);
    }

    Ok(output_content)
}

async fn compile_pdf(uuid: &str) -> Result<(), AppError> {

    let uuid_dir = get_uuid_dir(uuid).await?.unwrap();

    Command::new(std::env::current_exe()?.parent().unwrap().join("typst"))
        .args([
            "compile",
            "--format",
            "pdf",
            "--root",
            uuid_dir.parent().unwrap().to_str().unwrap(),
            "--jobs",
            &format!("{}", std::thread::available_parallelism().unwrap_or(unsafe{NonZero::new_unchecked(1)})),
            uuid_dir.join("main.typ").to_str().unwrap(),
            "Hosentaschenkarte.pdf"
        ])
        .current_dir(&uuid_dir)
        .output()
        .map(|ok| {
            if ok.status.success() {
                Ok(())
            } else {
                Err(anyhow::Error::msg(format!("Die Hosentaschenkarte konnte nicht in ein PDF umgewandelt werden. Exit Code: {}.\n------ Typst StdOut -------\n{}.\n------ Typst StdErr -------\n{}\n", 
                ok.status.code().unwrap_or(-1), String::from_utf8(ok.stdout).unwrap_or("StdOut was no valid UTF-8".into()), String::from_utf8(ok.stderr).unwrap_or("StdOut was no valid UTF-8".into()))))
            }
        })??;

    Ok(())
} 

pub async fn generate_pdf_for_pocketpaper(
    kind: PocketPaperKind,
    uuid: &str,
) -> Result<(), AppError> {

    let updated_conf = match kind {
        PocketPaperKind::STL(stlrequest) => write_stl_request_to_conf(&stlrequest, String::from_utf8(HTK_STL_CONF.to_vec()).map_err(|err| { anyhow::Error::from(err) })?),
        PocketPaperKind::VLT(vltrequest) => write_vlt_request_to_conf(&vltrequest, String::from_utf8(HTK_VLT_CONF.to_vec()).map_err(|err| { anyhow::Error::from(err) })?),
        PocketPaperKind::SPI(spirequest) => write_spi_request_to_conf(&spirequest, String::from_utf8(HTK_SPI_CONF.to_vec()).map_err(|err| { anyhow::Error::from(err) })?),
    }?;

    fs::write(get_uuid_dir(uuid).await?.unwrap().join("conf.typ"), updated_conf.as_bytes())?;

    compile_pdf(uuid).await?;

    Ok(())
}