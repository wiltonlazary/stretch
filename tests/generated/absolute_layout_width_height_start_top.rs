#[test]
fn absolute_layout_width_height_start_top() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                position_type: stretch::style::PositionType::Absolute,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(10f32),
                    height: stretch::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                position: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10f32),
                    top: stretch::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).unwrap().size.width, 100f32);
    assert_eq!(stretch.layout(node).unwrap().size.height, 100f32);
    assert_eq!(stretch.layout(node).unwrap().location.x, 0f32);
    assert_eq!(stretch.layout(node).unwrap().location.y, 0f32);
    assert_eq!(stretch.layout(node0).unwrap().size.width, 10f32);
    assert_eq!(stretch.layout(node0).unwrap().size.height, 10f32);
    assert_eq!(stretch.layout(node0).unwrap().location.x, 10f32);
    assert_eq!(stretch.layout(node0).unwrap().location.y, 10f32);
}
