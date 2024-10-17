use aiscript_engine_common::Utf16Str;

#[derive(Clone, PartialEq, Eq)]
pub(crate) struct Path<'src> {
    segments: Vec<&'src Utf16Str>,
}

impl<'src> Path<'src> {
    pub(crate) fn new(name: &'src Utf16Str) -> Self {
        Path { segments: vec![name] }
    }

    pub(crate) fn resolve(&self, name: &'src Utf16Str) -> Self {
        let mut segments = self.segments.clone();
        segments.push(name);
        return Path { segments };
    }

    pub(crate) fn eq_to_ref(&self, ref_name: &Utf16Str) -> bool {
        let _ = ref_name;
        todo!()
    }
}
