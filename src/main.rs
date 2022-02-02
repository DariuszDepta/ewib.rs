extern crate roxmltree;

use roxmltree::Node;
use std::fs;

const NODE_INSTYTUCJE: &str = "Instytucje";
const NODE_INSTYTUCJA: &str = "Instytucja";
const NODE_JEDNOSTKA: &str = "Jednostka";
const NODE_NAZWA_INSTYTUCJI: &str = "NazwaInstytucji";
const NODE_NAZWA_JEDNOSTKI: &str = "NazwaJednostki";
const NODE_NUMER_ROZLICZENIOWY: &str = "NumerRozliczeniowy";
const NODE_NR_INSTYTUCJI: &str = "NrInstytucji";
const NODE_NR_ROZLICZENIOWY: &str = "NrRozliczeniowy";

/// Common result type.
pub type Result<T, E = EwibError> = std::result::Result<T, E>;

/// Common error definition used in EWIB project.
#[derive(Debug, PartialEq)]
pub struct EwibError(String);

fn main() {
  let file_name = "/home/ddepta/Work/ewib.rs/data/plewiba.xml";
  let xml = fs::read_to_string(file_name).expect("loading input data failed");
  parse_input_data(&xml).expect("parsing input data failed");
}

fn parse_input_data(xml: &str) -> Result<()> {
  match roxmltree::Document::parse(xml) {
    Ok(document) => {
      let instytucje_node = document.root_element();
      if instytucje_node.tag_name().name() != NODE_INSTYTUCJE {
        return Err(err_unexpected_xml_node(NODE_INSTYTUCJE, instytucje_node.tag_name().name()));
      }
      match parse_instytucje(&instytucje_node) {
        Err(reason) => eprint!("{:?}", reason),
        _ => {}
      }
    }
    Err(reason) => eprint!("{}", reason),
  }
  Ok(())
}

/// Parses the content of `Instytucje` node.
fn parse_instytucje(node: &Node) -> Result<()> {
  for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_INSTYTUCJA) {
    let nazwa_instytucji = required_child_required_content(child_node, NODE_NAZWA_INSTYTUCJI)?;
    let _nr_instytucji = required_child_required_content(child_node, NODE_NR_INSTYTUCJI)?;
    let numery_rozliczeniowe = parse_jednostki(child_node)?;
    if !numery_rozliczeniowe.is_empty() {
      print_as_rule(&nazwa_instytucji, &numery_rozliczeniowe);
    }
  }
  Ok(())
}

/// Parses the list of nodes `Jednostka`.
fn parse_jednostki(node: &Node) -> Result<Vec<String>> {
  let mut numery_rozliczeniowe = vec![];
  for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_JEDNOSTKA) {
    let _nazwa_jednostki = required_child_required_content(child_node, NODE_NAZWA_JEDNOSTKI)?;
    numery_rozliczeniowe.append(&mut parse_numery_rozliczeniowe(child_node)?);
  }
  Ok(numery_rozliczeniowe)
}

/// Parses the list of nodes `NumerRozliczeniowy`.
fn parse_numery_rozliczeniowe(node: &Node) -> Result<Vec<String>> {
  let mut numery_rozliczeniowe = vec![];
  for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_NUMER_ROZLICZENIOWY) {
    let numer_rozliczeniowy = required_child_required_content(child_node, NODE_NR_ROZLICZENIOWY)?;
    numery_rozliczeniowe.push(numer_rozliczeniowy);
  }
  Ok(numery_rozliczeniowe)
}

/// Returns required text content of the specified node.
fn required_content(node: &Node) -> Result<String> {
  if let Some(text) = node.text() {
    Ok(text.to_owned())
  } else {
    Err(err_missing_required_content(node.tag_name().name()))
  }
}

/// Returns the required text content of the required child node.
fn required_child_required_content(node: &Node, child_name: &str) -> Result<String> {
  if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
    required_content(&child_node)
  } else {
    Err(err_missing_required_child_node(child_name))
  }
}

/// Creates an error describing unexpected tag.
fn err_unexpected_xml_node(s1: &str, s2: &str) -> EwibError {
  EwibError(format!("unexpected tag '{}' in node '{}'", s2, s1))
}

/// Creates an error describing missing content.
fn err_missing_required_content(s: &str) -> EwibError {
  EwibError(format!("missing required content in node '{}'", s))
}

/// Creates an error describing missing child node.
fn err_missing_required_child_node(s: &str) -> EwibError {
  EwibError(format!("missing required child node '{}'", s))
}

/// Prints the result as a rule.
fn print_as_rule(nazwa_instytucji: &str, numery_rozliczeniowe: &Vec<String>) {
  let template = r#"      <rule>
          <inputEntry>
              <text>#NR#</text>
          </inputEntry>
          <outputEntry>
              <text>#NI#</text>
          </outputEntry>
      </rule>"#;
  let ni = format!("\"{}\"", nazwa_instytucji);
  let nr = numery_rozliczeniowe.iter().map(|n| format!("\"{}\"", n)).collect::<Vec<String>>().join(",");
  println!("{}", template.replace("#NI#", &ni).replace("#NR#", &nr))
}
