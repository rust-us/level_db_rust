use custom_proc_macro::arr;

#[derive(Debug, PartialEq)]
struct Test;

#[test]
fn test_arr() {
    let origin = [0; 16];

    let u32_arr = arr!([0_u32; 16]);
    println!("{:?}", u32_arr);
    assert_eq!(origin, u32_arr);

    let num_arr = arr!([0; 16]);
    println!("{:?}", num_arr);
    assert_eq!(origin, num_arr);

    let u32_arr: [u32; 16] = arr!([0_u32; 16]);
    println!("{:?}", u32_arr);
    assert_eq!(origin, u32_arr);

    let num_arr: [u32; 16] = arr!([0; 16]);
    println!("{:?}", num_arr);
    assert_eq!(origin, num_arr);

    let num_arr: [u64; 16] = arr!([0; 16]);
    println!("{:?}", num_arr);
    assert_eq!(origin, u32_arr);

    let test_origin = [
        Test, Test, Test, Test, Test, Test, Test, Test,
        Test, Test, Test, Test, Test, Test, Test, Test
    ];
    let test_arr = arr!([Test; 16]);
    println!("{:?}", test_arr);
    assert_eq!(test_origin, test_arr);

    let err = arr!(Test;16);
}