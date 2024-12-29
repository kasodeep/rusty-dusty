use test_crate::Rectangle;

#[test]
fn test_rectangle_creation() {
    let rect = Rectangle::new(5, 10);
    assert_eq!(rect.area(), 50, "Area calculation failed in integration test");
}

#[test]
fn test_multiple_rectangles() {
    let rect1 = Rectangle::new(3, 4);
    let rect2 = Rectangle::new(4, 3);
    
    assert_eq!(rect1.area(), rect2.area(), "Rectangles with swapped dimensions should have same area");
    assert_eq!(rect1.area(), 12, "Area should be 12 for 3x4 rectangle");
}

#[test]
#[ignore]
fn test_large_rectangle() {
    let rect = Rectangle::new(1000000, 1000000);
    assert_eq!(rect.area(), 100000000);
}