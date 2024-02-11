use super::template_parts::{parse_template_parts, TemplateParts};
use regex::Regex;

pub fn parse_template_pairs<'r>(template: &'r str, re: &'r Regex) -> TemplatePairs<'r> {
    TemplatePairs::new(template, re)
}

pub struct TemplatePairs<'r> {
    parts: TemplateParts<'r>,
    index: usize,
    is_finished: bool,
}

impl<'r> TemplatePairs<'r> {
    fn new(template: &'r str, re: &'r Regex) -> Self {
        let parts = parse_template_parts(template, re);
        let index = 0;
        let is_finished = false;

        Self {
            parts,
            index,
            is_finished,
        }
    }
}

impl<'r> Iterator for TemplatePairs<'r> {
    type Item = (&'r str, Option<&'r str>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        let result = if let Some(part) = self.parts.next() {
            if self.index == 0 {
                Some((part, None))
            } else {
                let anchor = self.parts.next().unwrap();
                Some((anchor, Some(part)))
            }
        } else {
            None
        };

        self.index += 1;

        if result.is_none() {
            self.is_finished = true;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::TEMPLATE_PLACEHOLDER_REGEX;
    use super::*;

    #[test]
    fn parse_template_pairs_test() {
        let pairs: Vec<_> =
            parse_template_pairs("/a/{b}/{c}", &TEMPLATE_PLACEHOLDER_REGEX).collect();

        assert_eq!(
            pairs,
            vec![("/a/", None), ("/", Some("b")), ("", Some("c"))]
        );

        let pairs: Vec<_> =
            parse_template_pairs("/a/{b}/{c}/", &TEMPLATE_PLACEHOLDER_REGEX).collect();

        assert_eq!(
            pairs,
            vec![("/a/", None), ("/", Some("b")), ("/", Some("c"))]
        );

        let pairs: Vec<_> = parse_template_pairs("", &TEMPLATE_PLACEHOLDER_REGEX).collect();

        assert_eq!(pairs, vec![("", None)])
    }
}
