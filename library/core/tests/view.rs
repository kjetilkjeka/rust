#[test]
fn test_ref_size() {
    assert_eq!(std::mem::size_of::<&[usize[1]]>(), (1+1*2)*std::mem::size_of::<usize>());
    assert_eq!(std::mem::size_of::<&[usize[2]]>(), (1+2*2)*std::mem::size_of::<usize>());
    assert_eq!(std::mem::size_of::<&[usize[3]]>(), (1+3*2)*std::mem::size_of::<usize>());
    assert_eq!(std::mem::size_of::<&[usize[4]]>(), (1+4*2)*std::mem::size_of::<usize>());
    assert_eq!(std::mem::size_of::<&[usize[5]]>(), (1+5*2)*std::mem::size_of::<usize>());
    assert_eq!(std::mem::size_of::<&[usize[6]]>(), (1+6*2)*std::mem::size_of::<usize>());
}
