use printpdf::*;

pub struct LetterPdfService;

impl LetterPdfService {
    pub fn generate_pdf(
        org_name: &str,
        letter_number: &str,
        _subject: &str,
        _recipient: &str,
        _date_str: &str,
        _body: &str,
    ) -> Vec<u8> {
        let title = format!("Surat {} - {}", letter_number, org_name);
        let mut doc = PdfDocument::new(&title);
        let page = PdfPage::new(Mm(210.0), Mm(297.0), vec![]);
        doc.pages.push(page);
        doc.save(&PdfSaveOptions::default(), &mut Vec::new())
    }
}
