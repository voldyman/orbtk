use std::cell::Cell;

use dces::entity::{Entity, EntityComponentManager};

use layout_object::{LayoutObject, LayoutResult};
use properties::{Constraint, Offset, Rect};
use theme::Theme;

#[derive(Default)]
pub struct ScrollLayoutObject {
    child_bounds: Cell<Rect>,
}

impl Into<Box<LayoutObject>> for ScrollLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

impl LayoutObject for ScrollLayoutObject {
    fn layout(
        &self,
        entity: Entity,
        ecm: &mut EntityComponentManager,
        constraint: &Constraint,
        children: &[Entity],
        size: Option<(u32, u32)>,
        _theme: &Theme,
    ) -> LayoutResult {
        if let Some(size) = size {
            let width = {
                if constraint.width > 0 {
                    constraint.width
                } else {
                    size.0
                }
            };

            let height = {
                if constraint.height > 0 {
                    constraint.height
                } else {
                    size.1
                }
            };

            let center_size = constraint.perform((width, height));

            let mut offset = (0, 0);

            let old_bounds = self.child_bounds.get();

            if let Ok(off) = ecm.borrow_component::<Offset>(entity) {
                // off.0 = (center_size.0 as i32 - size.0 as i32).min(0);
                // off.1 = (center_size.1 as i32 - size.1 as i32).min(0);

                offset = (off.0, off.1);
            }

            if let Ok(bounds) = ecm.borrow_mut_component::<Rect>(children[0]) {
                if bounds.width <= center_size.0 {
                    offset.0 = 0;
                } else {
                    let offset_width = old_bounds.width as i32 - bounds.width as i32;

                    if offset_width != 0 {
                        offset.0 = (offset.0 + offset_width).min(0);
                    }
                }

                if bounds.height <= center_size.1 {
                    offset.1 = 0;
                }

                // todo: vertical scrollint

                bounds.x = offset.0;
                bounds.y = offset.1;

                self.child_bounds.set(*bounds);
            }

            if let Ok(off) = ecm.borrow_mut_component::<Offset>(entity) {
                off.0 = offset.0;
                off.1 = offset.1;
            }

            LayoutResult::Size(center_size)
        } else {
            if children.is_empty() {
                return LayoutResult::Size((constraint.max_width, constraint.max_height));
            }
            LayoutResult::RequestChild(children[0], *constraint)
        }
    }
}
