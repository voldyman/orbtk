use backend::{FontMeasure, FONT_MEASURE};
use dces::entity::{Entity, EntityComponentManager};
use layout_object::LayoutObject;
use properties::{Constraint, Label, Offset, Rect, TextSelection};
use theme::{Selector, Theme};

use LayoutResult;

pub struct TextSelectionLayoutObject;

impl Into<Box<LayoutObject>> for TextSelectionLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

impl LayoutObject for TextSelectionLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        _children: &[Entity],
        _size: Option<(u32, u32)>,
        theme: &Theme,
    ) -> LayoutResult {
        let size = {
            if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
                let mut width = theme.uint("width", selector) as u32;
                let mut height = theme.uint("height", selector) as u32;

                if width == 0 {
                    width = constraint.width;
                }

                if height == 0 {
                    height = constraint.height;
                }

                (width, height)
            } else {
                (0, 0)
            }
        };

        let mut pos = 0;

        if let Ok(selector) = ecm.borrow_component::<Selector>(entity) {
            if let Ok(label) = ecm.borrow_component::<Label>(entity) {
                if let Ok(selection) = ecm.borrow_component::<TextSelection>(entity) {
                    if let Some(label_part) = label.0.get(0..selection.start_index) {
                        pos = FONT_MEASURE
                            .measure(
                                label_part,
                                &theme.string("font-family", selector),
                                theme.uint("font-size", selector),
                            )
                            .0 as i32;
                    }
                }
            }
        }

        if let Ok(off) = ecm.borrow_component::<Offset>(entity) {
            pos += off.0;
        }

        if let Ok(bounds) = ecm.borrow_mut_component::<Rect>(entity) {
            bounds.x = pos;
        }

        LayoutResult::Size(size)
    }
}
