use backend::{FontMeasure, FONT_MEASURE};
use dces::entity::{Entity, EntityComponentManager};
use layout_object::LayoutObject;
use properties::{Constraint, FontIcon};
use theme::{Selector, Theme};

use LayoutResult;

pub struct FontIconSizeLayoutObject;

impl Into<Box<LayoutObject>> for FontIconSizeLayoutObject {
    fn into(self) -> Box<LayoutObject> {
        Box::new(self)
    }
}

impl LayoutObject for FontIconSizeLayoutObject {
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
            if let Ok(icon) = ecm.borrow_component::<FontIcon>(entity) {
                let size = {
                    if icon.0.is_empty() {
                        (0, 0)
                    } else {
                        let mut size = FONT_MEASURE.measure(
                            &icon.0,
                            &theme.string("icon-font-family", selector),
                            theme.uint("icon-size", selector),
                        );
                        if size.0 == 0 {
                            size = (0, 0);
                        }
                        size.0 = size.0 + theme.uint("icon-margin", selector);
                        size
                    }
                };

                return LayoutResult::Size(size);
            }
        }

        LayoutResult::Size((0, 0))
    }
}
