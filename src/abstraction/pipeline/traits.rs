pub trait Pipeline {
    type PipelineBuilderType;
    fn from_builder(builder: Self::PipelineBuilderType) -> Self;
}

pub trait PipelineBuilder {}
