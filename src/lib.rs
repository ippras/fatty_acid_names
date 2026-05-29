pub mod l10n {
    use egui_l10n::ftl;

    pub const EN: &[&str] = &[
        ftl!("en/main.ftl"),
        ftl!("en/main.selectors.ftl"),
        ftl!("en/aocs.org.ftl"),
        ftl!("en/aocs.org.ext.ftl"),
    ];

    pub const RU: &[&str] = &[
        ftl!("ru/main.ftl"),
        ftl!("en/aocs.org.ftl"),
        ftl!("en/aocs.org.ext.ftl"),
    ];
}

pub mod r#const;

#[cfg(feature = "egui")]
pub mod egui;
