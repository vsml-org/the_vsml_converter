use super::*;

#[test]
fn test_calc_rendering_info() {
    let base_element_rect = ElementRect {
        alignment: Default::default(),
        parent_alignment: Default::default(),
        x: 1.0,
        y: 2.0,
        width: 1.0,
        height: 2.0,
    };

    // x軸方向のテスト
    let element_rect = ElementRect {
        alignment: Alignment::Left,
        parent_alignment: Alignment::Left,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).x, 1.0);
    let element_rect = ElementRect {
        alignment: Alignment::Center,
        parent_alignment: Alignment::Left,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).x, 0.5);
    let element_rect = ElementRect {
        alignment: Alignment::Right,
        parent_alignment: Alignment::Left,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).x, 0.0);
    let element_rect = ElementRect {
        alignment: Alignment::Left,
        parent_alignment: Alignment::Center,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).x, 3.0);
    let element_rect = ElementRect {
        alignment: Alignment::Center,
        parent_alignment: Alignment::Center,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).x, 2.5);
    let element_rect = ElementRect {
        alignment: Alignment::Right,
        parent_alignment: Alignment::Center,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).x, 2.0);
    let element_rect = ElementRect {
        alignment: Alignment::Left,
        parent_alignment: Alignment::Right,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).x, 5.0);
    let element_rect = ElementRect {
        alignment: Alignment::Center,
        parent_alignment: Alignment::Right,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).x, 4.5);
    let element_rect = ElementRect {
        alignment: Alignment::Right,
        parent_alignment: Alignment::Right,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).x, 4.0);

    // y軸方向のテスト
    let element_rect = ElementRect {
        alignment: Alignment::Top,
        parent_alignment: Alignment::Top,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).y, 2.0);
    let element_rect = ElementRect {
        alignment: Alignment::Center,
        parent_alignment: Alignment::Top,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).y, 1.0);
    let element_rect = ElementRect {
        alignment: Alignment::Bottom,
        parent_alignment: Alignment::Top,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).y, 0.0);
    let element_rect = ElementRect {
        alignment: Alignment::Top,
        parent_alignment: Alignment::Center,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).y, 6.0);
    let element_rect = ElementRect {
        alignment: Alignment::Center,
        parent_alignment: Alignment::Center,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).y, 5.0);
    let element_rect = ElementRect {
        alignment: Alignment::Bottom,
        parent_alignment: Alignment::Center,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).y, 4.0);
    let element_rect = ElementRect {
        alignment: Alignment::Top,
        parent_alignment: Alignment::Bottom,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).y, 10.0);
    let element_rect = ElementRect {
        alignment: Alignment::Center,
        parent_alignment: Alignment::Bottom,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).y, 9.0);
    let element_rect = ElementRect {
        alignment: Alignment::Bottom,
        parent_alignment: Alignment::Bottom,
        ..base_element_rect
    };
    assert_eq!(element_rect.calc_rendering_info(4.0, 8.0).y, 8.0);
}

#[test]
fn test_render_frame_image() {

    render_frame_image()
}
