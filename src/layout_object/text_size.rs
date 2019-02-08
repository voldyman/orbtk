use backend::{FontMeasure, FONT_MEASURE};
use dces::entity::{Entity, EntityComponentManager};
use layout_object::LayoutObject;
use properties::Constraint;
use theme::{Selector, Theme};

use {Label, LayoutResult};

pub struct TextSizeLayoutObject;

impl Into<Box<LayoutObject>> for TextSizeLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

impl LayoutObject for TextSizeLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        _constraint: &Constraint,
        _children: &[Entity],
        _size: Option<(u32, u32)>,
        theme: &Theme,
    ) -> LayoutResult {
        if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
            if let Ok(label) = ecm.borrow_component::<Label>(entity) {
                let size = {
                    if label.0.is_empty() {
                        (0, 0)
                    } else {
                        FONT_MEASURE.measure(
                            &label.0,
                            &theme.string("font-family", selector),
                            theme.uint("font-size", selector),
                        )
                    }
                };
                return LayoutResult::Size(size);
            }
        }

        LayoutResult::Size((0, 0))
    }
}
