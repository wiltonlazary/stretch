pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch
        .new_node(
            stretch::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: stretch::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node10 = stretch
        .new_node(
            stretch::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: stretch::style::Dimension::Percent(0f32),
                size: stretch::geometry::Size { width: stretch::style::Dimension::Points(20f32), ..Default::default() },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let node1 = stretch
        .new_node(
            stretch::style::Style {
                display: stretch::style::Display::None,
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: stretch::style::Dimension::Percent(0f32),
                ..Default::default()
            },
            &[node10],
        )
        .unwrap();
    let node2 = stretch
        .new_node(
            stretch::style::Style {
                flex_grow: 1f32,
                flex_shrink: 1f32,
                flex_basis: stretch::style::Dimension::Percent(0f32),
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
            &[node0, node1, node2],
        )
        .unwrap();
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
}
