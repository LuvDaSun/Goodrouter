use criterion::{black_box, criterion_group, criterion_main, Criterion};
use goodrouter::router::Router;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;

pub static TEMPLATE_PLACEHOLDER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{(.*?)\}").unwrap());

criterion_group!(
    benches,
    router_parse_bench_small,
    router_parse_bench_docker,
    router_parse_bench_github
);
criterion_main!(benches);

fn router_parse_bench_small(criterion: &mut Criterion) {
    setup_group(criterion, "small");
}

fn router_parse_bench_docker(criterion: &mut Criterion) {
    setup_group(criterion, "docker");
}

fn router_parse_bench_github(criterion: &mut Criterion) {
    setup_group(criterion, "github");
}

fn setup_group(criterion: &mut Criterion, name: &str) {
    let mut path = std::path::PathBuf::new();
    path.push("..");
    path.push("..");
    path.push("..");
    path.push("fixtures");
    path.push(name);
    path.set_extension("txt");

    let templates = std::fs::read_to_string(path.as_path()).unwrap();
    let templates: Vec<_> = templates
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let mut parameter_names: HashSet<&str> = Default::default();

    for template in templates.iter() {
        for captures in TEMPLATE_PLACEHOLDER_REGEX.captures_iter(template) {
            parameter_names.insert(captures.get(1).unwrap().as_str());
        }
    }

    let parameter_values: Vec<_> = (0..parameter_names.len())
        .map(|index| format!("p{}", index))
        .collect();

    let parameters = parameter_names
        .into_iter()
        .zip(parameter_values.iter().map(|v| v.as_str()))
        .collect();

    let template_count = templates.len();

    let mut router = Router::new();

    for template in templates.iter() {
        router.insert_route(template, template);
    }

    let paths: Vec<_> = templates
        .iter()
        .map(|template| router.stringify_route(template, &parameters).unwrap())
        .collect();

    let mut group = criterion.benchmark_group(format!("router parse {}", name));

    group.bench_function(format!("{} routes", template_count), |bencher| {
        let mut iteration = 0;
        bencher.iter(|| {
            let path = &paths[iteration % template_count];

            router.parse_route(black_box(path));

            iteration += 1;
        })
    });

    group.finish();
}
