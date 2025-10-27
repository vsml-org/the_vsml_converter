use super::*;
use mockall::predicate;

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

pub struct MockImage {}
pub struct MockAudio {}

struct MockTextRenderer;

impl TextRenderer for MockTextRenderer {
    type Image = MockImage;

    fn render_text(&mut self, _text_data: &schemas::TextData) -> Self::Image {
        MockImage {}
    }
}

#[test]
fn test_render_frame_image() {
    let iv_data = schemas::IVData::<MockImage, MockAudio> {
        resolution_x: 1920,
        resolution_y: 1080,
        fps: 60,
        sampling_rate: 44100,
        object: ObjectData::Element {
            object_type: ObjectType::Wrap,
            start_time: 0.0,
            duration: 1.0,
            element_rect: ElementRect {
                alignment: Alignment::Center,
                parent_alignment: Alignment::Center,
                x: 0.0,
                y: 0.0,
                width: 1920.0,
                height: 1080.0,
            },
            attributes: Default::default(),
            styles: Default::default(),
            children: vec![],
        },
    };
    let mut mock_rc = MockRenderingContext::new();
    mock_rc.expect_create_renderer().times(1).returning(|| {
        let mut mock_renderer = MockRenderer::new();
        mock_renderer
            .expect_render()
            .with(predicate::eq(1920), predicate::eq(1080))
            .times(1)
            .returning(|_, _| MockImage {});
        mock_renderer
    });
    let mut mock_text_renderer = MockTextRenderer;
    render_frame_image(&iv_data, 0, mock_rc, &mut mock_text_renderer);
}

#[test]
fn test_mix_audio() {
    let iv_data = schemas::IVData::<MockImage, MockAudio> {
        resolution_x: 1920,
        resolution_y: 1080,
        fps: 60,
        sampling_rate: 44100,
        object: ObjectData::Element {
            object_type: ObjectType::Wrap,
            start_time: 0.0,
            duration: 1.0,
            element_rect: ElementRect {
                alignment: Alignment::Center,
                parent_alignment: Alignment::Center,
                x: 0.0,
                y: 0.0,
                width: 1920.0,
                height: 1080.0,
            },
            attributes: Default::default(),
            styles: Default::default(),
            children: vec![],
        },
    };
    let mut mock_mc = MockMixingContext::new();
    mock_mc.expect_create_mixer().times(1).returning(|_| {
        let mut mock_mixer = MockMixer::new();
        mock_mixer.expect_mix().times(1).returning(|_| MockAudio {});
        mock_mixer
    });
    mix_audio(&iv_data, mock_mc);
}
