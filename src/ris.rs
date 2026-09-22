use crate::data::Publication;
use crate::text::{normalize_doi, present};

pub(crate) fn render(publications: &[Publication]) -> String {
    let mut output = String::new();
    for publication in publications {
        field(&mut output, "TY", Some("JOUR"));
        field(&mut output, "TI", Some(&publication.title));
        // Keep source names intact: splitting on the last space would corrupt
        // compound surnames, suffixes, and consortium authors.
        for author in &publication.author {
            field(&mut output, "AU", Some(author));
        }
        field(&mut output, "JO", Some(&publication.journal));
        field(&mut output, "PY", Some(&publication.year.to_string()));
        field(&mut output, "VL", publication.volume.as_deref());
        field(&mut output, "IS", publication.number.as_deref());
        if let Some(pages) = present(publication.pages.as_deref()) {
            if let Some((start, end)) = pages.split_once(['-', '–', '—']) {
                field(&mut output, "SP", Some(start));
                field(&mut output, "EP", Some(end));
            } else {
                field(&mut output, "SP", Some(pages));
            }
        }
        field(
            &mut output,
            "DO",
            normalize_doi(publication.doi.as_deref()).as_deref(),
        );
        field(&mut output, "AN", publication.pmid.as_deref());
        field(&mut output, "UR", publication.url.as_deref());
        if let Some(pmid) = present(publication.pmid.as_deref()) {
            field(&mut output, "N1", Some(&format!("PMID: {pmid}")));
        }
        field(&mut output, "N1", publication.note.as_deref());
        output.push_str("ER  - \n\n");
    }
    output
}

fn field(output: &mut String, tag: &str, value: Option<&str>) {
    if let Some(value) = present(value) {
        // Embedded newlines must not create spurious RIS tags or records.
        let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
        output.push_str(&format!("{tag}  - {value}\n"));
    }
}

#[cfg(test)]
mod tests {
    use super::render;
    use crate::data::Publication;

    #[test]
    fn preserves_metadata_and_record_boundaries() {
        let publications: Vec<Publication> = noyalib::from_str(
            r#"
- title: "A title\nER  - with accents é & punctuation"
  author: [Cole Brokamp, Sarah de Loizaga, Richard A. Falcone Jr.]
  journal: Example Journal
  year: 2026
  volume: '12'
  number: '3'
  pages: '101–109'
  doi: 'https://doi.org/10.1234/example'
  pmid: '12345678'
  url: 'https://example.org/paper'
- title: In press article
  author: [Cole Brokamp]
  journal: Another Journal
  year: 2026
  pages: e0324673
  note: In Press
  doi: ''
"#,
        )
        .unwrap();
        let output = render(&publications);
        assert_eq!(
            output.lines().filter(|line| *line == "TY  - JOUR").count(),
            2
        );
        assert_eq!(output.lines().filter(|line| *line == "ER  - ").count(), 2);
        assert!(output.contains("TI  - A title ER - with accents é & punctuation\n"));
        assert!(output.contains("AU  - Sarah de Loizaga\nAU  - Richard A. Falcone Jr.\n"));
        assert!(output.contains("SP  - 101\nEP  - 109\n"));
        assert!(output.contains("DO  - 10.1234/example\nAN  - 12345678\n"));
        assert!(output.contains("N1  - PMID: 12345678\n"));
        assert!(output.contains("SP  - e0324673\nN1  - In Press\nER  - \n"));
        assert_eq!(
            output
                .lines()
                .filter(|line| line.starts_with("DO  - "))
                .count(),
            1
        );
    }
}
