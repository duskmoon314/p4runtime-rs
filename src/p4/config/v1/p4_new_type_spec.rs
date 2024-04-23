#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, :: prost :: Oneof)]
pub enum Representation {
    #[doc = " if no @p4runtime_translation annotation present"]
    #[prost(message, tag = "1")]
    OriginalType(super::P4DataTypeSpec),
    #[doc = " if @p4runtime_translation annotation present"]
    #[prost(message, tag = "2")]
    TranslatedType(super::P4NewTypeTranslation),
}
