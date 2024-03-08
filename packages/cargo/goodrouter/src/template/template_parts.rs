use regex::{CaptureMatches, Regex};

pub fn parse_template_parts<'r>(template: &'r str, re: &'r Regex) -> TemplateParts<'r> {
  TemplateParts::new(template, re)
}

pub struct TemplateParts<'r> {
  template: &'r str,
  matches: CaptureMatches<'r, 'r>,
  is_finished: bool,
  index: usize,
  part_offset: usize,
  parameter: &'r str,
}

impl<'r> TemplateParts<'r> {
  fn new(template: &'r str, re: &'r Regex) -> Self {
    let matches = re.captures_iter(template);
    let is_finished = false;
    let part_index = 0;
    let part_offset = 0;
    let parameter = "";

    Self {
      template,
      matches,
      is_finished,
      index: part_index,
      part_offset,
      parameter,
    }
  }
}

impl<'r> Iterator for TemplateParts<'r> {
  type Item = &'r str;

  fn next(&mut self) -> Option<Self::Item> {
    if self.is_finished {
      return None;
    }

    let result = if self.index % 2 == 0 {
      let part_offset = self.part_offset;

      if let Some(current_match) = self.matches.next() {
        let first_capture = current_match.get(0).unwrap();
        let current_capture = current_match.get(1).unwrap();

        self.part_offset = first_capture.end();
        self.parameter = current_capture.as_str();

        Some(&self.template[part_offset..first_capture.start()])
      } else {
        self.is_finished = true;

        Some(&self.template[part_offset..])
      }
    } else {
      Some(self.parameter)
    };

    self.index += 1;

    result
  }
}

#[cfg(test)]
mod tests {
  use super::super::TEMPLATE_PLACEHOLDER_REGEX;
  use super::*;

  #[test]
  fn parse_template_parts_test() {
    let parts: Vec<_> = parse_template_parts("/a/{b}/{c}", &TEMPLATE_PLACEHOLDER_REGEX).collect();

    assert_eq!(parts, vec!["/a/", "b", "/", "c", ""]);

    let parts: Vec<_> = parse_template_parts("/a/{b}/{c}/", &TEMPLATE_PLACEHOLDER_REGEX).collect();

    assert_eq!(parts, vec!["/a/", "b", "/", "c", "/"]);

    let parts: Vec<_> = parse_template_parts("", &TEMPLATE_PLACEHOLDER_REGEX).collect();

    assert_eq!(parts, vec![""])
  }
}
