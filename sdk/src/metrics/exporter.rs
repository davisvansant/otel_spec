#[derive(Debug, PartialEq)]
pub enum ExportKind {
    Delta,
    Cumulative,
    PassThrough,
}

pub struct ExportKindSelector {
    pub export: ExportKind,
}

impl ExportKindSelector {
    pub fn select(export_kind: ExportKind) -> ExportKindSelector {
        ExportKindSelector {
            export: export_kind,
        }
    }
}

pub struct ExportRecord {
    pub instrument: String,
    pub label_set: String,
    pub resource: String,
    pub aggregation: String,
}

pub struct ExportRecordSet {
    pub export_records: (String, String),
}

pub struct Exporter {
    pub some_collection: Vec<String>,
}

impl Exporter {
    pub fn init() -> Exporter {
        Exporter {
            some_collection: Vec::with_capacity(10),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select() {
        // let test_exporter = Exporter::init();
        // assert_eq!(test_exporter.some_collection.len(), 0);
        let test_delta = ExportKindSelector::select(ExportKind::Delta);
        let test_cumulative = ExportKindSelector::select(ExportKind::Cumulative);
        let test_passthrough = ExportKindSelector::select(ExportKind::PassThrough);
        assert_eq!(test_delta.export, ExportKind::Delta);
        assert_eq!(test_cumulative.export, ExportKind::Cumulative);
        assert_eq!(test_passthrough.export, ExportKind::PassThrough);
    }

    #[test]
    fn init() {
        let test_exporter = Exporter::init();
        assert_eq!(test_exporter.some_collection.len(), 0);
    }
}
