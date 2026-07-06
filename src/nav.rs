use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct NavItem {
    text: &'static str,
    href: &'static str,
}

#[derive(Debug, Serialize)]
pub(crate) struct NavLink {
    icon_svg: &'static str,
    href: &'static str,
    label: &'static str,
}

pub(crate) fn nav_items() -> Vec<NavItem> {
    vec![
        NavItem {
            text: "Research",
            href: "research.html",
        },
        NavItem {
            text: "Publications",
            href: "publications.html",
        },
    ]
}

pub(crate) fn nav_links() -> Vec<NavLink> {
    vec![
        NavLink {
            icon_svg: r#"<svg class="nav-icon nav-icon-fill" viewBox="0 0 24 24" aria-hidden="true"><path d="M12 .5C5.65.5.5 5.65.5 12c0 5.1 3.29 9.41 7.86 10.94.58.1.79-.25.79-.56v-2.16c-3.2.7-3.88-1.36-3.88-1.36-.52-1.34-1.28-1.69-1.28-1.69-1.05-.72.08-.7.08-.7 1.16.08 1.77 1.19 1.77 1.19 1.03 1.77 2.71 1.26 3.37.96.1-.75.4-1.26.73-1.55-2.56-.29-5.25-1.28-5.25-5.7 0-1.26.45-2.29 1.19-3.1-.12-.29-.52-1.47.11-3.05 0 0 .97-.31 3.17 1.18A11.02 11.02 0 0 1 12 6c.98 0 1.95.13 2.87.39 2.2-1.49 3.17-1.18 3.17-1.18.63 1.58.23 2.76.11 3.05.74.81 1.19 1.84 1.19 3.1 0 4.43-2.69 5.41-5.26 5.69.42.36.79 1.07.79 2.16v3.2c0 .31.21.67.8.56A11.51 11.51 0 0 0 23.5 12C23.5 5.65 18.35.5 12 .5Z"/></svg>"#,
            href: "https://github.com/cole-brokamp",
            label: "GitHub",
        },
        NavLink {
            icon_svg: r#"<svg class="nav-icon nav-icon-stroke" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 6h16v12H4z"/><path d="m4 7 8 6 8-6"/></svg>"#,
            href: "mailto:cole@colebrokamp.com",
            label: "Email",
        },
        NavLink {
            icon_svg: r#"<svg class="nav-icon nav-icon-stroke" viewBox="0 0 24 24" aria-hidden="true"><path d="M6 3h8l4 4v14H6z"/><path d="M14 3v5h5"/><path d="M9 13h6"/><path d="M9 17h6"/></svg>"#,
            href: "peds-cv-brokamp.docx",
            label: "CV",
        },
        NavLink {
            icon_svg: r#"<svg class="nav-icon nav-icon-stroke" viewBox="0 0 24 24" aria-hidden="true"><path d="m2 10 10-5 10 5-10 5z"/><path d="M6 12v5c2 1.4 4 2 6 2s4-.6 6-2v-5"/><path d="M22 10v6"/></svg>"#,
            href: "https://scholar.google.com/citations?user=N_CkwfoAAAAJ&hl=en",
            label: "Google Scholar",
        },
        NavLink {
            icon_svg: r#"<svg class="nav-icon nav-icon-stroke" viewBox="0 0 24 24" aria-hidden="true"><path d="M4 5.5A3.5 3.5 0 0 1 7.5 2H20v17H7.5A3.5 3.5 0 0 0 4 22z"/><path d="M4 5.5V22"/><path d="M8 6h8"/><path d="M8 10h8"/></svg>"#,
            href: "colebrokamp.bib",
            label: "BibTeX",
        },
    ]
}
