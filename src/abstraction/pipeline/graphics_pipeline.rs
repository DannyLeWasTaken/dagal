use super::*;

pub struct GraphicsPipelineBuilder {}

impl PipelineBuilder for GraphicsPipelineBuilder {}

pub struct GraphicsPipeline {}

impl Pipeline for GraphicsPipeline {
    type PipelineBuilderType = GraphicsPipelineBuilder;
    fn from_builder(builder: GraphicsPipelineBuilder) -> Self {
        todo!()
    }
}
