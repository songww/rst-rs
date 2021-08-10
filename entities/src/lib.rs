dtd::dtd!("definitions/docutils.dtd");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_generated() {
        let document = Document {
            opt_tuple_title_opt_subtitle: None,
            opt_meta: None,
            opt_decoration: None,
            opt_tuple_docinfo_opt_transition: None,
            tuple_tuple_non_empty_document_choices_opt_transition_opt_tuple_tuple_section_tuple_opt_transition_tuple_section: TupleTupleNonEmptyDocumentChoicesOptTransitionOptTupleTupleSectionTupleOptTransitionTupleSection {
                tuple_non_empty_document_choices_opt_transition: TupleNonEmptyDocumentChoicesOptTransition {
                    non_empty_document_choices: vec![DocumentChoices::Paragraph(Paragraph)],
                    opt_transition: None,
                },
                opt_tuple_tuple_section_tuple_opt_transition_tuple_section: None,
            },
        };
        assert!(document.opt_meta.is_none());
        println!("{:?}", document);
    }
}
